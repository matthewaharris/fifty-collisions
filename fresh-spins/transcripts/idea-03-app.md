# Idea 03 supplement — Morning Tile as an app

### Turn A — Visionary
**Her morning: the home screen becomes the fridge.** A Morning Tile widget shows three tiles: green *fine*, blue *out, don't fuss*, orange *call me today*. Interactive widgets (iOS 17+, Android) take one tap, with no app to open and no login. The tap plays today's family voice note, and a big Reply button records her answer, or a photo of the tomatoes. On an Apple Watch the tiles become a complication, and Siri works too ("Morning Tile, green"). The "while the kettle boils" sticker still ships, on the welcome card.

**The family sees** one shared Live Activity: "Mum: green, 8:12." A single broadcast push updates every sibling's lock screen (iOS 18). Below it sits the thread of her replies. Builder's turn-4 contributor rules carry over unchanged: payer-invited, OTP-verified, approved by her and screened for money talk.

**Silence.** At her chosen hour her phone shows one line, "Lily left you something." That's her only nudge, and it's a gift, not a chime. Ninety minutes later her one chosen person gets "No tile from Mum yet." She alone can opt in to one more thing: if HealthKit or Health Connect shows 300+ steps since 6am, that alert turns grey, "up and about, no tile." Steps can soften an alert but never raise one.

**Not Check In.** Check In is a timer she starts for each trip, and if it lapses it hands over location, battery, route and last unlock. Morning Tile is a daily ritual that shares a colour and a time, never location, and the reward is the point. If WWDC 2027 ships a recurring Check In, Apple has shipped the alarm, not the grandchildren.

**Lost:** an object in her kitchen that visitors see; the 21% of over-65s without a smartphone (Pew 2026); the feel of her hand on the wall. **Gained:** no hardware and no IVR minutes. The payer goes through a hard-paywalled web funnel ($12/month or $99/year), and her app is free. The Live Activity is the shareable artifact, and setup takes five minutes at Sunday lunch. The phone number stays for mothers on landlines.

### Turn B — Builder
**iOS claims, checked.**
- **Widget:** right: iOS 17 widgets run an App Intent in one tap, and an audio-playback intent plays the note without opening the app. Reply must open the app to record.
- **Live Activity:** broadcast push is iOS 18, but an activity updates for 8 hours at most and leaves the lock screen by hour 12. Push-to-start it (iOS 17.2+) at her chosen hour; end it when the tile lands. Broadcast saves nothing for three siblings. Android siblings get a plain notification.
- **Siri:** the phrase must name the app ("Green in Morning Tile"). A bonus, not a pillar.
- **Watch:** a complication only opens the app, so two taps; watchOS 26 controls might make it one (WWDC25 session 334). Cut from v1: wearables lag among older adults (AARP 2026).

**Competitors:** Snug (free), AllsOK, Are You Ok?, Life360, and the real one: the family WhatsApp group, where grandchildren already send voice notes free. The only new thing is the gate.

**The 80-year-old.** I found no retention data by age; RevenueCat and Adapty don't split it. A widget is a strong cue for a 70-year-old who unlocks thirty times a day, and a weak one for an 85-year-old whose phone lives in a drawer. Failure is silent: a deleted widget, a new phone or muted notifications send false "no tile" alerts, which lose the payer faster than churn.

**Price and channel:** $12/month or $99/year to the payer through a web funnel, with in-app purchase beside it; her side free. Cost to serve under $0.50/month. Meta CAC still about $70: two people must be won.

**Verdict: beside the board, never instead.** One backend. At setup the daughter picks Mum's door (widget, phone number, or both), and every family gets the printed card with the number. The widget adds about three weeks to the family app. Kill the widget door if under 60% of app parents tap 5 of 7 days by day 60.

**Scores (app alone): 3/8/5.** Snug plus a gated voice note, no new dated trigger, and it loses the over-80s. Bundled with the line, 6.
