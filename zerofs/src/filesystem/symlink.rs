use zeroutils_store::IpldStore;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A symlink to a file or directory.
#[derive(Debug)]
pub struct Symlink<S>
where
    S: IpldStore,
{
    phantom: std::marker::PhantomData<S>,
}
