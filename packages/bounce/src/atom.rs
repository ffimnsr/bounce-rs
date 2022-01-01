use std::any::Any;
use std::rc::Rc;

use crate::slice::Slice;

pub use bounce_macros::Atom;

#[doc(hidden)]
pub trait Atom: PartialEq + Default {
    #[allow(unused_variables)]
    fn apply(self: Rc<Self>, notion: Rc<dyn Any>) -> Rc<Self> {
        self
    }
}

impl<T> Slice for T
where
    T: Atom,
{
    type Action = T;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }

    fn apply(self: Rc<Self>, notion: Rc<dyn Any>) -> Rc<Self> {
        Atom::apply(self, notion)
    }
}

/// A trait to provide cloning on atoms.
///
/// This trait provides a `self.clone_atom()` method that can be used as an alias of `(*self).clone()`
/// in apply functions to produce a owned clone of the atom.
pub trait CloneAtom: Atom + Clone {
    /// Clones current atom.
    #[inline]
    fn clone_atom(&self) -> Self {
        self.clone()
    }
}

impl<T> CloneAtom for T where T: Atom + Clone {}