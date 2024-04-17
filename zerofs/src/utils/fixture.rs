use ucan::{builder::UcanBuilder, Ucan};

use crate::Ed25516KeyPair;

//--------------------------------------------------------------------------------------------------
// Function
//--------------------------------------------------------------------------------------------------

pub async fn root_cap() -> Ucan {
    let audience_did = "did:key:z6Mkq";
    let issuer_key = Ed25516KeyPair::generate(None);

    UcanBuilder::default()
        .issued_by(&issuer_key)
        .for_audience(audience_did)
        .with_lifetime(60)
        .build()
        .unwrap()
        .sign()
        .await
        .unwrap()
}
