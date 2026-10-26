# App brief: software-only fresh spins (phone, watch, existing wearables)

As of 2026-10-26. This brief companions [market_brief.md](market_brief.md), which already covers physical goods, TikTok Shop, Amazon, Kickstarter, PFAS bans, Trump Accounts, AI agents calling SMBs and the 63M-caregiver figure. Those are not repeated here. Every link below was opened. Vendor data is labelled as such. **(unverified)** marks claims I could not confirm from an opened page.

---

## 1. Subscription-app benchmarks, 2025–26

There are two large datasets. RevenueCat's *State of Subscription Apps 2026* covers 115k+ apps and $16B+ of revenue (2025 data). Adapty's 2026 report covers 16k+ apps and $3B, and leans toward larger apps ([RevenueCat](https://www.revenuecat.com/blog/growth/subscription-app-trends-benchmarks-2026); [Adapty](https://adapty.io/state-of-in-app-subscriptions-report/)).

| Metric | Figure | Source |
|---|---|---|
| Install → trial | 11.2% globally, 14.5% North America (Adapty). RevenueCat D30 download-to-trial: Business 9.1%, Health & Fitness 6.9%, Education/Utilities 6.5% | [Adapty](https://adapty.io/state-of-in-app-subscriptions-report/), [RevenueCat](https://www.revenuecat.com/state-of-subscription-apps) |
| Trial → paid | 27.8% (Adapty). Health & Fitness 37.7% (RevenueCat). Trials of 17–32 days convert 42.5%, ≤4 days 25.5%, yet 46.5% of apps now use ≤4-day trials | [RevenueCat blog](https://www.revenuecat.com/blog/growth/subscription-app-trends-benchmarks-2026) |
| Hard paywall vs freemium | Median D35 conversion 10.7% vs 2.1%; revenue per install at D60 $3.09 vs $0.38. One-year retention of yearly subscribers is about the same (27% vs 28%) | same |
| Download → paid, all apps | Median 2.0%; Health & Fitness 2.9%; North America 2.8% | [RevenueCat](https://www.revenuecat.com/state-of-subscription-apps) |
| Retention | Year-1 retention: annual plans 28%, monthly 8%, weekly 1.2%. First annual renewal is 23% (Productivity) to 40% (Business). 35% of annual cancellations happen in month 1. Only 5% of cancelled annual subscribers come back | [RevenueCat pt 2](https://www.revenuecat.com/sosa-2026-insights-part-2) |
| All-user D30 retention | Health & Fitness 3.9% iOS; Finance 6.6%; Education 2.7% (AppsFlyer via Business of Apps) | [Braze](https://www.braze.com/resources/articles/mobile-app-retention) |
| Typical prices | Weekly $4.99–6.89, monthly $4.99–9.99, annual $20–44.99. Median annual price rose $31.60 → $34.80; North America $39.99 | [RevenueCat](https://www.revenuecat.com/state-of-subscription-apps), [SaaStr summary](https://saastr.com/the-top-10-learnings-from-revenuecats-state-of-subscription-apps-how-115000-mobile-apps-deliver-16b-in-revenue-whats-working-whats-quietly-killing-growth) |
| Plan mix | RevenueCat: 42% of subscriptions monthly, 34% yearly. Category splits are internally inconsistent. Adapty: weekly plans are 55.5% of revenue, and one-time purchases rose to 10.3% | both |
| Refunds | 8.3% refund rate (Adapty). RevenueCat median: AI apps 4.2%, non-AI 3.5% | [Adapty](https://adapty.io/state-of-in-app-subscriptions-report/), [TechCrunch](https://techcrunch.com/2026/03/10/ai-powered-apps-struggle-with-long-term-retention-new-report-shows) |
| AI apps | Year-1 LTV per payer is 41% higher ($30.16 vs $21.37), but annual retention is 21.1% vs 30.7% | [TechCrunch](https://techcrunch.com/2026/03/10/ai-powered-apps-struggle-with-long-term-retention-new-report-shows) |

**Time to $10k MRR.** Among apps that get there, the median is **58 days to $1k MRR and 109 days to $10k**. Business apps are slowest, at 113 days to $1k. But only **17.3% of new apps reach $1k MRR within two years, and 4.6% reach $10k** (Business 1.6%) ([RevenueCat](https://www.revenuecat.com/state-of-subscription-apps)). Adapty finds 58% of new apps earn under $1k in year one. The top 10% of apps take 94.5% of revenue. New apps grew 31% YoY while median revenue per new app fell 22%. Monthly launches were 14,700 in January 2026, against 2,000 in January 2022. Apps launched in 2025 or later earn 3% of revenue ([SaaStr](https://saastr.com/the-top-10-learnings-from-revenuecats-state-of-subscription-apps-how-115000-mobile-apps-deliver-16b-in-revenue-whats-working-whats-quietly-killing-growth)).

---

## 2. Apps that found fit fast, 2022–26

| App (old habit) | Numbers | Speed | Price | Channel / lever |
|---|---|---|---|---|
| **Cal AI** (typed calorie logs → photo) | 15M+ downloads, $30M+ annual revenue, 7 staff. Bought by MyFitnessPal (closed Dec 2025, announced Mar 2026) ([TechCrunch](https://techcrunch.com/2026/03/02/myfitnesspal-has-acquired-cal-ai-the-viral-calorie-app-built-by-teens)) | Launched May 2024; under 2 years to $30M ([Fortune](https://www.fortune.com/2025/10/12/zach-yadegari-entrepreneur-gen-z-coder-rejected-by-the-ivy-league-despite-founding-a-30-million-app-you-dont-need-college-to-find-success-millionaire-co-founder)) | $2.49/mo or $29.99/yr | Influencer swarm **(unverified; TechCrunch credits rankings and press)** |
| **Opal** (willpower → Screen Time blocks) | $10M ARR with 11 people ([Speedinvest](https://speedinvest.com/blog/scaling-smart-how-opal-built-a-10m-arr-business-in-just-2-years)) | ~2 years from late-2022 launch | Charged from day 1 | **First app on Apple's Screen Time API**; performance ads |
| **one sec** (doom-scroll → forced pause) | Revenue "doubling every month" during the ad push; team of 18 ([RevenueCat](https://www.revenuecat.com/blog/growth/frederik-riedel-expected-12-his-app-cut-screen-time-by-57)) | Viral screen-recording tweet at launch | n/a | Instagram ads on the apps it blocks; Max Planck study showing **57%** less screen time |
| **Monarch** (Mint) | ~$60M ARR end-2025, ~$100M Sept 2026 (Sacra estimates); 20x growth in the year after Intuit announced Mint's closure in Nov 2023 ([Sacra](https://sacra.com/research/monarch-money-at-100m-arr/)) | Months | $99/yr | **Shutdown** plus Reddit word of mouth |
| **Cleo** (budget spreadsheet → chatty AI) | $400M annualized (May 2026 estimate), but 62% is cash-advance fees and only $90M subscriptions ([Sacra](https://sacra.com/research/400m-year-instinct-meets-cashnetusa/)) | Multi-year | n/a | Gen Z tone; the money is in lending, not budgeting |
| **Speak** (classroom English → speaking to AI) | $1B valuation Dec 2024, 10M+ downloads ([TechCrunch](https://techcrunch.com/2024/12/10/openai-backed-speak-raises-78m-at-1b-valuation-to-help-users-learn-languages-by-talking-out-loud)) | Founded ~2016; fast once LLMs arrived | $20/mo, $99/yr | Korea-first; OpenAI partnership |
| **Flighty** (airline apps → one tracker) | Self-funded and cash-flow positive; Apple Search Ads brings 20–30% of new users; no Meta/TikTok spend ([RevenueCat](https://www.revenuecat.com/blog/growth/ryan-jones-flighty-launched-podcast-2025)) | n/a | Subscription | **Live Activities on day one** → App Store features; shareable "flight passport" |
| **Ladder** (gym class → coach-led plan) | ~150k paid subscribers, 3x YoY (Dec 2024) ([Fortune](https://www.fortune.com/2024/12/10/strength-training-ladder-app-users-growth)) | ~4 years | Annual cheaper than monthly | General Catalyst funds 80% of marketing spend |
| **Finch** (self-care checklist → virtual pet) | ~$1M/month estimate ([Adapty paywall library](https://adapty.io/paywall-library/finch/), vendor estimate) | n/a | $39.99–69.99/yr | Emotional mascot |
| **Bevel** (Whoop subscription → Apple Watch data) | 100k+ DAU, 8x growth, >80% 90-day retention, $10M Series A (Oct 2025) ([FITT](https://insider.fitt.co/press-release/bevel-raises-10m-series-a-from-general-catalyst-to-build-the-future-of-personalized-health/)) | Founded 2023 | Subscription | Uses hardware people already own |
| **Granola** (meeting bot → local notepad) | $1.5B valuation (Mar 2026); net new customers 4x the prior average in Feb 2026 ([YipitData](https://www.yipitdata.com/resources/blog/granola-vs-fathom-otter-fireflies-ai-notetaking)) | ~2 years | n/a | "No bot," so it spreads quietly inside meetings |
| **Structured** (paper planner → timeline) | 1.7M+ downloads; Nov 2020 App Store feature pushed downloads past 20k/month ([Structured](https://structured.app/blog/2-years)) | Months | Started as tip jar, then $5 one-off | Apple editorial |
| **Woofz** (dog trainer → app) | $5.2M → $20M in one year; 21M downloads; ~70 staff; profitable without VC ([tech.eu](https://tech.eu/2025/08/01/bootstrapped-and-thriving-how-woofz-hit-20m-without-vc-funding/)) | ~1 year of scale | n/a | Paid social on Meta, TikTok and Snap |

Slower comparisons: Rise launched in June 2021 at $60/yr after selling to pro sports teams first ([High Alpha](https://highalpha.com/rise-science-app-launch/)). Rosebud (AI journal) had 7,500+ paying customers two years after its 2023 launch ([Rosebud](https://www.rosebud.app/blog/rosebud-raises-6m-to-expand-the-worlds-leading-ai-journal)).

**Patterns:**
1. **Remove the logging chore:** Cal AI's photo, Flighty's automatic log, Rise without a wearable.
2. **Be first on a new Apple surface:** Opal on the Screen Time API, Flighty on Live Activities. Apple promotes early adopters.
3. **Ride a forced migration:** Mint's closure made Monarch.
4. **Charge on day one, behind a hard paywall:** Opal, Cal AI. This is what makes paid acquisition pay back (section 3).
5. **Proof and shareable artifacts beat ads:** one sec's study, Flighty's passport, Granola in meetings.
6. **Tiny teams, and expect an incumbent to respond:** Cal AI had 7 people and was bought; iOS 26 copied Flighty.

---

## 3. Distribution and payments in 2026

**Store fees.**
- **Apple:** 15% in the Small Business Program (under $1M in proceeds in the prior year); otherwise the standard rate applies. Apple's page also mentions 10% on subscriptions after their first year, but the wording ties this to EU alternative terms, so it is **unverified** for the US ([Apple](https://developer.apple.com/app-store/small-business-program/)).
- **Google:** In June 2026 Google announced a 10% fee on the first $1M and on all auto-renewing subscriptions, plus a separate 5% fee when Play Billing is used. The rollout starts in the EEA and UK; US scope is unclear ([Stash](https://www.stash.gg/blog/googles-lower-fees-announcement-june-2026-whats-really-going-on-and-what-us-developers-should-do), vendor). In the US, apps enrolled in Google's link-out and alternative-billing programs must report transactions and pay fees from **1 Oct 2026**. Download reporting is due by 1 Feb 2027 ([Google Play Help](https://support.google.com/googleplay/android-developer/answer/15582165?hl=en)). Whether these fees are enforceable under the Epic injunction is contested.

**US anti-steering.**
- **April 30, 2025:** the court barred Apple from charging any commission on link-outs and from controlling how those links look or are placed ([RevenueCat](https://www.revenuecat.com/blog/growth/apple-anti-steering-ruling-monetization-strategy)). Guideline 3.1.1(a) now exempts the US storefront from the ban on purchase buttons and links ([App Review Guidelines](https://developer.apple.com/app-store/review/guidelines/)).
- **December 2025:** the Ninth Circuit upheld the contempt finding but said Apple may charge a fee for costs "genuinely and reasonably necessary." The fee will be set on remand.
- **June 30, 2026:** the Supreme Court took the contempt question (No. 25-1311) for this term. Apple has collected **0%** on link-outs for about a year ([TNW](https://thenextweb.com/news/supreme-court-apple-epic-contempt-app-store-commission); [9to5Mac](https://9to5mac.com/2026/07/01/apple-to-ask-court-to-halt-epic-games-proceedings-pending-supreme-court-ruling/)).

**What web checkout did to margins:**
- **Fees:** a link-out swaps a 15–30% store fee for roughly 3% card processing **(unverified list price)**. The developer then carries sales tax, fraud and failed payments. Most apps must still offer in-app purchase alongside the link.
- **Revenue share:** web checkout is still only **3.2% of revenue (4.9% in North America)**. Yet **41% of top-tier apps already sell on the web, against 1.3% of the smallest** ([SaaStr](https://saastr.com/the-top-10-learnings-from-revenuecats-state-of-subscription-apps-how-115000-mobile-apps-deliver-16b-in-revenue-whats-working-whats-quietly-killing-growth)).
- **Treat it as optional upside:** the fee could return after the Supreme Court decision or the remand.

**Install costs.** Apple Ads, US, 2025 medians of cost per tap / cost per install ([AppTweak](https://www.apptweak.com/en/aso-blog/apple-ads-benchmarks?lang=en-US)):

| Category | CPT / CPI |
|---|---|
| Utilities | $1.23 / $2.25 |
| Lifestyle | $1.18 / $2.62 |
| Education | $1.52 / $2.91 |
| Productivity | $1.74 / $3.58 |
| Health & Fitness | $1.68 / $3.77 |
| Business | $3.33 / $6.86 |
| Finance | $4.17 / $9.65 |

Blended iOS CPI rose 44% QoQ to $9.11 in Q1 2026 ([Singular](https://lp.singular.net/q1-2026-quarterly-trends-report), games included). I found no primary source for Meta or TikTok CPI by category **(vendor ranges of ~$2–5.50 for health and ~$3.50–9 for finance are unverified)**.

**The arithmetic.** At a $3.77 CPI:
- **Freemium:** 2.9% download-to-paid → **~$130 per payer**, against a Health & Fitness yearly realized LTV of **$35.64** ([RevenueCat](https://www.revenuecat.com/state-of-subscription-apps)).
- **Hard paywall:** 10.7% → **~$35 per payer**.

So paid acquisition only works with a hard paywall, a web funnel or organic reach.

**Web-to-app funnels** (FunnelFox vendor data, [State of Web2App 2026](https://funnelfox.com/state-of-web2app-2026)):
- **Adoption:** 82% of top-grossing apps run them.
- **Conversion:** final conversion is 3.0% on web funnels vs 1.5% in-app.
- **LTV:** Health & Fitness LTV is ~$80–90 on web vs ~$40–45 in the app.
- **Plan mix:** monthly plans take 48.8% of web revenue, weekly 30.5%.
- **Payments:** Apple Pay is ~60% of payments, but **30–50% of initiated payments fail**.
- **Refunds:** 92% happen in the first month.

**Creators and UGC.** UGC campaigns average **$197** (creators paid ~$154), and TikTok's share of Collabstr engagements fell from 41% to 21% in a year ([NetInfluencer/Collabstr](https://www.netinfluencer.com/collabstr-report-tiktok-campaigns-plummet-as-brands-pivot-to-ugc/)). The playbook: many cheap creators repeating one 5-second demo (a plate photographed, a blocked app's pause screen), all pointing to a web quiz funnel.

---

## 4. What's newly possible on phones and wearables

- **Apple Foundation Models (iOS 26).** An on-device LLM with structured output and tool calling, at no inference cost. Already used by Stoic (journaling prompts), SmartGym, Streaks and Stuff ([9to5Mac](https://9to5mac.com/2025/09/29/apple-intelligence-ios-26-models-developers/)). WWDC 2026 (iOS 27) made the on-device model **multimodal (accepts images)** and opened Apple's **Private Cloud Compute model** to apps ([Superwall](https://superwall.com/blog/3-things-for-developers-from-wwdc-2026)).
- **Private Cloud Compute access.** **No cloud API cost** for Small Business Program developers with under 2M first-time downloads. Daily usage limits apply. Above 2M downloads you must migrate within 6 months, and a paid tier is coming ([Apple](https://developer.apple.com/private-cloud-compute)).
- **Siri and App Intents (iOS 27).** Apps that fit an App Intent Domain get Siri natural-language queries and data export. A new modifier tells Siri what is on screen ([Superwall](https://superwall.com/blog/3-things-for-developers-from-wwdc-2026)). The "Siri AI is Gemini-based" claim comes from secondary coverage **(unverified)**.
- **Android on-device AI.** ML Kit GenAI (Gemini Nano) offers summarise, proofread, rewrite and image description, with no per-call charge, on recent Pixel, Samsung and other devices ([Android Developers](https://android-developers.googleblog.com/2025/05/on-device-gen-ai-apis-ml-kit-gemini-nano.html)).
- **Apple Watch (watchOS 26).**
  - **Hypertension notifications:** FDA-cleared, Series 9 and later. Apple expects to notify **1M+ people with undiagnosed hypertension in the first year**, and tells them to log blood pressure for 7 days ([Apple Newsroom](https://www.apple.com/newsroom/2025/09/apple-debuts-apple-watch-series-11-featuring-groundbreaking-health-insights/)).
  - **Sleep score:** free.
  - **iOS 27:** adds perimenopause tracking, and Apple's AI health coach was reportedly delayed. HealthKit still "measures but doesn't interpret" ([Sahha](https://sahha.ai/blog/wwdc-2026-apple-health-developers/), vendor).
- **Health Connect (Android).** Adds background reads, history beyond 30 days, skin temperature and planned workouts ([Android Developers](https://android-developers.googleblog.com/2025/03/health-connect-jetpack-sdk-now-in-beta.html)). Google launched the Google Health API in March 2026 and is winding down the Fitbit Web API in September 2026 ([Terra](https://tryterra.co/blog/garmin-connect-developer-program-pause)).
- **Wearable APIs are closing.**
  - **Oura:** apps are capped at 10 users before approval, and users need a membership. The **agreement effective 8 June 2026 bars sending Oura API data to AI models (§4(d))** ([Terra](https://tryterra.co/blog/oura-api-ai-apps-access-permissions-commercial-restrictions)).
  - **Garmin:** paused new developer access in spring 2026 ([Terra](https://tryterra.co/blog/garmin-connect-developer-program-pause)).
  - **Whoop:** V2 API with a 10-member developer cap **(unverified)**.
  - **Implication:** build on HealthKit or Health Connect, not vendor clouds.
- **Live Activities.** Since iOS 18, one broadcast push updates every subscribed Live Activity on a channel, so a shared event costs one push instead of one per token ([WWDC24](https://developer.apple.com/videos/play/wwdc2024/10069/)).
- **Built-in safety features.**
  - **iOS Check In:** timer or arrival based. If missed, it shares location, battery, signal and, in full mode, route, last unlock and Watch removal. Both people need iOS 17 ([Apple Support](https://support.apple.com/en-au/guide/personal-safety/ips56b5bc469/web)).
  - **Android Safety Check:** 1 minute to 8 hours; starts from Wear OS 4+; texts location if missed ([Google](https://blog.google/products/pixel/google-pixel-phone-watch-safety-features/)).
- **What got cheaper.**
  - **On-device and Apple cloud inference:** free (above).
  - **Frontier LLMs:** the price of a fixed capability fell 9x–900x per year ([Epoch AI](https://epoch.ai/data-insights/llm-inference-price-trends)).
  - **AI at scale:** Duolingo held a 72.6% gross margin while rolling out AI video calls ([Duolingo 8-K](https://www.sec.gov/Archives/edgar/data/0001562088/000162828026053299/q2fy26duolingo6-30x26share.htm)).
  - **Implication:** AI features are no longer a cost problem, so they are no longer a moat.

---

## 5. Categories where people already pay and demand is shifting

1. **Sleep and focus.** People pay to be stopped: Opal reached $10M ARR, and one sec scaled on Instagram ads. Bevel shows people will pay to get more from a watch they already own. The risk is that Apple's free sleep score now covers the basics.
2. **Budgeting.** Monarch's ~$100M ARR at $99/yr shows post-Mint demand; Cleo's mix shows the big money is in lending, not budgeting. Finance has the highest install cost (CPI $9.65) but the best D30 retention found (6.6% iOS), and apps must be submitted by a legal entity (5.1.1(ix)).
3. **Caregiving and checking on older parents.**
   - **Devices:** **79% of Americans 65+** own a smartphone; 14% have a non-smartphone cellphone ([Pew 2026](https://www.pewresearch.org/internet/fact-sheet/mobile/)). Among adults 50+ it is **90%**, wearables lag younger adults, and 55% of caregivers use tech tools ([AARP 2026 Tech Trends](https://www.aarp.org/pri/topics/technology/internet-media-devices/2026-technology-trends-older-adults/)).
   - **Free OS features cover the emergency:** Check In (the parent must start it); Apple Watch fall detection, which auto-calls emergency services and texts contacts and defaults to workouts-only for ages 18–55 at setup ([Apple Support](https://support.apple.com/en-au/guide/watch/apd34c409704)); Pixel Watch fall detection and Safety Check.
   - **The gap:** none give a routine, passive "is Mom OK today" signal or catch slow drift (less walking, missed meds).
   - **Willingness to pay:** Greenlight sells a **$24.98/mo "Family Shield" plan aimed at caregivers of seniors** ([Sacra](https://sacra.com/research/greenlight)). HHS launched a $2M Caregiver AI Prize in November 2025 ([DailyCaring](https://dailycaring.com/ai-caregiving-tools/)).
4. **Parenting and kids' money.** Greenlight has 6.5M+ parents and children and about $228M in 2024 revenue (Sacra estimate). It distributes through 150+ bank and employer partners, so banks are a channel as well as an app store. Trump Accounts are covered in the companion brief.
5. **Pets.** Woofz shows a dog-training app can grow 4x in a year on paid social. Pet spending details are in the companion brief.
6. **Language.** Duolingo: 58.7M DAU (+23%), 12.7M paid subscribers, 84% next-day retention, Q2 2026 ([8-K](https://www.sec.gov/Archives/edgar/data/0001562088/000162828026053299/q2fy26duolingo6-30x26share.htm)). Speak charges $99/yr for speaking practice. The winning wedge is speaking practice, not vocabulary drills.
7. **Freelancer taxes and receipts.**
   - **Fewer tax forms:** OBBBA restored the **1099-K threshold to $20k and 200 transactions**, and raised **1099-NEC/MISC to $2,000 from 2026** ([Avalara](https://www.avalara.com/us/en/blog/2025/07/one-big-beautiful-bill-act-1099-reporting-threshold)). Freelancers will get fewer forms and must track income themselves.
   - **A new deduction:** **"no tax on tips"** allows up to **$25k a year for 2025–2028**. It covers 68 occupations and phases out above $150k AGI. New W-2/1099 reporting starts in 2026 ([BDO](https://www.bdo.com/insights/tax/irs-lists-jobs-that-qualify-for-obbbas-no-tax-on-tips)).
   - **Forcing deadline:** the 2027 filing season, with prep starting January.
   - **Incumbent:** Keeper raised a $13M Series A ([media kit](https://keepertax.com/media-kit)); its revenue is **unverified**.
8. **Trades field apps and AI receptionists.** Jobber's Receptionist went GA on 18 Aug 2025 and handled 200k+ conversations; 96% of home-service consumers say response time affects who they buy from ([PM Mag](https://www.pmmag.com/articles/106754-jobber-launches-ai-powered-receptionist)). Jobber's "Teammate" agent entered beta in Sept 2026 **(unverified)**. Business apps are slowest to $1k MRR (113 days), but have the best first annual renewal (40%). AI notetakers are crowded: Granola is consolidating the space, and Fireflies, Otter and Fathom are stalling.

---

## 6. Failure modes

- **Retention cliffs.** 55% of 3-day trial cancellations happen on day 0, 35% of annual cancellations in month 1, and year-1 retention is 28% annual / 8% monthly ([RevenueCat](https://www.revenuecat.com/sosa-2026-insights-part-2)). Weekly web plans keep ~3% by month 12 ([FunnelFox](https://funnelfox.com/state-of-web2app-2026)).
- **Why habit apps die by week 4.** The median time to automaticity was **66 days (range 18–254)** ([Lally et al., via The Behavioral Scientist](https://www.thebehavioralscientist.com/articles/how-long-to-form-a-habit)), but Health & Fitness D30 retention is 3.9%, so most users leave about a month before a habit forms. Those who succeed no longer need the app (Productivity has the lowest first annual renewal, 23%). Design for value after the habit forms, or for a recurring external trigger.
- **Sherlocking.**
  - **iOS 18:** put about **$393M** of annual app revenue at risk (Appfigures; [TechCrunch](https://techcrunch.com/2024/06/18/ios-18-could-sherlock-400m-in-app-revenue)).
  - **iOS 26:** copied Flighty (Wallet Live Activities), Truecaller and Robokiller (Call Screening) and package trackers ([TechCrunch](https://techcrunch.com/2025/06/10/wwdc-2025-everything-that-apple-sherlocked-this-time/)).
  - **ChatGPT Health:** opened to all eligible US users on 23 July 2026. It reads Apple Health data and medical records ([iClarified](https://www.iclarified.com/101568/openai-launches-chatgpt-health-to-all-us-users-with-apple-health-integration)). An app whose pitch is "AI reads your health data" is now competing with a free product.
- **Thin AI wrappers.** AI apps keep annual subscribers 30% worse. Their install-to-trial rate is 5.3%, against 10.9% for the average app ([Adapty](https://adapty.io/state-of-in-app-subscriptions-report/)). Inference prices are collapsing. Guideline 4.3(b) rejects apps that copy existing categories. Incumbents can also buy the leader, as MyFitnessPal did with Cal AI.
- **Rising acquisition costs and crowding.** Blended iOS CPI rose 44% QoQ. Launches are up 7x since 2022. Bottom-quartile apps shrank 33% YoY.
- **App Review and policy risk** ([Guidelines](https://developer.apple.com/app-store/review/guidelines/)): 1.4.1 bars claiming blood pressure or glucose from phone sensors and requires disclosed methodology; 5.1.3 bars health data in ads and personal health information in iCloud; 5.1.2(i) requires explicit consent before sharing data with third-party AI; 5.1.1(ix) requires finance and health apps to come from a legal entity; 3.1.2 removes bait-and-switch subscriptions. Oura's API terms now block AI use.
- **Web-checkout traps.** 30–50% of initiated web payments fail. The 0% Apple fee may not survive the Supreme Court.

---

## What this means for the brainstorm

1. Spin a chore people already log or check by hand, and remove the input step: use a photo, the watch, HealthKit or Health Connect, or a Live Activity.
2. Use hardware the user already owns (iPhone, Apple Watch, Pixel Watch). Avoid Oura, Garmin and Whoop APIs as the core: they are capped, paused or AI-restricted.
3. Plan for a hard paywall or a web quiz funnel at $30–60 a year. Freemium at a $3–10 CPI does not pay back.
4. Choose a wedge with a clock or trigger: hypertension alerts (BP logging), the 2027 tax season (tips deduction, 1099 changes), a parent's fall or decline, a shutdown or price hike.
5. Older-parent ideas should add a routine passive signal plus a shared view for the adult child. Do not rebuild Check In or fall detection, which are free.
6. Use on-device Foundation Models, Gemini Nano and free Private Cloud Compute to keep marginal cost near zero. Then assume any competitor can do the same, so AI cannot be the moat.
7. Build a moat from data the user accumulates, a shared artifact that spreads (a passport, card or report sent to someone else), or proof (a study or measured outcome).
8. Avoid features Apple or ChatGPT ship free (sleep score, generic AI health chat, call screening). Ask "would WWDC 2027 kill this?"
9. Avoid diagnostic claims and keep finance features clear of lending. Submit as a company. Budget for consent screens before sending data to third-party AI.
10. Target $10k MRR in about 4 months: RevenueCat's median is 109 days for apps that get there, and only 4.6% do. Kill or pivot ideas that cannot show 2.9%+ download-to-paid and a working creator demo within 60 days.
