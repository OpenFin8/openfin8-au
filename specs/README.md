# OpenFin8:AU - Upstream Specification References

**The CDR standards may be vendored - with the notice.**

The Consumer Data Right standards are published by the Data
Standards Body (Data61) under the **MIT Open Licence**
(<https://github.com/ConsumerDataStandardsAustralia/standards>).
This repository MAY include CDR material (the resource paths in the
published `docs/openapi.json`) with the copyright notice preserved -
the CI drift guard enforces the notice is present.

Optional local reference copies, gitignored:

```
specs/cdr/{version}/       # your local copy of the CDR swagger definitions
```

Operators download from the CDR standards repository and point
conformance tooling at them. Nothing in this service reads them at
runtime - the served surface is configured via `OPENFIN8_AU_VERSION`
(sector: banking first; energy/telco are profile data).

Contrast: the FDX (US/CA) adapters must NEVER publish the licensed
FDX specification - see their repositories.
