use async_once_cell::OnceCell;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A link representing an association between an identifier and some lazily loaded value.
pub struct Link<L, T> {
    /// The identifier of the link.
    pub(crate) identifier: L,

    /// The cached value of the link.
    pub(crate) cached: Cached<T>,
}

/// A type alias for `OnceCell` holding a lazily initialized value.
pub type Cached<T> = OnceCell<T>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<L, T> Link<L, T> {
    /// Gets the cached value.
    pub fn cached(&self) -> Option<&T> {
        self.cached.get()
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<L, T> PartialEq for Link<L, T>
where
    L: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}
