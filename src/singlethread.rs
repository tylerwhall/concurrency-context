use core::cell::{Ref, RefCell, RefMut};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use super::STC;

/// A context-aware wrapper around RefCell that can be accessed in single-thread context.
///
/// Single-thread context is assumed to have no concurrency. This means the struct shall only be
/// accessed from a single thread which also excludes interrupts and signal handlers.
///
/// The intended use is to provide mutable access to static global data in a single-threaded
/// application or during early boot of an OS before interrupts and SMP are enabled.
///
/// Borrowing the underlying data requires a borrow on a type implementing STC (single-thread
/// context). This unsafe marker trait should be constructed and destroyed such that it marks the
/// beginning and end of a single-threaded context, unburdening the interlying code from requiring
/// unsafe code for each mutable static accesses. This will catch data races that would be caused,
/// for example, by starting a thread earlier in the program
///
/// # Example
/// ```
/// #![feature(const_fn)]
/// use concurrency_context::SingleThreadRefCell;
/// static G_INT: SingleThreadRefCell<i32> = SingleThreadRefCell::new(5);
///
/// // Create the context
/// let ctx = unsafe { concurrency_context::Init::new() };
/// {
///     let g = G_INT.borrow(&ctx);
///     assert_eq!(*g, 5);
/// }
/// {
///     let mut g = G_INT.borrow_mut(&ctx);
///     *g = 6;
///     assert_eq!(*g, 6);
/// }
/// {
///     let g = G_INT.borrow(&ctx);
///     assert_eq!(*g, 6);
/// }
/// ```
pub struct SingleThreadRefCell<T> {
    value: RefCell<T>
}

unsafe impl<T> Sync for SingleThreadRefCell<T> {}

pub struct SingleThreadRef<'a, 'b, T: 'a, C: STC + 'b> {
    value: Ref<'a, T>,
    _context: PhantomData<&'b C>,
}

impl<'a, 'b, T: 'a, C: STC + 'b> Deref for SingleThreadRef<'a, 'b, T, C> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value.deref()
    }
}

pub struct SingleThreadRefMut<'a, 'b, T: 'a, C: STC + 'b> {
    value: RefMut<'a, T>,
    _context: PhantomData<&'b C>,
}

impl<'a, 'b, T: 'a, C: STC + 'b> Deref for SingleThreadRefMut<'a, 'b, T, C> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value.deref()
    }
}

impl<'a, 'b, T: 'a, C: STC + 'b> DerefMut for SingleThreadRefMut<'a, 'b, T, C> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.value.deref_mut()
    }
}

impl<T> SingleThreadRefCell<T> {
    #[inline]
    pub const fn new(value: T) -> SingleThreadRefCell<T> {
        SingleThreadRefCell {
            value: RefCell::new(value)
        }
    }

    #[inline]
    pub fn borrow<'a, 'b, C: STC + 'b>(&'a self, _context: &'b C) -> SingleThreadRef<'a, 'b, T, C> {
        SingleThreadRef {
            value: self.value.borrow(),
            _context: PhantomData,
        }
    }

    #[inline]
    pub fn borrow_mut<'a, 'b, C: STC + 'b>(&'a self, _context: &'b C) -> SingleThreadRefMut<'a, 'b, T, C> {
        SingleThreadRefMut {
            value: self.value.borrow_mut(),
            _context: PhantomData,
        }
    }
}

#[test]
fn test_zero_size() {
    use core::mem;
    static G_INT: SingleThreadRefCell<i32> = SingleThreadRefCell::new(5);

    let ctx = unsafe { ::Init::new() };

    assert_eq!(mem::size_of_val(&G_INT.value), mem::size_of_val(&G_INT));
    let borrow = G_INT.borrow(&ctx);
    assert_eq!(mem::size_of_val(&borrow), mem::size_of_val(&borrow.value));
}
