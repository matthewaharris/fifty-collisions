use crate::bot::{self, BotCfg};
use crate::rules::*;
use crate::sim;
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

fn empty(params: Params) -> State {
    let mut st = State::new(params);
    st.board = [None; SQUARES];
    st
}

fn put(st: &mut State, s: &str, color: Color, kind: Kind, price: u8) {
    let b = s.as_bytes();
    let idx = sq((b[0] - b'a') as usize, (b[1] - b'1') as usize);
    st.board[idx as usize] = Some(Piece { color, kind, price, bought_ply: -10 });
}

fn at(s: &str) -> u8 {
    let b = s.as_bytes();
    sq((b[0] - b'a') as usize, (b[1] - b'1') as usize)
}

#[test]
fn opening_has_ten_moves() {
    // Six single pawn pushes plus two squares for each knight.
    assert_eq!(State::new(Params::default()).legal_moves().len(), 10);
}

#[test]
fn buy_flips_in_place_and_pays_the_owner() {
    let mut st = empty(Params::default());
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "f6", Color::Black, Kind::King, 0);
    put(&mut st, "c3", Color::White, Kind::Rook, 4);
    put(&mut st, "c5", Color::Black, Kind::Knight, 3);
    let buy = Action::Buy { sq: at("c5") };
    assert!(st.is_legal(&buy));
    let (w0, b0) = (st.treasury[0], st.treasury[1]);
    st.apply(&buy);
    let p = st.at(at("c5")).unwrap();
    assert_eq!((p.color, p.kind, p.price), (Color::White, Kind::Knight, 3));
    assert_eq!(st.treasury[0], w0 - 3.0);
    assert_eq!(st.treasury[1], b0 + 3.0);
}

#[test]
fn kings_unattacked_and_unaffordable_pieces_cannot_be_bought() {
    let mut st = empty(Params { start_treasury: 5.0, ..Params::default() });
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "a6", Color::Black, Kind::King, 0);
    put(&mut st, "a3", Color::White, Kind::Rook, 4);
    put(&mut st, "f3", Color::Black, Kind::Queen, 9); // attacked along rank 3, but costs 9 > 5
    put(&mut st, "e6", Color::Black, Kind::Knight, 2); // not attacked
    assert!(st.legal_buys().is_empty());
    assert!(!st.is_legal(&Action::Buy { sq: at("a6") }));
}

#[test]
fn a_just_bought_piece_cannot_be_bought_back_next_ply() {
    let mut st = empty(Params::default());
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "f6", Color::Black, Kind::King, 0);
    put(&mut st, "c3", Color::White, Kind::Rook, 4);
    put(&mut st, "c5", Color::Black, Kind::Rook, 3);
    st.apply(&Action::Buy { sq: at("c5") }); // white buys the c5 rook, which still faces c3
    assert!(!st.is_legal(&Action::Buy { sq: at("c3") }) || st.at(at("c3")).unwrap().color == Color::White);
    assert!(!st.legal_buys().contains(&Action::Buy { sq: at("c5") }));
}

#[test]
fn buying_the_checking_piece_answers_check() {
    let mut st = empty(Params::default());
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "f6", Color::Black, Kind::King, 0);
    put(&mut st, "a5", Color::Black, Kind::Rook, 2); // checks a1 down the file
    put(&mut st, "b6", Color::White, Kind::Rook, 5); // attacks b-file and rank 6; a6 but not a5
    put(&mut st, "c3", Color::White, Kind::Queen, 9); // attacks a5 on the diagonal
    assert!(st.in_check(Color::White));
    assert!(st.is_legal(&Action::Buy { sq: at("a5") }));
    // A reprice never answers check.
    assert!(!st.is_legal(&Action::Reprice { n: 1, sqs: [at("b6"), 0], prices: [4, 0] }));
}

#[test]
fn a_buy_that_leaves_your_king_in_check_is_illegal() {
    let mut st = empty(Params::default());
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "f6", Color::Black, Kind::King, 0);
    put(&mut st, "a6", Color::Black, Kind::Rook, 5); // checks a1
    put(&mut st, "c2", Color::White, Kind::Knight, 3);
    put(&mut st, "e3", Color::Black, Kind::Pawn, 1); // knight on c2 attacks e3, buying it ignores the check
    assert!(!st.is_legal(&Action::Buy { sq: at("e3") }));
}

#[test]
fn promotion_resets_price_to_rote_value() {
    let mut st = empty(Params::default());
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "f1", Color::Black, Kind::King, 0);
    put(&mut st, "c5", Color::White, Kind::Pawn, 2);
    st.apply(&Action::Move { from: at("c5"), to: at("c6"), promo: Some(Kind::Queen) });
    assert_eq!(st.at(at("c6")).unwrap().price, 9);
}

#[test]
fn settlement_taxes_prices_pays_rent_and_bankrupts() {
    let params = Params { tax_rate: 0.5, rent: 1.0, start_treasury: 3.0, prog_rate: 0.0, ..Params::default() };
    let mut st = empty(params);
    put(&mut st, "a1", Color::White, Kind::King, 0);
    put(&mut st, "f6", Color::Black, Kind::King, 0);
    put(&mut st, "c3", Color::White, Kind::Rook, 10); // centre: rent 1, tax 5 -> net -4
    put(&mut st, "a6", Color::Black, Kind::Pawn, 1);
    put(&mut st, "f2", Color::Black, Kind::Knight, 1);
    assert_eq!(st.settlement(Color::White), -4.0);
    st.apply(&Action::Move { from: at("c3"), to: at("c2"), promo: None }); // off centre: -5
    st.apply(&Action::Move { from: at("f6"), to: at("e6"), promo: None });
    assert_eq!(st.outcome.unwrap(), Outcome { winner: Some(Color::Black), reason: Reason::Bankruptcy });
}

#[test]
fn reprice_rules() {
    let st = State::new(Params { reprice_limit: 1, ..Params::default() });
    let two = Action::Reprice { n: 2, sqs: [at("a1"), at("b1")], prices: [6, 4] };
    assert!(!st.is_legal(&two), "over the reprice limit");
    assert!(st.is_legal(&Action::Reprice { n: 1, sqs: [at("a1"), 0], prices: [6, 0] }));
    assert!(!st.is_legal(&Action::Reprice { n: 1, sqs: [at("d1"), 0], prices: [6, 0] }), "kings have no price");
    assert!(!st.is_legal(&Action::Reprice { n: 1, sqs: [at("a1"), 0], prices: [13, 0] }), "price cap");
}

#[test]
fn random_games_never_panic_and_keep_invariants() {
    let mut rng = ChaCha8Rng::seed_from_u64(7);
    for g in 0..300 {
        let mut st = State::new(Params { max_plies: 120, ..Params::default() });
        while st.outcome.is_none() {
            let mut acts = st.legal_moves();
            acts.extend(st.legal_buys());
            if acts.is_empty() || rng.next_u32() % 10 == 0 {
                let me = st.side;
                if let Some(s) = (0..SQUARES as u8)
                    .find(|&s| matches!(st.at(s), Some(p) if p.color == me && p.kind != Kind::King))
                {
                    let p = st.at(s).unwrap().price;
                    let r = Action::Reprice { n: 1, sqs: [s, 0], prices: [if p < 12 { p + 1 } else { 1 }, 0] };
                    if st.is_legal(&r) {
                        acts.push(r);
                    }
                }
            }
            assert!(!acts.is_empty(), "game {g}: live game with no action");
            let a = acts[(rng.next_u32() as usize) % acts.len()];
            assert!(st.is_legal(&a));
            let mover = st.side;
            st.apply(&a);
            assert!(!st.in_check(mover), "game {g}: mover left in check");
            assert_eq!(st.king_sq(Color::White).is_some() && st.king_sq(Color::Black).is_some(), true);
        }
    }
}

#[test]
fn bot_games_are_reproducible() {
    let p = Params { max_plies: 60, ..Params::default() };
    let a = sim::play(p, &BotCfg::full(100.0), &BotCfg::rote(100.0), 42, false);
    let b = sim::play(p, &BotCfg::full(100.0), &BotCfg::rote(100.0), 42, false);
    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
}

#[test]
fn honest_setup_prices_stay_in_range() {
    let mut st = State::new(Params::default());
    bot::post_setup_prices(&mut st, Color::White, &BotCfg::full(150.0));
    for p in st.board.iter().flatten().filter(|p| p.color == Color::White && p.kind != Kind::King) {
        assert!((MIN_PRICE..=MAX_PRICE).contains(&p.price));
    }
}
