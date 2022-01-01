use std::any::Any;
use std::fmt;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;
use yew::callback::Callback;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Id(u64);

impl Default for Id {
    fn default() -> Self {
        static CTR: Lazy<AtomicU64> = Lazy::new(AtomicU64::default);

        Self(CTR.fetch_add(1, Ordering::SeqCst))
    }
}

impl Id {
    pub fn new() -> Self {
        Self::default()
    }
}

pub(crate) struct Listener {
    _listener: Rc<dyn Any>,
}

impl Listener {
    pub fn new(inner: Rc<dyn Any>) -> Self {
        Self { _listener: inner }
    }
}

impl fmt::Debug for Listener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Listener").finish()
    }
}

/// A trait to limit certain types to be Rc.
pub trait RcTrait {
    type Inner: 'static;

    /// Clones a `Rc<T>`.
    fn clone_rc(&self) -> Self;
}

impl<T> RcTrait for Rc<T>
where
    T: 'static,
{
    type Inner = T;

    fn clone_rc(&self) -> Rc<Self::Inner> {
        self.clone()
    }
}

pub(crate) type ListenerVec<T> = Vec<Weak<Callback<Rc<T>>>>;
