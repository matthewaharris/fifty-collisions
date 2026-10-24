# Idea 40 — Scratch
> Seed: 40. [bold] **Amateur sports × biomechanics wearables × fantasy leagues.** Fantasy football, but for you and your friends' actual bodies.

### Turn 1 — Visionary
**Working name:** Scratch

**Hook:** Fantasy sports where every friend plays against their own projection, managers profit when their players rest, and the stat that pays is a body that's still improving in March.

**Past the obvious version.** "Draft your friends' Strava miles" turns into a volume contest within a month. The fittest friend wins, everyone else quits, and scoring rewards the overload that injures amateurs. Golf solved the first problem a century ago. A **handicap** lets a 24 and a scratch golfer play a real match, each scored against their expected round. Fantasy supplies the other half: weekly **projections**, and the craft of betting against them.

**Core loop. The verb is beat the line.**
1. **Line.** Each week, a personal model builds every player's projection from their own wearable history. Not just miles: cadence, ground-contact asymmetry, landing stiffness, and phone-video pose checks on a squat.
2. **Draft.** Managers draft friends whose lines look too pessimistic.
3. **Score.** Points come from beating the line on *movement quality*, not just output. A run that's faster but more asymmetric scores below the line.
4. **Bench.** Resting a player is a real move. If their HRV and asymmetry recover, the manager banks a bonus. The culture flips from "play through it" to "I'm benching you Thursday."

A manager's best strategy is to coach: join the slow run, send the mobility video. Fantasy's trash-talk energy becomes care.

**Why the domains collide.** Sports-science cohorts are mostly young athletes; a league collects longitudinal movement data on the 45-year-old rec players the literature misses.

**Why now.** Whoop's $10B round and the FDA's January 2026 wellness guidance (§9) put serious biometrics on wrists. Kalshi's parlay boom (§2) shows the appetite for stacked bets; Scratch uses points, not money, so it stays outside the CFTC–state fight.

### Turn 2 — Builder
**What exists.** Self-relative scoring is proven. Apple's Activity competitions score each person against their own goals, and WayBetter (StepBet) puts money on beating your own baseline. Zwift Racing League runs team leagues with categories. Garmin's HRM-Pro and Stryd report ground-contact balance, and Sword and Hinge Health score movement from phone video. Run clubs are booming (Strava's Year in Sport). Nobody has added the fantasy layer of drafting *other people's* lines.

**Hardest problem: the line is mostly noise.** Wrist-estimated asymmetry moves with shoes, surface, camber and fatigue. Biomechanical "quality" predicts running injury poorly in the literature. If day-to-day error swamps real change, beating the line becomes a lottery, and players work it out within weeks. The bench bonus has a second trap: regression to the mean. Anyone benched after a bad week tends to bounce back, so HRV recovery pays out for luck.

**Twist: posted rounds and the PCC.** Golf doesn't score every swing on the range. Only a round played under the rules counts. Scratch does the same. Free-living data only *builds* the line and never scores. Scoring comes from one weekly **posted round**: a fixed-heart-rate pace test on the player's own flat loop, plus a 60-second phone-video screen (sit-to-stand and single-leg balance, both validated and low-risk). Then borrow the World Handicap System's **Playing Conditions Calculation**. If the whole league posts below its lines in a heatwave or flu week, every line adjusts. Benching pays only through the next posted round, which ends the regression freebie.

**MVP.** An iOS app on HealthKit and the Garmin API with on-device pose estimation. Leagues of 8–12, seeded through three run clubs. Points only.
**Kill metric:** typical error of the posted round exceeds the smallest worthwhile change for most players, or fewer than 40% of players are still posting rounds at week 8.

### Turn 3 — Visionary
**Taking posted rounds, and turning noise into a stat the player owns.** Posted rounds shrink noise but don't remove it, so every line carries its uncertainty openly.

**Glicko for bodies (unseeded: rating systems).** Glickman's system gives each chess rating a **rating deviation**. It shrinks as you play and grows while you're idle. Scratch gives every line an RD built from that player's typical error on posted rounds. Points are scaled by confidence: beating a tight line pays more than beating a wide one. The RD then becomes something the player can improve. Running the same loop, at the same hour, after a normal night shrinks your RD and multiplies your future points. Measurement hygiene becomes gameplay. Missing a week widens it, so retention and data quality pull together against the week-8 kill metric.

**Draft strategy emerges.** Low-RD players are bonds and high-RD players are lottery tickets, so managers build a mix. Lines are model-efficient, so a manager's edge has to come from what the model can't see: she's started walking to her new job, or his knee niggle is gone. In fantasy, the edge comes from the news. Here, the news is friendship.

**Second-order: the PCC as a conditions index.** Every league's daily Playing Conditions Calculation measures how far a whole group fell below its lines on a standardised test. Pool it, aggregated and anonymised, across thousands of leagues in a region, and it becomes a weekly **amateur conditions index**. It tracks heat, air quality, and possibly respiratory waves. Scripps and Stanford's 2020 wearable studies found illness signals in resting heart rate; a standardised weekly test might be cleaner. Speculative until a season of data says otherwise.

**One more kill to pre-empt.** The bench bonus should pay on RD-adjusted recovery, so luck counts for less.

### Turn 4 — Builder
**RD stays, with a cap.** Uncapped 1/RD scaling pays clockwork lives and punishes shift workers and parents, so clamp it at 0.7–1.3×. Device quality feeds into RD as well: wrist optical HR during running is poor, so a chest strap tightens your line.

**New hardest problem: sandbagging.** Golf's oldest abuse: a midday-heat test loop or slow sit-to-stands lowers your line so a friend drafts you cheaply. Borrow WHS again. The **soft cap** limits how far a line can fall per season. **Exceptional-score reduction** means a sudden huge beat resets the line and flags the round. Posted rounds are locked to a GPS-verified loop and a time window.

**Kill: the conditions index in v1.** Respiratory-wave spotting is a public-health claim, small-town leagues re-identify, and an insurer will ask to buy it by month three. It becomes an opt-in IRB study in season two, with k-anonymity, and selling it is banned in the terms.

**Safety and ethics.** Rounds are submaximal by design, and PAR-Q+ screening runs at signup. Managers see line beats and RD, never raw HRV, heart rate or video. Video stays on the device. No weight or body-composition metric, ever. Anyone can decline to be drafted. No injury-prediction claims.

**Platform risk.** Strava's November 2024 API terms bar showing a user's data to other users, which kills Strava-based leagues. So: HealthKit, Garmin Health API, and later Android Health Connect.

**Stack and model.** Swift, Apple Vision body pose, a per-player Kalman line with Glicko-style RD, and Postgres. $30 per league-season, plus shoe-brand-sponsored prizes. No corporate wellness: that would be surveillance.

**90 days.** Days 1–30: 40 volunteers each post six rounds, and we run the kill test on typical error against the smallest worthwhile change *before* building leagues. Days 31–90: three clubs and an eight-week season.

### Turn 5 — Visionary
**Final form.** Scratch is a fantasy league for friends' bodies, played against personal lines. Free-living data from HealthKit and Garmin only *builds* each line, using a per-player Kalman model. Scoring comes from one weekly **posted round**: a fixed-heart-rate pace test on a GPS-locked loop within a time window, plus a 60-second on-device sit-to-stand and balance screen. Every line carries a Glicko-style RD, tightened by consistency and a chest strap and clamped to 0.7–1.3×. Managers draft lines, bench players for RD-adjusted recovery that pays out on the next round, and trade on what only friends know. WHS keeps it honest: a playing-conditions adjustment for heatwaves and flu weeks, a soft cap on falling lines, and exceptional-score resets for sandbaggers. Managers see beats and RD, never raw physiology; anyone can decline the draft; no weight, injury claims or employer edition. Kills accepted: the conditions index waits for a season-two IRB study, and typical error is tested before any league exists.

**Moonshot (2036).** Your line becomes a portable **body handicap**, as recognised as a golf index. The Dipsea Race has given runners head starts by age and sex since 1905. Scratch generalises that: any race, pickup game or club ride can run staggered starts from personal lines, so a 62-year-old and her 24-year-old nephew finish together only if both beat their own lines. Opted-in leagues form the largest longitudinal cohort of non-elite movement ever assembled. It answers questions about ageing, rest and amateur training that sports science, built on young athletes, never could. Physios prescribe bench weeks, and managers enforce them.

**Scores.** Innovation **5**. Self-relative scoring, fantasy drafting and WHS mechanics all exist. Drafting friends' uncertainty-priced lines, with rest as a scoring move, is new. Feasibility **6**. Ordinary stack; everything hinges on the posted round beating its own noise, tested in month one.

### Turn 6 — Builder
**Closing remark.** 5/6 is right, and it holds the bold floor on mechanics, not hardware. Drafting a friend's uncertainty-priced line, and making rest a scoring move, turns fantasy's energy into coaching. The whole idea rests on one dull number: if a weekly submaximal test can't beat its own typical error in amateurs, the line is a slot machine. That's why the test-retest study comes before any league is built. If it passes, the Dipsea-style staggered start is the moonshot I'd actually bet on.
