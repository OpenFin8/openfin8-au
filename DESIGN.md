# openfin8-au - Detailed Design Document

> Status: **PROPOSED** - the Australian open-finance adapter
> (OBARCH-8). The **Consumer Data Right** (CDR) - the Australian
> Government's sector-agnostic data-sharing regime (banking is the
> first sector; energy, telco, and non-bank lending follow), CDR
> standards v1.36.0 line. Depends on: `openfin8-core` (OBARCH-1).
> Read `openfin8-uk/DESIGN.md` for the sibling design; this
> document defines the AU deltas.

## 1. Scope & Context

**What this service is.** The Australian open-finance adapter
(`service_type: openfin8:au`, binary `openfin8-au`): the **data
holder** surface for the CDR banking sector - the endpoints an
accredited data recipient consumes - plus the recipient-side
connection surfaces, over the CDR standards.

**The wire family: CDR** - its own ceremony family. Three defining
properties:

1. **The CDR Register** mediates onboarding: participants register
   legal entities → brands → software products, each with
   accreditation by the ACCC; the Register is the trust anchor (not
   raw DCR).
2. **Sharing arrangements** - a CDR consent is an *arrangement* with
   an id (`CDR Arrangement ID`): collection/disclosure permissions
   with duration, revocation rights, and the **90-day
   reauthorisation rule**.
3. **Binding consumer-experience obligations** - the CX standards
   are normative, and the data holder MUST host a **consumer
   consent dashboard** (view/revoke), 90-day notifications, and
   mandated wording. Several obligations are consumer-facing UI, not
   API shape.

**A key scope fact**: CDR banking has **no payment initiation** -
the current standards are data-plane only (NPP payment work is
consultation-stage). The AU adapter is the first data-plane-only
adapter.

## 2. The CDR Wire (as designed, per the verified research)

| Family | Resources | Notes |
|---|---|---|
| **Banking (data plane)** | accounts, balances, transactions, direct debits, payees, scheduled payments, products, common/customer endpoints, `GET /cds-au/v{maj}/banking/...` | The data-holder surface an ADR consumes; parceled per the CDR data clusters |
| **Common** | the discovery endpoints (outages), the customer + customer-detail, the CDR arrangement endpoint (`POST /cdrArrangement` revocation) | |
| **Energy / telco** | the sector extensions | Sector-agnostic by legislation - the adapter's sector dimension is profile data (banking first) |
| **Consent/arrangement** | sharing-arrangement lifecycle, 90-day reauthorisation, revocation | Core's consents + `intent_payload` carry the arrangement + permission sets; the revocation endpoint maps to the kernel's revocation surface |
| **Admin/consumer dashboard** | the data-holder dashboard obligations | The RFW admin pattern + a consumer-facing dashboard surface (ebank-style) |

**Envelope**: CDR JSON with `_links`, the CDR error model
(`{errors: [{code, title, detail}]}`), `x-v` versioning headers (the
CDR versioning contract), the rate-limit headers, and the
`x-fapi-interaction-id` echo.

## 3. Ceremony: CDR Register + Levels of Assurance

1. **Onboarding via the CDR Register** - the adapter's provider-side
   registration reads participant/brand/software-product data from
   the Register (the directory-mediated model); the recipient plane
   holds per-provider client registrations.
2. **PAR + PKCE + mTLS** - the authorization ceremony per the CDR
   security profile; client authentication by the software-product
   certificate.
3. **Levels of Assurance (LoA 2/3/4)** - the auth ceremony must
   meet the claimed LoA per the CDR identity assurance framework;
   the consent records the LoA.
4. **PPID** - pairwise pseudonymous identifiers per
   (recipient-brand, data-holder) - no raw identifiers cross the
   boundary; the kernel's customer subject maps through the PPID.

**Consent lifecycle mapping** (core ↔ CDR):
`awaiting_authorization` → `active` (the sharing arrangement) →
`revoked` (arrangement revocation - the `POST /cdrArrangement`
endpoint and the dashboard action both land here) /
`expired` (the 90-day reauthorisation window elapsing maps to a
reauthorisation-required state). The CX obligations (notifications,
dashboard wording) are notification-worker + RFW content per the
CX standards.

## 4. Money Movement

**None in v1** - CDR banking is data-plane only. The design leaves
the payments dimension to the NPP work (consultation-stage); when
the standards land payment initiation, it arrives as a follow-on
family over bank8:Payments (the same one-executor model).

## 5. Registration & Configuration

- `service-registration.json`: service type `openfin8:au`; Shield
  gates `openfin8:au:*`.
- Env: `OPENFIN8_AU_PORT`, `OPENFIN8_AU_DATABASE_URL`,
  `OPENFIN8_AU_BASE_URL`, `OPENFIN8_AU_VERSION` (the CDR standards
  version line), `OPENFIN8_AU_REGISTER_URL` (the CDR Register),
  the software-product cert/key references, the served sector list
  (banking first). Per-env overrides via `sys.services.config`.

## 6. Spec Publication (OBARCH-2)

**Vendored**: the CDR standards are published under the **MIT Open
Licence** (Data61/DSB). This adapter MAY carry the CDR resource
paths with the copyright notice preserved. The drift guard asserts
the notice and the `/cds-au/v{maj}/{sector}/...` path shape.

## 7. Invariants (as designed)

- Every kernel invariant holds.
- **AU additions**: the arrangement revocation is honored on the
  next call AND surfaced to the consumer dashboard; the 90-day
  reauthorisation never extends silently (expiry drives the state);
  the LoA claim is recorded on the consent and enforced at
  reauthorisation; PPID linkage never leaks raw identifiers.

## 8. Open Questions

- The consumer dashboard obligation: is it a bank8-ebank page
  (bank-owned UX), a kernel RFW surface, or an adapter-rendered
  page? Leaning: RFW pages served to the bank's app (the hosted
  consent-screen pattern, consumer-facing variant).
- Do the non-bank-lending (integrated) + energy sectors land as
  sectors on this adapter (the sector dimension is profile data)
  or as separate adapter binaries per sector?
- The Register in dev/e2e: the UK mock-directory fixture pattern
  (a fixture Register + JWKS) until sandbox access exists?
- NPP payment initiation (consultation-stage): tracked, not
  designed - revisit when the standards publish.

---

**Architecture & Design:** openfin8:au and its underlying concepts
were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).