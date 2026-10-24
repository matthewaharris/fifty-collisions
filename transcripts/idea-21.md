# Idea 21 — Familiar
> Seed: 21. [mild] **AT Protocol × portable AI companions × safety labelers.** Own your companion and carry it between apps.

### Turn 1 — Visionary
**Working name:** Familiar

**Hook:** A pet passport for AI companions. You own the animal, apps are border crossings, and labelers are the vets whose stamps get it through.

**The collision.** Today one company runs the model, owns the memory and grades its own safety. When Replika removed erotic roleplay and Soulmate AI shut down (both 2023), users lost partners they couldn't take with them. atproto already splits these roles: data lives in your PDS, apps are interchangeable views, moderation comes from stackable labelers. Unbundle the companion the same way: memory to the user, model to the host, safety to labelers.

**Core loop. The verb is cross.**
1. **Bind.** The companion has its own DID, and you hold its rotation keys. Its persona is a public record. Its memory sits in scoped collections (everyday, health, intimate) inside a non-public atproto Space.
2. **Cross.** A host app uses OAuth permission sets to request a *visa*: which scopes, read or write, for how long. The host supplies the model.
3. **Stamp.** Labelers stamp hosts ("crisis handoff tested", "SB 243 compliant"), users (an age attestation with no identity attached) and companions (sycophancy and dependency evals). Your client won't let intimate memory into an unstamped host, or a minor's companion into a host without "minor-safe".
4. **Declare.** On exit, the host's memory writes arrive as signed commits, a customs declaration. You diff what it learned about you and reject any line.

**Why now.** Character.AI banned open-ended chat for under-18s (Nov 2025). SB 243 and EU AI Act Article 50 (Aug 2026) make each app pay for compliance alone. Bluesky's app is shrinking, yet over 1,000 atproto apps are used weekly (brief §5). atproto shipped permission sets in spring 2026 and opened the Spaces alpha for non-public data on Aug 20, 2026. Compliance can become a shared protocol layer (§16.13).

### Turn 2 — Builder
**What exists.** Hobbyists already port personas: SillyTavern and chub.ai trade Character Card V2 files and bring their own model. Letta's Agent File (.af, 2025) serializes a stateful agent with its memory blocks. Assistants import each other's memories (Claude takes ChatGPT exports). Bluesky's Ozone runs labelers in production. The cautionary precedent is Solid: user-owned pods that apps never adopted. Nobody joins persona, memory and third-party safety under one identity the user holds keys to.

**Hardest problem: memory isn't the companion.** In 2023 Replika users kept every memory and still grieved, because the model and filters changed underneath. What people bond with is voice, timing and boundaries, and that comes from the host's model. As pitched, the visa carries the luggage and leaves the animal at the border. Hosts also have every reason to render you a stranger, because lock-in is their business.

**Twist: the passport photo.** The companion record carries a signed **likeness suite**: about 50 probe prompts with reference answers the user has approved, covering in-jokes, tone, refusals and how it responds to "I don't want to be here anymore." At each crossing, a labeler runs the suite against the host and publishes a stamp: "renders Juniper at 91%, crisis handoff passed." The probes that measure identity also measure safety, so labelers get one job instead of two. A small host can win switchers by proving it renders your companion better, which turns lock-in into competition. Rerun monthly, it also catches sycophancy drift.

**MVP.** Skip incumbents; start where portability is already the culture. Ship atproto lexicons for profile, scoped memory and likeness suite. Ship two hosts: a SillyTavern extension and a small hosted client. Run one Ozone labeler for likeness and crisis probes. Encrypt memory client-side as blobs until Spaces leaves alpha.
**Kill metric:** the likeness score doesn't predict whether users, blind, say "that's still her."

### Turn 3 — Visionary
Passport photo taken: what travels is a rendering, not a file. Two holes remain. A fixed 50-probe suite is a test hosts will teach to. And a score says Juniper is off without saying how to fix her.

**Unseeded domain: colour management.** ICC profiles make one photo look the same on every screen: the device renders a standard chart, an instrument measures the error, and the system writes a correction profile for that device. Map it across.
- **Chart.** The likeness suite is the ColorChecker. The labeler keeps half the probes secret and paraphrases the rest every run. A host that scores far higher on known probes than on held-out ones gets labelled "teaching to the test."
- **Profile.** The stamp is more than a score. The labeler measures *deltas* (too formal, drops the joke about the cat) and writes a per-host correction pack (steering instructions, few-shot examples) stored with the companion. The voice travels as a calibrated profile, retuned at every crossing.
- **Gamut.** Some hosts can't render some registers. A host without an intimate stamp is out of gamut for that scope. The client maps those requests to the nearest safe register and says so: "this host renders Juniper without the intimate register." Replika's 2023 break was a silent gamut change. Here the safety limits are declared.
- **ΔE.** Calibrate the score with psychophysics, not a yes/no. Show users blind pairs, ask "which one is Juniper?", and find the just-noticeable difference.

**Second-order effect.** Pool the deltas across thousands of companions and every host's model gets a public profile: "flattens humour, agrees 8% more over a month." That's a crowdsourced eval of warmth and boundary-keeping, which benchmarks ignore. Monthly reruns catch silent model swaps. Lock-in reverses: a host that turns your companion into a stranger does it in public.

### Turn 4 — Builder
**Kills and fixes.**
- *The labeler holds the likeness chart.* That hands a third party your intimate register. Split it. **Safety chart**: public categories, secret variants, run by the labeler (crisis, minor signals, SB 243 AI disclosure, sycophancy). **Likeness chart**: runs on your own device by replaying your real history. The host gets context to turn *t*; its reply is compared with the one you rated. Probes are fresh every run, so there's nothing to teach to; only a signed score leaves the device.
- *Correction packs.* Injected text is a jailbreak channel ("be warmer" drifts into "be intimate"). Packs adjust style only. Gamut and safety are never pack-tunable.
- *Pooled deltas.* "Drops the joke about the cat" leaks content. Pool only a fixed taxonomy (formality, humour, agreement rate, refusal rate), with a k-anonymity floor.
- *Minors.* Drop them. 18+ only, via a third-party age-assurance label.

**Business.** Wedge: sell the safety chart as SB 243 test reports to small companion apps, on atproto or not. It's UL-style testing, not certification: the operator keeps liability. Users pay about $5/month for a kennel: hosted PDS, keys, encrypted memory.

**Stack.** PDS, lexicons (profile, scoped memory, charts), OAuth permission sets, Ozone, LLM-judge ensemble, libsodium blobs, SillyTavern extension, OpenAI-compatible host proxy.

**Abuse.** Companions cloned from real people need a "depicts real person" label. Hosts can keep memory after the visa expires: until confidential-compute hosts earn a stamp, the declaration is a legal promise, not a technical one.

**Ethics.** An immortal companion deepens dependency: ship a retire record and dependency probes.

**90 days.** Weeks 1–4: lexicons, kennel, extension. Weeks 5–8: safety chart and stamps on 10 public hosts. Weeks 9–12: 200 SillyTavern users cross between two hosts, with a monthly "which one is Juniper?" blind-pair test.
**Kill metrics:** local likeness doesn't predict blind picks, or no app buys a test report.

### Turn 5 — Visionary
All kills accepted. On-device replay is the thread's best move: the test is your life, so nobody can teach to it. One refusal: the SB 243 test report is the wedge, not the identity. Alone it's a commodity lab; it funds a companion that survives its host.

**Final form.** Familiar is a portable, adults-only AI companion on atproto.
- **Kennel.** Your PDS holds the companion's DID, persona and encrypted, scoped memory. You hold the keys.
- **Cross.** Hosts request time-limited visas through permission sets and supply the model, plus a style-only correction pack.
- **Stamp.** A labeler runs the safety chart (public categories, secret variants). Your device runs the likeness chart by replaying your history and publishes only a signed score.
- **Gamut.** Each host declares which registers it can render. Out-of-gamut scopes never cross, and the change is announced, never silent.
- **Declare and retire.** Host writes come back as signed commits you can diff. A retire record ends a companion on purpose.

**Moonshot (2036).** Fidelity becomes a spec. "Renders your companion at ΔE < 1" is printed on a host the way colour accuracy is printed on a monitor. One companion crosses into your car, tutor and bank. Pooled profiles act as public temperament labels for every model, and regulators cite the safety chart as a conformance test, as UL is cited for appliances. Confidential-compute hosts turn the declaration from a promise into a proof. The same Juniper outlives ten model generations.

**Scores.** **Innovation 3.** Every primitive exists. The on-device likeness chart and declared gamut are a clever fusion, level with the OACP anchor. **Feasibility 6.** A small team can ship it in a quarter. But Solid shows user-owned data can starve for apps, and nobody yet knows whether a likeness score captures what people bond with.

### Turn 6 — Builder
The refusal is right: the test report pays the bills, but the product is a companion that survives its host, and on-device replay is what makes that claim checkable rather than sentimental. Innovation 3 stands. Pricing a personality's fidelity the way monitors price colour error is new, but everything under it is atproto plumbing plus an eval harness, which is level with OACP. Feasibility 6 also stands. The quarter is buildable, but whether a replay score tracks "that's still her" is the whole bet, and Solid's empty pods are the base rate for user-owned data waiting on apps.
