// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:AU
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 OpenFin8:AU - Australian open finance: the Consumer Data Right (CDR) - the
//! sector-agnostic data-sharing regime (banking sector first;
//! energy, telco, and non-bank lending follow by legislation).
//! Data-plane only: CDR banking has no payment initiation in the
//! current standards. See
//! `DESIGN.md`.
//!
//! This service embeds `openfin8-core` (the jurisdiction-agnostic
//! kernel) and supplies the AU profile, the CDR ceremony (CDR Register-mediated
//! onboarding, PAR + PKCE + mTLS, Levels of Assurance, PPID), the
//! sharing-arrangement consent model with the 90-day
//! reauthorisation rule, and the binding consumer-experience
//! obligations (the consent dashboard, mandated wording). See
//! `DESIGN.md` §2-§5 for the wire, the ceremony, and the surface.
//!
//! Spec publication is the vendored case: the CDR
//! standards are published under the MIT Open Licence, so the
//! published `docs/openapi.json` MAY carry the CDR resource paths
//! with the copyright notice preserved (see `src/openapi.rs`).

use utoipa::OpenApi;

mod openapi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sentin8::shell::serve(
        sentin8::shell::Sentin8Shell::new("openfin8:au", "OPENFIN8_AU")
            .with_registration(print_registration_stub)
            .with_own_health()
            .with_openapi(openapi::AUOpenApi::openapi()),
    )
    .await
}

/// The registration data (policy sets, Shield gates `openfin8:au:*`
/// role-scoped, admin UI pages, SQL migrations, the schedules) - the
/// per-jurisdiction identity shape follows `openfin8-us/registration.rs`
/// (DESIGN.md §5).
fn print_registration_stub(section: Option<String>) -> anyhow::Result<()> {
    // The full registration module carries it (DESIGN.md §5).
    let _ = section;
    Ok(())
}
