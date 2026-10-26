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
