# Idea 03 — Morning Tile
> Seed: 03. **The fridge calendar and the "call Mum" routine** (caregiving for ageing parents; app, device or service). Families check on parents through calls, texts, shared calendars and the occasional camera. Possible borrowings: #20 Facing Out (warning signs that face out only to chosen people, through scarce, graded clues), #38 Tally (a nightly check that proves the system still works), #31 Tombstone (map what depends on what before a crisis). Respect the parent's dignity and stay clear of medical-device claims.

### Turn 1 — Visionary
**Working name: Morning Tile.** The old products are the fridge calendar and the 9am "just checking you're alive" call. It makes the daughter an inspector. Families who escalate end up at the pendant ($25–45/month), which many independent 78-year-olds won't wear because it announces "frail."

**The one change:** the parent sends the signal, nobody watches her. A magnetic fridge calendar has one "today" slot. Each morning she drops in a tile: green *fine*, blue *out, don't fuss*, or orange *call me today*. Her family's phones show only that tile and the time it went in. No camera, mic or motion sensing, and no health claims. If no tile arrives by her own chosen hour, the board chimes and asks her first. Only then does it text her one chosen person. Inside: magnet-coded tiles, a Hall-sensor slot and a pre-certified LTE-M module, so no Wi-Fi needed.

**Buyer: the adult child, paying now.** Not Medicare Advantage or Area Agencies on Aging: their bid and grant cycles are too slow. Adult children are already the payers: 63M caregivers spend about $7,200/yr out of pocket (AARP 2025), on a credit card. They're reachable through Meta ads to women aged 45–64 and caregiver groups. The moment: the week after the call that went unanswered for three hours.

**Why now:** Amazon's Alexa Together, a $19.99/month family check-in subscription, is "no longer available" (Amazon note, May 2025). Its families lost a service. And 75% of over-50s want to age in place (AARP 2024).

**Borrows:** from #20 Facing Out, her signs face out only to people she chooses, as scarce graded clues. From #38 Tally, a weekly drill texts the family "test: tap if you'd have got this," which proves the chain works.

**Price:** $129 board plus $12/month: cheaper than the pendant, and earlier.

### Turn 2 — Builder
**Facts check out:** Amazon's May 21, 2025 note says Alexa Together is gone (pointing to Emergency Assist); AARP figures match the brief.

**What exists (crowded):**
- **Snug:** free daily check-in, with a $19.99/month dispatch tier.
- **Other apps:** AllsOK, OneTap Safe, KinStatus, "Are You Ok?"
- **County and police "RUOK" calls:** free.
- **Pendants** (Medical Guardian, Lively): $25–45/month.
- **GrandPad and passive sensors** (Alarm.com Wellness via security dealers): prices unverified.

None is a phone-free ritual the parent starts herself. That gap is why hardware is the point: her pride lives in a fridge object, not an app.

**Hardest problem: the habit lapses.** She forgets on day 9, the chime nags, the daughter gets texts, and the board goes in a drawer. The parent churns; the child pays.

**Hardware reality:**
- **Certification:** a pre-certified LTE-M module (nRF9160 class) still needs $6.5–10k of FCC testing.
- **Carrier:** an IoT MVNO (Hologram, 1NCE) avoids a carrier deal, about $1/month (unverified).
- **Power:** no outlet on a fridge door, so AA cells and power-save mode.
- **BOM:** about $35–45, landed about $50.
- **Economics:** the $129 board nets about $70; $12/month leaves about $10.50. A Meta CAC of $80–150 pays back in 1–8 months. Six-month retention decides it, not margin.
- **Positioning:** "not an emergency device" on the box, or a missed fall becomes a lawsuit.

**Twist: prove the ritual before the radio.** Ship a $15 unpowered magnetic tile board. She reports her tile by pressing 1, 2 or 3 on a free number from her own landline (Twilio IVR, about $0.02 a call). Same parent-initiated signal, chosen hour and one-person escalation; no certification, carrier or returns. Charge $12/month from day one. Build the LTE-M board only once 60-day IVR retention passes 70%.

**MVP:** IVR, SMS dashboard, printed boards, 30 families, 4 weeks.

### Turn 3 — Visionary
**Accepted:** the $15 board and landline IVR first, with radio only after 70% retention at 60 days.

**Day 9: make the call pay her, not us.** Pressing her tile number unlocks today's message from the family. Family members send voice notes to the Morning Tile number, and one plays the moment she reports: "Hi Gran, it's Lily, I lost a tooth!" After the tone she can answer, and her reply goes to the family thread. The tile becomes an answering machine that only plays when she says she's fine. It borrows the daily-unlock pull of Wordle and the advent calendar.

Any nudge goes to the payer, never to her. The child's dashboard says "Mum's had no message from you in 3 days." If nobody recorded anything, the line plays a question from her own life ("What was your first job?") and keeps her answer for the family archive. A missed morning is never a chime. At her chosen hour the line rings once: "Lily left you something."

Habit-stack it: the board ships with a sticker that says "while the kettle boils," and it hangs beside the kettle.

**Five-second demo:** she slides in green, lifts the landline, presses 1. Grandchild's voice, she laughs. Cut to the daughter's phone: "Mum: green, 8:12."

**Demand test (3 weeks, about $1.5k):**
- **Ads:** Meta ads to women aged 45–64 for "Mum checks in, and gets a message from the grandkids," with a $12/month card-on-file pre-sale.
- **Go:** paid CAC of $60 or less, and in the first 30 families, 70%+ of parents calling 5 of 7 days in week 3, with 60%+ of days carrying a family message.
- **Kill:** if families stop recording, the pull dies.

### Turn 4 — Builder
**The voice notes are a scam surface, so lock them down.** Voice-clone "distress" scams reportedly passed $5M in losses in FBI IC3 2025 (secondary sources, unverified).
- **Recording:** in-app only, from OTP-verified contributors the payer invites and the parent approves once on the line ("press 1 to accept Lily"). No phone-in recording, because caller ID can be spoofed.
- **Screening:** transcripts mentioning money, bail, gift cards or urgency don't play; they go to the payer.
- **Cut:** the outbound "Lily left you something" ring. The board says "Morning Tile never calls you and never asks for money."
- **Cut:** the fallback questions. "First job" is a bank security answer, so keep non-identifying prompts only.
- **Data:** voice kept 30 days, encrypted; no accounts for grandchildren.

**Phones:** 65+ wireless-only was 17% in 2014 (NHIS), far higher now (2025 split not found). Irrelevant: the IVR works from a mobile speed-dial.

**When families stop recording,** it's a plain check-in again; churn follows family effort.

**12-month plan (assumptions I'd defend):**
- **Price:** $12/month, board free on annual ($120).
- **CAC:** about $80 on Meta, above the brief's $53, because two people have to be won.
- **Churn:** 5% a month, so about $210 lifetime contribution, about 2.6x CAC.
- **First 100:** 30 from the pre-sale cohort, then caregiver Facebook groups, AgingCare and sibling referrals ($12 credit).
- **Month 12:** 1,000 families, about $12k MRR, about $80k of ads.
- **Legal:** "not an emergency service" everywhere; a spoken notice that replies are shared, for two-party-consent states.

**90 days:** weeks 1–3, demand test. Weeks 4–8, IVR, app and screening. Weeks 9–13, 100 families.

**Kill:** under 60% of parents calling 5 of 7 days by day 60; churn over 8% a month by month 4; or CAC over $100 after month 3.
