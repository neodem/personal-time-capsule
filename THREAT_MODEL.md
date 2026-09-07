# Threat Model

This document lists what — and whom — the Personal Time Capsule design has to defend against, and what it explicitly does not try to defend against. It exists so that later specification and tooling decisions can be checked against a concrete list of threats rather than vague intuition. See [`PRINCIPLES.md`](./PRINCIPLES.md) for the commitments this responds to, especially #8 (preservation and access are different things) and #9 (no brittle promises).

Threats below are grouped by category, not by severity or likelihood — a capsule's actual risk profile depends heavily on what the creator puts in it and who they choose as custodian, which is exactly why the standard has to name the general threats rather than assume one particular creator's situation.

## 1. Data loss

- **Media failure** — the storage the capsule sits on fails (disk death, bit rot, cloud provider data loss).
- **Account/service closure** — a cloud provider closes the creator's account, shuts down, or changes terms in a way that strands the data.
- **Accidental deletion** — by the creator, a custodian, or a successor custodian.
- **No redundancy** — a single copy in a single location is a single point of failure regardless of any other protection.

*Mitigation direction:* LOCKSS-style "lots of copies keep stuff safe" — multiple custodians holding independently verifiable copies, not reliance on any one storage medium or provider.

## 2. Format obsolescence

- File formats, codecs, or container formats become unreadable as the software that produced them disappears.
- This is a threat to the **payload content itself**, distinct from the wrapper around it. Personal Time Capsule cannot force a creator's files into a longer-lived format than they already are — it can only avoid making the problem worse by wrapping them in something proprietary (Principle #3).

*Mitigation direction:* payload stays in its native, standard format rather than a proprietary container, preference for open/durable formats where the tooling has a choice (e.g. export formats), PREMIS-style format metadata so a future custodian at least knows what it's dealing with. Full format-migration tooling is out of scope for Phase 1.

## 3. Institutional collapse

- The custodian institution holding a capsule shuts down, merges, loses funding, or otherwise stops being able to fulfill its role.
- A successor custodian was never designated, or the designated successor has also since collapsed.
- Institutional memory of the capsule's existence or release policy is lost even if the underlying files survive (e.g. staff turnover, poor internal records).

*Mitigation direction:* an explicit succession chain of fallback custodians is part of the deposit agreement, not an afterthought; capsule metadata is self-describing enough that a *new*, previously uninvolved institution can pick it up cold.

## 4. Unauthorized early access

- Someone — a curious family member, a custodian's employee, a hacker, a future custodian who didn't inherit the original release terms — reads capsule content before its release condition is met.
- This includes technically-authorized-but-not-intended access: a custodian who can technically decrypt because of how keys were managed, even though the design intent was that they couldn't.

*Mitigation direction:* Principle #8 — preservation and access are separate. A custodian should be able to verify and replicate a capsule without holding decryption capability at all, or only holding a share of it (e.g. via key-splitting among multiple trustees) so no single party can unlock early alone. See `ENCRYPTION.md` for the actual algorithm (XChaCha20-Poly1305) and splitting scheme (SLIP-0039).

## 5. Unauthorized modification or forgery

- Tampering with payload content, metadata, or annotations after sealing — whether by an outside attacker, a custodian, or (for the human-context layer) a family member with their own agenda about how the creator should be remembered.
- Forgery: creating a fake capsule, or fake records within a real one, and passing them off as authentic.

*Mitigation direction:* cryptographic integrity manifests (SHA-256, per BagIt) generated at sealing time and independently verifiable by any custodian or future researcher without needing to trust the party presenting the capsule.

## 6. Coercion

- The creator is pressured — by a family member, an abusive partner, an employer, a state actor — into releasing a capsule early, redacting it, or handing over keys against their actual wishes.
- A custodian or key-holding trustee is legally compelled (subpoena, court order) or coerced to release early.
- Coercion aimed the other way: someone pressures a creator to *destroy* material that documents them (e.g. evidence of harm) before it can be preserved.

*Mitigation direction:* this is only partially a technical problem. Multi-party key custody (so no single coerced individual can unlock a capsule alone — `ENCRYPTION.md`'s SLIP-0039 quorum) helps, and `ACCESS.md` has that same quorum double as independent confirmation of an `after_death` trigger, not just a single executor's say-so; the rest is a legal/procedural question for Phase 3 (deed of gift, access restriction agreements) rather than something cryptography alone can solve. Documented explicitly per Principle #9 — we do not claim coercion resistance we can't actually provide.

## 7. Loss of legitimate access

- The creator dies without keys or instructions being findable.
- A sole key-holding trustee dies, becomes unreachable, or refuses to cooperate at the release trigger.
- A time-locked or service-dependent encryption scheme depends on a specific external provider that no longer exists when the release date arrives.

*Mitigation direction:* redundant, documented key custody (secret-sharing across multiple trustees with a defined quorum, per `ENCRYPTION.md`) instead of a single point of failure; no dependency on any single external service surviving decades (Principle #9) — notably, no public-key infrastructure or third-party service is part of the encryption design at all.

## 8. Privacy harm to third parties

- A capsule contains identifying or sensitive material about people *other than* the creator (correspondents, family members, people photographed) who never consented to being archived, and who may still be alive when the capsule is released.
- Sensitive material left in inadvertently — SSNs, financial account numbers, credentials, medical details — either about the creator or third parties.

*Mitigation direction:* sensitive-data scanning and redaction tooling, and privacy zones (public / restricted / never-release) as a core feature (Principle #7), not something bolted on later. This is a threat the *creator* needs help seeing, not just a threat from outside attackers. See `PRIVACY.md` for per-item overrides and a recommended (not enforced) starter vocabulary for scan categories.

## 9. Insider threat at a custodian institution

- Staff at a custodian institution with legitimate physical or system access misuse it — browsing sealed capsules, leaking content, or altering release terms.

*Mitigation direction:* same as #4 — preservation-without-access design means insider access to storage shouldn't imply insider access to content. Integrity manifests (per #5) make undetected tampering harder even for insiders.

## 10. Supply-chain / tooling compromise

- A malicious or compromised version of the reference CLI (`init`, `import`, `scan`, `redact`, `encrypt`, `verify`, `unlock`, `deposit`) is used instead of the genuine tool — e.g. to exfiltrate keys during encryption, or to silently skip redaction during scanning.

*Mitigation direction:* the spec should be simple and open enough that capsule validity doesn't depend on trusting any particular binary — a capsule that conforms to the published spec should be verifiable by independently written tooling, not just our own.

## 11. Legal and regulatory change over time

- Future law (e.g. an expanded "right to erasure," inheritance law changes, changes to what a custodian institution is legally permitted to hold) conflicts with a capsule's stated release policy or an institution's ability to honor it.

*Mitigation direction:* mostly a Phase 3 (Legal) concern — document assumptions clearly enough that custodians and successors can reason about conflicts if and when they arise, rather than pretending the policy is immutable. `ACCESS.md` is explicit that jurisdictional recognition of death, disputed cases, and similar are Phase 3 territory, not something the release-trigger record itself can resolve.

## Explicit non-goals

To keep the threat model honest, some things are deliberately **out of scope**:

- **Nation-state-level targeted attackers with unlimited resources.** The design assumes realistic threats to an individual's or family's archive, not resistance to an adversary capable of, say, compelling every custodian in a succession chain simultaneously. This is consistent with Principle #9 — we don't promise what we can't deliver.
- **Protecting a creator from their own choices about content.** If a creator chooses to include something, or chooses a release policy a future reader might disagree with, that's Principle #2 (the archive belongs to the creator) working as intended, not a threat to defend against.
- **General digital preservation research problems.** Long-term format migration, exotic media recovery, and similar are real problems the broader preservation field (which we build on — BagIt, PREMIS, OAIS, LOCKSS) is already working on. We inherit their solutions where they exist rather than re-solving them.
- **Guaranteeing perpetual existence of any specific custodian or of this project itself.** The whole point of the succession chain and the "readable without our software" commitment (Principle #3) is that no single organization, including this one, needs to survive for a capsule to remain valid.

## Status

This is a first pass, written during Phase 1 (Definition). It should be revisited once `ACTORS.md` and `LIFECYCLE.md` exist, and again once real custodians, cryptographers, and estate lawyers have had a chance to poke holes in it (Principle #10).
