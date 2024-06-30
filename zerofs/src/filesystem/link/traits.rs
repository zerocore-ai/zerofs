use std::future::Future;

use zeroutils_store::IpldStore;

use crate::filesystem::FsResult;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A trait for types that can be resolved to a target.
pub trait Resolvable<'a, S>
where
    S: IpldStore,
{
    /// The target type that the resolvable type can be resolved to.
    type Target: 'a;

    /// Resolves the resolvable type to the target.
    fn resolve(&'a self, store: S) -> impl Future<Output = FsResult<&'a Self::Target>>;
}
