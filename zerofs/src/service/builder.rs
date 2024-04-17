use crate::BlockStore;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A builder for constructing a `ZerofsService`.
pub struct ZerofsServiceBuilder<S>
where
    S: BlockStore,
{
    _store: S,
}
