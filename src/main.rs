// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:AU
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 OpenFin8:AU - Australian open finance: the Consumer Data Right (CDR) - the
//! sector-agnostic data-sharing regime (banking sector first;
//! energy, telco, and non-bank lending follow by legislation).
//! Data-plane only: CDR banking has no payment initiation in the
//! current standards. See
//! `DESIGN.md` (OBARCH-8).
//!
//! This service embeds `openfin8-core` (the jurisdiction-agnostic
//! kernel - OBARCH-1) and supplies the AU profile, the CDR ceremony (CDR Register-mediated
//! onboarding, PAR + PKCE + mTLS, Levels of Assurance, PPID), the
//! sharing-arrangement consent model with the 90-day
//! reauthorisation rule, and the binding consumer-experience
//! obligations (the consent dashboard, mandated wording).
//!
//! Skeleton - the OBARCH-1 core extraction lands first; this binary
//! then takes the OB wire handlers, the intent-based consent model
//! over core's `awaiting_authorization` state, and its registration
//! data. Auth gains the FAPI 1 Advanced profile flag alongside
//! (sequenced with this adapter).
//!
//! Spec publication (OBARCH-2) is the vendored case: the CDR
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

/// Registration stub - the full registration data (policy sets,
/// Shield gates `openfin8:au:*` role-scoped, admin UI pages, SQL
/// migrations, the staged-intent expiry schedule) lands with the
/// OBARCH-1 extraction; the shape follows the `openfin8-us`
/// registration (per-jurisdiction identity).
fn print_registration_stub(section: Option<String>) -> anyhow::Result<()> {
    // Placeholder until the registration module lands (OBARCH-8).
    let _ = section;
    Ok(())
}
