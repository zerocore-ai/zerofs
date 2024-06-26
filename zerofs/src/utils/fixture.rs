use std::time::{Duration, SystemTime};

use zeroutils_did_wk::{Base, WrappedDidWebKey};
use zeroutils_key::{GetPublicKey, IntoOwned, JwsAlgName, Sign};
use zeroutils_store::IpldStore;
use zeroutils_ucan::{caps, Ucan, UcanAuth};

use crate::filesystem::FsResult;

//--------------------------------------------------------------------------------------------------
// Function
//--------------------------------------------------------------------------------------------------

pub fn mock_ucan_auth<'a, K, S>(
    issuer_key: &'a K,
    store: S,
) -> FsResult<UcanAuth<'a, S, K::OwnedPublicKey>>
where
    K: GetPublicKey + Sign + JwsAlgName,
    S: IpldStore,
{
    let issuer_did = WrappedDidWebKey::from_key(issuer_key, Base::Base58Btc)?;
    let ucan = Ucan::builder()
        .issuer(issuer_did)
        .audience("did:wk:z6MkhjKAZ8a3bzDRE95wWERcVL2Jvo6yY58enNduuWbUYGvG")
        .expiration(Some(SystemTime::now() + Duration::from_secs(60)))
        .capabilities(caps!()?)
        .store(store)
        .sign(issuer_key)?;

    let owned_pk = issuer_key.public_key().into_owned();
    let auth = UcanAuth::new(ucan, owned_pk);

    Ok(auth)
}
