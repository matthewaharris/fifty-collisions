# Idea 24 — Always For Sale
> Seed: 24. [bold] **Chess × economics × roguelite.** Pieces cost upkeep, boards are markets, and every run is an economy.

### Turn 1 — Visionary
**Working name:** Always For Sale

**Hook:** Chess under a Harberger tax. Every piece carries a price you set yourself, you pay tax on that price every turn, and your opponent can buy any piece it attacks at that price.

**Why the domains collide.** Chess already runs a hidden economy (material, tempo, "compensation"), but its prices are frozen by convention (knight = 3) while real value is positional; AlphaZero's piece values even shift between variants (Tomašev et al., 2020). Plain upkeep, the obvious reading, just taxes material. Posner and Weyl's Harberger tax (*Radical Markets*, 2018) does something stranger. It forces you to *state* your evaluation: price low and invite a buyout, price high and bleed. Every price is a tell.

**Core loop. The verb is price.**
1. **Move or reprice.** Each turn, make a chess move or reprice up to two pieces. Tax is a fixed share of your posted prices, paid from the treasury.
2. **Buy or capture.** Capture as usual, or pay an *attacked* piece's posted price to its owner and it changes colour where it stands, so a guard pawn is no defence; only price is. A fork becomes a liquidity crisis: two pieces under attack, and only enough cash to raise one price. Kings are never for sale.
3. **Go broke.** Miss a tax payment and your cheapest piece is auctioned. Bankruptcy loses like mate.
4. **Run.** Eight boards, treasury and debt carried over. Shops sell fairy pieces and relics ("limited liability", "tax holiday"). Each boss is an economic regime: a monopolist who never sells, an inflation boss whose prices rise each turn, a tax collector who doubles the rate.

**Why now.** Slay the Spire 2 peaked at 574K concurrent players (brief §4). Freestyle chess proved audiences want chess without memorized theory; no opening theory survives prices.

### Turn 2 — Builder
**What exists.** Chess roguelites are a proven shelf: Shotgun King, Pawnbarian, and Zach Gage's Really Bad Chess with its random armies. Crazyhouse and bughouse already let captured material change sides. The Harberger precedent is art: Simon de la Rouviere's *This Artwork Is Always On Sale* (2019), a self-assessed, taxed NFT. The deep ancestor is Lizzie Magie's *Landlord's Game* (1904), built to teach Georgist land tax. Nobody has put self-assessed prices on chess pieces.

**Hardest problem: the action space explodes for both sides.** Sixteen pieces with continuous prices make spreadsheet chess. Humans stall, and the AI can't search it, because every turn branches over moves × price vectors. Off-the-shelf engines have no idea what a price is.

**Twist: price is flip value.** A buyout flips the piece *where it stands*, so you lose it and your opponent gains it inside your camp. Its true value is the evaluation swing of that flip, not its material. A pawn sheltering your king flips into an enemy pawn one step from promotion. A variant engine computes each flip swing in one evaluation per piece. Pricing becomes evaluation, not search, so the AI is tractable. It also gives bosses a clean knob: each prices at flip value times a personality. The monopolist uses ×3, the gambler ×0.6, and the inflation boss drifts upward. Mastery is spotting where flip value parts from material convention. "Every price is a tell" becomes literal: an overpriced bishop says that's where your plan lives.

**MVP.** Play on a 6×6 board (as in Los Alamos chess) with small armies, integer prices from 1 to 12, and two reprices a turn. Use Fairy-Stockfish via pyffish with a flip-swing wrapper. Build three boards, three bosses and a Steam Playtest.
**Kill metric:** within five runs, players price by a rote rule, or median turn time passes 45 seconds.

### Turn 3 — Visionary
Flip value taken. It makes the AI tractable, and it shows the real structure: flip value *moves with the position*, but a price moves only when you spend a turn on it. So every price on the board is a standing quote that can go stale.

**Unseeded domain: market microstructure.** Market makers post quotes; informed traders pick off stale ones when news arrives. That's adverse selection (Glosten–Milgrom). Budish, Cramton and Shim (2015) showed that the race to snipe stale quotes is the core cost of continuous markets. Map it across:
- **Moves are news.** A discovered attack is news that reprices your pieces' flip values.
- **Tempo is latency.** "Move *or* reprice" is the market maker's dilemma: update your quotes or pursue your plan. Gaining a tempo means seeing the news before the opponent can reprice.
- **Sniping is the core tactic.** The new combination isn't a fork. It's a quiet move that makes three of the opponent's prices stale at once.

That also solves the human side of the action-space problem. Players don't price sixteen pieces. They ask one question each turn: which of my quotes did that move just make stale?

**Second-order effect: chess acquires bluffing.** A bought piece keeps its old price, and the buyer inherits the tax. That makes a cheap piece ambiguous: it's either a blunder or a poisoned pawn, a lemon that, once flipped, is pinned, bleeds tax and gets recaptured. Akerlof arrives: when lemons are common, nobody buys cheap pieces, so cheap prices become safe again. A meta-cycle of trust and bluff emerges, which chess has never had.

**Bosses from microstructure.** A *Batch* boss clears all buyouts simultaneously at the end of each round, which kills sniping. A *Market Maker* reprices four pieces a turn but moves only every other turn.

### Turn 4 — Builder
**Kills and fixes.**
- *Lemons against the AI.* A flip-value engine sees through every poisoned pawn: a lemon's flip value is negative to the buyer. Bosses must search shallowly, by personality, to be lurable. The real Akerlof cycle needs humans: async PvP after launch.
- *No income.* Tax with no inflow ends every board in mutual bankruptcy. Add Georgist rent: each occupied centre square pays per turn. Land now has a price too, and Lizzie Magie gets her lesson back.
- *Degenerate turtling.* Price everything at 12 and wait for the opponent to go broke. Rent punishes passivity; tax rising past turn 30 ends stalls.
- *Engine fit.* Fairy-Stockfish can't search buyouts. Write our own rules core; Fairy-Stockfish only scores FENs. The AI runs alpha-beta over moves, treats buys greedily (price below flip swing → buy), and reprices its stalest quotes.

**Business.** A $15–20 premium Steam game with a Next Fest demo, daily seeded runs and a classroom licence. No tokens, no NFTs, no paid power: pay-to-win in an economy game reads as satire.

**Stack.** Godot 4, a Rust rules core, Fairy-Stockfish (native and WASM), and telemetry logging posted price against flip swing, which is both the tell and a design dial.

**Failure modes.** Snowballing (buy, earn rent, buy more): progressive tax on treasury size. Edge cases (buying a pinned piece, a buy that exposes your king) follow chess legality strictly. Engine cheating on daily leaderboards: server-side replay validation.

**90 days.** Weeks 1–3: rules core and self-play sweeps over tax and rent to find the band where buyouts happen every 6–10 turns. Weeks 4–8: three bosses (Honest, Monopolist, Batch), ten relics and an optional flip-value overlay. Weeks 9–12: a Steam Playtest with 500 players.
**Kill metrics:** self-play finds no band where buying and pricing both matter, or players reprice on under 10% of turns.

### Turn 5 — Visionary
All kills accepted. Georgist rent is the missing half: tax drains, rent feeds, and the centre becomes literal real estate. One refusal: the live flip-value overlay. Shown live, it answers the question the game asks. Show it after the run as receipts ("your knight sat stale for 7 turns, worth 4 pawns to a sniper") and in tutorials, never in ranked play.

**Final form.** Always For Sale is a chess roguelite where every piece is a standing quote.
- **Price.** Posted prices, taxed every turn. Any attacked piece can be bought at its price and flips where it stands, and the buyer inherits the price and the tax.
- **Rent.** Occupied centre squares pay income. A progressive tax on treasury size stops snowballing, and a tax that rises late ends stalls.
- **Tempo is latency.** Move *or* reprice. Quiet moves that make enemy quotes stale are the signature tactic.
- **Run.** Eight boards; bosses as market regimes (Honest, Monopolist, Batch, Market Maker); relics; daily seeded runs.
- **Receipts.** After each run, posted price against flip swing, quote by quote.

**Moonshot (2036).** Async PvP brings the real Akerlof cycle: across seasons, the ladder swings between trusting cheap pieces and fearing lemons. Millions of posted prices on identical positions give chess its first *market* valuation, to set beside the engine's. The game also becomes the largest human-subject experiment on self-assessed taxation. Posner–Weyl's COST and Weyl and Zhang's depreciating spectrum licences are debated almost entirely in theory; here millions of players show whether people price honestly, panic-sell or bluff.

**Scores.** **Innovation 6.** Chess roguelites exist, but Harberger pieces, tempo-as-latency and bluffing in a perfect-information game are an unusual mechanic. **Feasibility 6.** It's an indie-scale build on proven tools. The risk is design, not technology: self-play may find no band where pricing and buying both matter.

### Turn 6 — Builder
Moving the overlay to receipts is right: the game is the question of what a piece is worth here, and a live answer kills it. I'm marking innovation down to 5. Harberger pieces with flip-value pricing and stale-quote sniping are a genuinely new mechanic, but they sit inside an established genre, level with Hand Limit and Seedless rather than above them. Feasibility 6 stands. Godot, a rules core and Fairy-Stockfish are a small team's quarter, and the only thing that can kill it is the self-play sweep finding no tax-and-rent band where pricing matters.
