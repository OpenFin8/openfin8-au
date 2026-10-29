# OpenFin8:AU - Business Requirements & Functionality

> This BRD records the business scope the Australian adapter
> delivers; section 9 lists the acceptance criteria.
>
> Audience: product owners, business analysts. For architecture read
> `README.md`; for wire-level detail read `DESIGN.md`.

## 1. Business Purpose

OpenFin8:AU is **Australian open finance**: the **Consumer Data
Right** (CDR) - the Australian Government's sector-agnostic
data-sharing regime (banking first; energy, telco, and non-bank
lending follow by legislation). It delivers:

- **The CDR data-holder surface** - the banking-sector endpoints an
  accredited data recipient consumes, per the CDR standards
  (v1.36.0 line) - with versioning (`x-v` headers) and the CDR data
  clusters.
- **Sharing arrangements done right** - a CDR consent is an
  arrangement with an id: collection/disclosure permissions,
  duration, revocation, and the **90-day reauthorisation rule**
  that keeps consumers continuously in control.
- **The consumer-experience obligations** - the data holder MUST
  host a consent dashboard (view/revoke), 90-day notifications, and
  mandated wording: binding CX standards with teeth, delivered over
  the kernel's notification + RFW machinery.
- **CDR Register-mediated onboarding** - participants register
  legal entity → brand → software product with ACCC accreditation;
  the Register is the trust anchor.
- **Sector-agnostic by construction** - banking is the first
  sector; the sector dimension is profile data, so energy/telco
  arrive without a new architecture.

**A scope honesty note**: CDR banking has **no payment initiation**
in the current standards (NPP payment work is consultation-stage) -
this is the first data-plane-only adapter.

## 2. Business Scope (What the Service Will Provide)

| # | Capability area | What the business gets |
|---|---|---|
| 1 | Banking data plane | Accounts, balances, transactions, direct debits, payees, scheduled payments, products, customer + common endpoints |
| 2 | Sector extensions | Energy, telco, non-bank lending (as profile data - banking first) |
| 3 | Arrangement lifecycle | Sharing-arrangement consent, 90-day reauthorisation, revocation (`POST /cdrArrangement` + dashboard) |
| 4 | Discovery | Outages, the well-known endpoints |
| 5 | Consumer dashboard | The data-holder dashboard obligation (view/revoke, mandated wording) |
| 6 | Notifications | 90-day notifications + the kernel delivery worker under the CDR envelope |
| 7 | Admin console | The kernel pages under the AU identity |
| 8 | Events | Family `openfin8:au:*` |
| 9 | Health | Fleet supervision |

**Not owned**: the implementation (openfin8-core), the data plane
(bank8:Ledger), ceremony hosting (Sentin8:Auth), other markets'
surfaces (profile isolation), **payment initiation** (absent from
the current CDR standards).

## 3. Business Capabilities

### 3.1 The sharing arrangement model

A CDR consent is an arrangement: enumerated permissions
(collection/disclosure), duration, revocation rights, and the
90-day reauthorisation clock. Expiry never extends silently - the
consumer reauthorizes or sharing stops. The kernel's consent
machine + intent payload carry it; revocation lands on the same
terminal-state surface every adapter shares.

### 3.2 The consumer dashboard obligation

The CX standards are binding: the data holder hosts the consent
dashboard where consumers view and revoke every arrangement, with
mandated wording and the 90-day notifications. Delivered as
consumer-facing RFW pages served to the bank's app - the
hosted-consent-screen pattern, dashboard variant.

### 3.3 Identity assurance (LoA + PPID)

The ceremony records the Level of Assurance (2/3/4) per the CDR
identity framework and uses pairwise pseudonymous identifiers per
(recipient-brand, data-holder) - raw identifiers never cross the
boundary; the LoA is enforced at reauthorisation.

## 4. Business Events (Outbound)

Family `openfin8:au:*`: arrangement lifecycle, revocation, sector
data events - consumed by recipient-side reconciliation.

## 5. Business Rules & Invariants

- Every kernel invariant holds.
- Arrangement revocation is honored on the next call AND surfaced to
  the dashboard; the 90-day window never extends silently; the LoA
  claim is recorded on the consent and enforced; PPID linkage never
  leaks raw identifiers.

## 6. Target-User Roles & Use Cases

| Role | Use case |
|---|---|
| Australian institution (data holder) | Enable `openfin8:au`; serve the CDR banking sector + the dashboard obligations |
| Accredited recipient (ADR) | Register via the CDR Register, run sharing-arrangement journeys, consume consented data |
| Consumer | Views + revokes arrangements on the dashboard; reauthorizes every 90 days; receives mandated notifications |
| ACCC / DSB conformance | The CDR conformance suite against the adapter |

## 7. KPIs / Success Criteria

- **Conformance**: the CDR conformance test suite passes.
- **CX compliance**: the binding consumer-experience obligations
  (dashboard, wording, notifications) delivered structurally.
- **Sector extensibility**: energy/telco land as profile data when
  required - no new architecture.

## 8. Dependencies & Reliance on Other Services

The kernel + platform set, plus: the **CDR Register** (participant,
brand, software-product, accreditation + the trust anchors) and the
**software-product certificates** (secrets-referenced per
environment).

## 9. Acceptance Criteria (the implementation's definition of done)

- The publication guard passes (vendored with the MIT notice).
- The arrangement model over the kernel (permissions,
  duration, 90-day reauthorisation, revocation endpoint).
- The consumer dashboard pages (view/revoke + mandated wording) +
  the 90-day notification worker.
- The CDR Register integration + the LoA/PPID ceremony green against
  a fixture Register (the UK mock-directory pattern).
- The e2e journey gains the CDR arrangement + dashboard flows.

## 10. Open Business Questions

- Where does the consumer dashboard live: bank-owned app pages (the
  ebank pattern), kernel RFW surfaces, or adapter-rendered pages?
  (Leaning: RFW pages served to the bank's app.)
- Do energy/telco sectors land as sectors on this adapter or as
  separate binaries?
- The Register in dev/e2e: a fixture Register until sandbox access
  exists?
- NPP payment initiation (consultation-stage): tracked, not
  designed - revisit when the standards publish.

---

**Architecture & Design:** openfin8:au and its underlying concepts
were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).