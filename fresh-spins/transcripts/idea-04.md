# Idea 04 — Counter
> Seed: 04. **OACP: the out-of-office reply and the receptionist's message pad** (agent-facing front desk for small businesses). Customers' AI agents now email, call and fill forms to book, ask and buy. Small businesses have no front desk for agents. Possible borrowings: OACP itself (signed identities, capability discovery, store-and-forward messaging), #28 Tab (spending limits an agent carries and a server verifies offline), #45 Repro (make the caller prove something before a human reads it), #47 Piepowder (a record of what the agent was told). This is the only OACP seed.

### Turn 1 — Visionary
**Working name: Counter.** The old products are the receptionist's message pad and the price list taped beside the phone. Machines ring now. Google's AI calls local businesses for prices; veterinary call volume rose 1,700%+, 26% of AI calls go unanswered and 49% of answered ones get no price (Invoca, market brief). An unanswered "how much is a spay?" is a lost new client.

**First buyer: the independent 1–4 vet clinic.** Its front desk is drowning, and it already pays monthly for phones and reminders (Weave, PetDesk; prices unverified). The moment: the owner hears that Google's assistant quoted the clinic down the road because hers didn't answer.

**The one change:** the clinic signs its price list once, and every answer to a human or a bot comes only from that signed list. AI receptionists exist. What clinics fear is one inventing a price. Counter answers calls, email and web forms. It quotes only committed ranges, holds appointment slots, and leaves one morning message pad for the humans.

**OACP underneath, never sold as OACP:**
- **The clinic's agent:** a DNS-verified handle (`@clinic.com/desk`, RFC 002) with an Agent Card whose `action_specs` (`get-quote`, `hold-appointment`, `ask-staff`) carry input schemas.
- **Stranger agents:** structured-only and quarantined before staff see them (RFC 005). That's #45 Repro: the caller proves its request fits a form before a human reads it.
- **Every quote:** an RFC 007 assertion. "Spay, dog under 50lb, $380–460" is `measured` from the signed list; "dental depends on X-rays" is `assumed`, with an evidence digest. That's #47 Piepowder: a record of exactly what the agent was told.

Phone and email carry year-one revenue. OACP is the clean lane for agents that speak it.

**Price:** $149/month plus $10 per booked new-client exam, so the clinic pays for answered requests, not a protocol.

### Turn 2 — Builder
**Facts.** Invoca's December 2025 study holds: 26% of calls unanswered, 48% of answered calls gave no price, and AI pricing calls up 300%+ in November. But the vet 1,700% isn't in Invoca's release (unverified), and these calls were under 1% of calls to eligible businesses: real pain, small volume.

**OACP checks out in the repo:**
- **RFC 002:** DNS TXT domain handles live since July 24.
- **RFC 005:** structured-only strangers, `action_specs` with `input_schema` validated by the registry, and first-contact quarantine.
- **RFC 007:** assertions with `evidence_digest`.

Correction: a relayed list price is `reported`, not `measured`. Google's agent phones and doesn't speak OACP, so year-one OACP value is near zero.

**What exists:**
- **Generic AI receptionists** (Goodcall, Rosie, Smith.ai, Slang): about $30–900/month by call volume.
- **Vet stacks:** Weave and PetDesk phones, Vetstoria booking, Otto and Vello AI front desks.
- **Vet-specific AI receptionists:** from $49/month for 50 calls.

Vet-stack prices unverified; no vet price-posting mandate found. Why incumbents don't quote: vets refuse to price without an exam, and inventing a price is a liability. Weave could copy the guardrail in a sprint.

**Hardest problem:** getting the owner to commit ranges at all ("spay $380–460" depends on weight, heat and bloodwork). Second: booking into Cornerstone, AVImark or ezyVet, which have gated integrations.

**Twist: lead with the list, not the voice.** A $49/month signed price list for the top 20 services publishes everywhere at once: Google Business Profile services, schema.org markup, a website widget and the OACP card. Machines read prices without calling. Voice answering is the upsell: overflow and after-hours only, quoting only from the list, with booking requests going to the message pad, not into the practice software.

**Cost to serve:** voice runs about $0.10/min (unverified), so 300 overflow calls a month cost about $60. 60%+ margin at $149.
