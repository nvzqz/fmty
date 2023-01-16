use core::{cell::UnsafeCell, panic::RefUnwindSafe};

/// Value formatted at most once.
///
/// This crate uses the approach of taking ownership of the iterator rather than
/// call `Iterator::next()` against the internally-mutable reference. This is
/// because the latter approach causes undefined behavior if `next()` also
/// references the formatter. This behavior was verified against Miri with the
/// `concat_once_cycle` test.
///
/// This type is deliberately not publicly exported to reduce documentation
/// complexity. It is only `pub` in order to use in type aliases.
///
/// ```compile_fail
/// use fmty::Once;
/// ```
///
/// ```compile_fail
/// use fmty::once::Once;
/// ```
pub struct Once<T> {
    value: UnsafeCell<Option<T>>,
}

impl<T: Clone> Clone for Once<T> {
    fn clone(&self) -> Self {
        // SAFETY: This method is not called while `value` is mutably borrowed.
        let value = unsafe { (*self.value.get()).clone() };
        Self { value: UnsafeCell::new(value) }
    }
}

// Unwinding does not allow a double-`take`.
impl<T: RefUnwindSafe> RefUnwindSafe for Once<T> {}

impl<T> Once<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { value: UnsafeCell::new(Some(value)) }
    }

    pub(crate) fn take(&self) -> Option<T> {
        // SAFETY: The mutable borrow will not last outside of this scope.
        unsafe { (*self.value.get()).take() }
    }
}
