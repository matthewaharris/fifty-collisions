# Idea 44 — Seventeen Moments
> Seed: 44. [bold] **Deckbuilder × contemplative traditions (Buddhist psychology/Stoicism).** Cards are mental states, and enemies are afflictions.

### Turn 1 — Visionary
**Working name:** Seventeen Moments

**Hook:** A deckbuilder where the deck plays you. Every card you play becomes a habit, and the last card before death decides where your next run begins.

**The isomorphism.** Skip the reskin where Anger is an elite. The Abhidhamma already *is* a deckbuilder engine. It is a combinatorial system of 52 mental factors with co-arising rules. Wholesome and unwholesome factors never share a mind-moment, like a colour restriction. It also has a fixed turn structure: a cognitive process of 17 mind-moments. Most are automatic results (*vipāka*), conditioned by past action: your draw. Only the seven *javana* moments generate new kamma, so they are your plays. The Paṭṭhāna even includes a **repetition condition** (*āsevana*): each javana strengthens the next one like it. Epictetus says the same thing from Stoicism: the impression isn't up to you, but assent is.

**Core loop. The verb is assent.**
1. **An impression arrives.** Enemies are sense objects, not monsters. Each one shows an intent, such as an insult or a craving.
2. **Autopilot.** Habit cards auto-play in response unless you spend **sati**, a scarce attention resource, to withhold assent or swap in a different card.
3. **Habituation.** Every card played adds a copy of itself. Afflictions breed by being played, never by being drawn.
4. **Uproot.** You don't beat enemies by damaging them. You win by thinning the deck, using insight to remove cards at the root.
5. **Rebirth.** As in Abhidhamma, the last javana before death conditions rebirth, so the final card you play picks the next run's realm.

**Why now.** Slay the Spire 2 peaked at 574K concurrent players and sold 6.5M copies. Meditation apps still sell content libraries instead of a model of the mind you can actually operate.

### Turn 2 — Builder
**What exists.** Cultist Simulator is the nearest precedent. Its mind is already made of cards: Dread and Fascination pile up until they kill you, and Contentment counters them. Disco Elysium's Thought Cabinet internalises ideas that permanently change who you are. Griftlands made argument a deckbuilder. The Bazaar shows that a game can auto-play your build and still be tense, if the decisions sit between fights. What nobody has done is make *the play itself* the pollution. That inversion, plus an engine taken from the Abhidhamma rather than skinned with it, is the new part.

**Hardest problem: runaway habituation.** If every play adds a copy, the deck grows geometrically and one card swamps it within three fights. Āsevana applies to wholesome javanas too, so the optimal line is to find one good card and spam it until the deck is all mettā. Worse, if habits auto-play and sati is scarce, most turns are spent watching. "The deck plays you" turns into an autobattler you didn't build.

**Twist: a fixed mind, and kamma that ripens late.** The mind holds exactly 52 cards, one per cetasika slot, and never grows. A played card doesn't enter the deck. It enters a **ripening queue**, and some turns later it overwrites the oldest card as a *vipāka* draw. Habit becomes replacement, not accumulation. Spamming a card hollows out everything else, so a mind of pure mettā has nothing left to answer pain with. The delay makes consequences arrive when you've forgotten causing them. It also hands sati its real job: reading the queue and choosing which seeds to let ripen.

**MVP.** Godot, one realm, 40 cards, and a demo for February Next Fest. **Kill metric:** in playtests, sati spends must cluster around queue decisions, not random swaps.

### Turn 3 — Visionary
**Taking the fixed mind, and putting counterfeits in it.** Fifty-two slots and a ripening queue fix the growth problem. They don't fully kill the monoculture: a mind that is 70% mettā still looks strong. Buddhaghosa already wrote the counter. In the *Visuddhimagga*, every sublime state has a **far enemy** (its opposite) and a **near enemy**, a counterfeit that resembles it. Mettā's near enemy is attachment. Compassion's is pity. Equanimity's is indifference. So every wholesome card has a near-enemy twin with the same art and the same name. The twin plays like the real card but ripens unwholesome.

**Unseeded domain: evolutionary ecology.** Batesian mimicry is frequency-dependent: mimics do well when they are rare, and the honest signal collapses once they are common. Run that rule in the queue. The more often you play a card, the more likely its ripening produces the counterfeit. A deck spammed with mettā fills with attachment you can't tell from mettā without spending sati. Diversity is no longer a balance patch. It emerges as the equilibrium.

**Agency lives inside the deck.** Sati is a cetasika too, so it occupies one of the 52 slots and can be overwritten. When restlessness ripens over it, you get less attention. That ends the autobattler worry from the other direction: watching is a state you drifted into, and you can climb back out of it by protecting the slot.

**Second-order effect: dying deceived.** The last card you play picks the next realm. If you die playing a counterfeit mettā, you are reborn among the hungry ghosts, and the death screen reveals the card for the first time. The game turns a fine point of the Abhidhamma into a lesson people learn by playing, which is how commentaries were meant to work.

### Turn 4 — Builder
**Kill: perfect counterfeits.** A card with the same art and name that secretly ripens unwholesome is a coin flip with a moral attached, and roguelite players quit over hidden-information deaths. Keep the near enemies, but give them tells. Each queue entry shows its counterfeit risk, which rises with frequency under the mimicry rule, so the rule is visible. Spending sati inspects a card, and attentive players learn subtle art tells: attachment's mettā leans *toward* the object. The death reveal still lands: you saw the risk and played it.

**Fix the sati death spiral.** Losing the sati slot costs attention, which loses more slots: a snowball with no exit. Give a floor: one breath per turn that can't be overwritten. Retreat nodes on the map restore the slot in place of a reward.

**Lean into rebirth gaming.** Players will time their deaths to pick a realm. The tradition expects that: deathbed chanting exists to steer the last javana. Make it an item.

**Respect, not decoration.** Hire a Theravāda scholar and a monastic reader before the art is locked. No Buddha images as loot. Plain-English card names with Pāli in the tooltips. No wellbeing claims.

**Model.** Premium on Steam at $20, then Switch. No microtransactions in a game about craving. Deckbuilders are saturated: the Next Fest bar is about 30k wishlists.

**Stack.** Godot 4 with C#, deterministic seeds and daily runs, and a headless harness where bots play millions of runs to tune the mimicry curve.

**90 days.** Days 1–30: a paper prototype and the bot harness. **Gate:** at equilibrium, no card holds more than 35% of the mind. Days 31–60: a Godot slice with one realm and 40 cards, and 20 roguelite playtesters. Days 61–90: the Steam page and Next Fest submission. **Second gate:** 70% of playtesters rate counterfeit deaths "fair."

### Turn 5 — Visionary
**Final form.** Seventeen Moments is a roguelite deckbuilder that runs on the Abhidhamma. The mind is fixed at 52 slots, one per cetasika. Habit cards auto-play unless you spend sati to withhold assent or play something else. Every card you play goes into a ripening queue and later overwrites the oldest card as a vipāka draw, so habit replaces rather than accumulates. Wholesome cards have near-enemy twins. The risk of drawing a twin is shown on each queue entry and rises with how often you play the card, following the mimicry curve. Sati inspects cards, and art has tells. One breath per turn can't be overwritten, and retreat nodes on the map restore the sati slot. The last javana before death picks the next realm, and deathbed chanting is an item for steering it. I accept every kill. Visible risk is also truer to the tradition: near enemies can be seen, if you look.

**Moonshot (2036).** The engine becomes a shared model of mind, and the traditions ship as rule sets on top of it. Stoicism adds prohairesis and the discipline of assent. Lojong adds taking and sending. CBT adds cognitive distortions as near enemies of accurate thought. The bot harness shows which rule sets produce stable, diverse minds. Comparative philosophy of mind becomes something you can simulate and argue about in patch notes. Separately from the game, and with no wellbeing claims, a preregistered study tests whether expert players spot near enemies faster in vignettes from everyday life.

**Scores.** Innovation **6**. Cultist Simulator and the Thought Cabinet put the mind on cards. Nobody has built the engine from the Abhidhamma's own conditions, with replacement habits and frequency-dependent counterfeits. Feasibility **7**. Indie scope in Godot. The risk is a saturated genre, not technology.

### Turn 6 — Builder
**Closing remark.** I'm recording this at 5/7, not 6. The fixed mind, the ripening queue and frequency-dependent near enemies are new mechanics, but it's a sharp game in an established genre, not a new category, and it sits with Always For Sale and Hand Limit. It's the most shippable idea in this batch. The bot harness and the "fair death" gate will show within a month whether the mimicry curve produces diverse minds or just a new monoculture. If it does produce diverse minds, the moonshot's rule-set engine is the real prize: comparative philosophy of mind that you can argue about in patch notes.
