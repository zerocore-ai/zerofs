use zeroutils_store::IpldStore;

use crate::filesystem::Dir;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The filesystem state machine.
pub struct FsStateMachine<S>
where
    S: IpldStore,
{
    _root: Dir<S>,
}
