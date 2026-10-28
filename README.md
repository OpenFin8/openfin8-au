# OpenFin8:AU - README

**Australian open finance (the Consumer Data Right): the banking-sector
data-holder surface, sharing arrangements, the consent dashboard
obligations.**

OpenFin8:AU is the Australian adapter over the openfin8-core kernel:
the CDR data plane - sector-agnostic by legislation (banking first;
energy, telco, non-bank lending follow as profile data). The
ceremony is its own family: CDR Register-mediated onboarding
(legal entity -> brand -> software product, ACCC accredited),
PAR + PKCE + mTLS, Levels of Assurance, pairwise pseudonymous
identifiers. Consent is a **sharing arrangement** with the 90-day
reauthorisation rule - and the binding consumer-experience
obligations (the data-holder consent dashboard, mandated wording,
notifications) are part of the deliverable. Data-plane only: CDR
banking has no payment initiation in the current standards.

In design (OBARCH-8): DESIGN.md covers the CDR wire, the
arrangement model, the dashboard obligations, and the Register
integration.

- [Architecture (DDD)](https://github.com/OpenFin8/openfin8-au/blob/main/DESIGN.md)
- [Business (BRD)](https://github.com/OpenFin8/openfin8-au/blob/main/BUSINESS.md)
- [Spec references](https://github.com/OpenFin8/openfin8-au/blob/main/specs/README.md)

---

**Architecture & Design:** openfin8:au and its underlying concepts were
designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).
