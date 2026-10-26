# Idea 07 — Seated
> Seed: 07. **The paper blood-pressure log** (consumer health app on the phone and watch). Doctors ask patients to keep a week of home readings in a notebook, and the Apple Watch now flags possible hypertension, sending millions of people to buy a cuff and start logging. Possible borrowings: #11 Washout (a self-experiment you can't fool: did cutting salt or walking after dinner actually move your numbers?), #20 Facing Out (share with the people you choose), Morning Tile's ritual design. No diagnostic claims; logging, reminders and a doctor-ready report are the line.

### Turn 1 — Visionary
**Working name: Seated.**

**The trigger.** The 2026 Medicare fee schedule added CPT 99445, which pays for remote monitoring on 2–15 days in a 30-day period (about $47–52, vendor figures), plus 99470 for 10 minutes of review (about $26). A 7-day home log, which is exactly what the Apple Watch's hypertension notification tells a million-plus people a year to keep, is now billable for the first time. Self-measured BP codes already exist: 99473 for training (about $15) and 99474 for a month of reviewed readings (about $17–18; aggregator rates, unverified).

**Proven spend, so the clinic pays.** Practices already buy RPM programs. My first buyer is an independent primary-care practice or FQHC (99445 is payable to FQHCs from January 2026, vendor report), in Q4, when it chases blood-pressure gaps before year end.

**The stale part.** Paper logs and free apps record numbers, not how they were taken. Apple Health stores readings with no protocol. Omron Connect and Withings Health Mate are free but tied to one brand. None shows the clinician that the patient sat for five minutes, took paired readings, morning and evening. So doctors discount home numbers.

**The one change: a reading counts only if the watch saw you rest first.** The watch counts down five still, seated minutes and resets if you get up. Then it says "cuff now," takes paired readings a minute apart (any HealthKit or Health Connect cuff), and ticks "protocol-valid." Day 7 sends a one-page AHA-style average to the clinic. It never labels a stage.

**Price:** $9 per active patient-month to the clinic; patients use it free. Self-pay, $29.99 one-off for a doctor-ready week.

**Borrows:** #38 Tally and Ran It, so the event (verified rest) sets the record. #20 Facing Out: it faces only the clinic and the people the patient chooses.

### Turn 2 — Builder
**Codes, checked.** 99445 (device supply, 2–15 days in 30, FDA-cleared device, exclusive of 99454) and 99470 (first 10 minutes, one interactive communication) took effect January 1, 2026 (dollar figures from vendor blogs). RPM needs consent, an established patient and automatic transmission. A cleared Bluetooth cuff syncing to HealthKit qualifies only if we take device-sourced readings and reject typed ones. Billing guides treat 99445 as the practice *supplying* the cuff, so a patient's own is a grey area (unverified). 99474's 12-reading rule and its FQHC status: unverified.

**Incumbents.** Few publish prices: Accuhealth $20/patient-month plus $25 device and $15 onboarding; MedM $10; full-service programs $40–80 (vendor blogs). Cadence, Tenovi, Smart Meter, HRS, Validic, Omron VitalSight, Withings RPM and employer-paid Hello Heart: unpublished. Several ship cellular cuffs because phone pairing fails at 75. Nobody verifies rest: nobody pays for it.

**The watch** can confirm a still wrist and settling heart rate for five minutes (extended runtime session). It can't confirm seated, feet flat or arm at heart height. Label it "rested 5 min · paired," not "protocol-valid," which keeps it data handling rather than a device function (my reading, unverified).

**Hardest problem: the user isn't the patient.** Series 9+ owners aren't the Medicare and FQHC patients RPM pays for. Clinics bring BAAs, security reviews and 6–12-month FQHC cycles (estimate); $9 just undercuts a commodity.

**Twist: the patient pays, the clinic reads free.** $39.99/yr for repeat doctor-ready weeks, sold at the notification or the "keep a log" appointment. A patient-chosen app isn't a business associate (the FTC breach rule applies), and the clinic can bill 99473/99474 off the page. It nets $34 after Apple, against about $35 per payer (Health CPI $3.77, 10.7% paywall).

**MVP:** watch rest timer, HealthKit/Health Connect cuff import, a seven-day PDF, 6 weeks.

**Risk:** WWDC 2027 adds a seven-day BP summary to Health.
