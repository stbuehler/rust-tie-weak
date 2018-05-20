use core::cell::UnsafeCell;
use core::fmt;
use core::ops::Deref;

use traits::{
	SharedStrongPointer,
	SharedWeakPointer,
};

/// Wrap a weak pointer for safe knot tying
///
/// Can be set later through interior mutability.  This is unsafe in the generic
/// case, but the [`tie!`] macro offers a safe abstraction.
///
/// [`tie!`]: macro.tie.html
pub struct TieWeak<T: SharedWeakPointer> {
	weak: UnsafeCell<T>,
}

impl<T: SharedWeakPointer> TieWeak<T> {
	/// Create new (empty) weak pointer
	///
	/// Can be set later by [`TieWeak::init(...)`](#method.init)
	pub fn new() -> Self {
		Default::default()
	}

	/// initializes weak pointer to a downgraded strong pointer
	///
	/// This is unsafe because the pointer might already be shared, so only use
	/// this if you know nobody is using the weak pointer yet.
	///
	/// The [`tie!`] macro provides a safe abstraction for
	/// using this.
	///
	/// [`tie!`]: macro.tie.html
	pub unsafe fn init(ptr: &Self, target: &T::Strong) {
		*ptr.weak.get() = T::Strong::downgrade(target);
	}

	/// prevent coercion, needed to make [`tie!`] macro safe
	///
	/// if a struct field `f` is not of type `TieWeak<T>` but `&s.f` would
	/// coerce to `&TieWeak<_>`, `&&s.f` does still not coerce to
	/// `&&TieWeak<_>`, which is what we need to make `tie!` safe; otherwise
	/// someone might use `Rc<TieWeak<Weak<...>>>` for the cyclic link, which
	/// would be dangerous.
	///
	/// [`tie!`]: macro.tie.html
	pub fn tag_ref(_ptr: &&Self) -> T::Tag {
		Default::default()
	}
}

impl<T: SharedWeakPointer> Default for TieWeak<T> {
	fn default() -> Self {
		TieWeak {
			weak: UnsafeCell::new(T::new()),
		}
	}
}

impl<T: SharedWeakPointer> Clone for TieWeak<T> {
	fn clone(&self) -> Self {
		TieWeak {
			weak: UnsafeCell::new(self.deref().clone()),
		}
	}
}

impl<T: SharedWeakPointer> Deref for TieWeak<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.weak.get() }
	}
}

impl<T: SharedWeakPointer + fmt::Debug> fmt::Debug for TieWeak<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SharedWeakPointer({:?})", self.deref())
	}
}

unsafe impl<T: SharedWeakPointer + Send> Send for TieWeak<T> {}
unsafe impl<T: SharedWeakPointer + Sync> Sync for TieWeak<T> {}
