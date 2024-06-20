use std::time::{Duration, SystemTime};

use zeroutils_did_wk::{Base, KeyEncode, WrappedDidWebKey};
use zeroutils_key::{GetPublicKey, JwsAlgName, Sign};
use zeroutils_store::IpldStore;
use zeroutils_ucan::{caps, SignedUcan, Ucan};

use crate::filesystem::FsResult;

//--------------------------------------------------------------------------------------------------
// Function
//--------------------------------------------------------------------------------------------------

pub fn mock_ucan<'a, K, S>(issuer_key: &'a K, store: S) -> FsResult<SignedUcan<'a, S>>
where
    K: GetPublicKey + Sign + JwsAlgName,
    K::PublicKey<'a>: KeyEncode,
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

    Ok(ucan)
}
