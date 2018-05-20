//! only needed to implement tying for other reference counted pointers

/// Type without data to create new instances of the pointer
///
/// Implement this for some tag type, e.g. PhantomData<Strong<T>> or a custom
/// type
pub unsafe trait SharedPointerTag: Default {
	/// type the shared pointers point to
	type Content;
	/// Weak pointer
	type Weak: SharedWeakPointer<
		Content = Self::Content,
		Strong = Self::Strong,
	>;
	/// Strong pointer
	type Strong: SharedStrongPointer<
		Content = Self::Content,
		Weak = Self::Weak,
	>;

	/// Create new instance through `Self::Strong::new` (don't override)
	fn new(&self, data: Self::Content) -> Self::Strong {
		Self::Strong::new(data)
	}

	/// Create empty weak pointer through `Self::Weak::new` (don't override)
	fn new_weak(&self) -> Self::Weak {
		Self::Weak::new()
	}
}

/// A strong pointer type
pub unsafe trait SharedStrongPointer: Clone {
	/// type of the content the pointer points to
	type Content;

	/// the corresponding weak pointer type
	type Weak: SharedWeakPointer<
		Content = Self::Content,
		Strong = Self,
	>;

	/// the tag type that can be used to create new instances
	type Tag: SharedPointerTag<
		Content = Self::Content,
		Strong = Self,
		Weak = Self::Weak,
	>;

	/// Create new instance; the new instance must be only accessible through
	/// the returned pointer, don't leak the reference another way.
	fn new(data: Self::Content) -> Self;

	/// Create a weak pointer from a strong reference
	fn downgrade(ptr: &Self) -> Self::Weak;
}

pub unsafe trait SharedWeakPointer: Clone {
	/// type of the content the pointer points to
	type Content;

	/// the corresponding weak pointer type
	type Strong: SharedStrongPointer<
		Content = Self::Content,
		Weak = Self,
	>;

	/// the tag type that can be used to create new instances
	type Tag: SharedPointerTag<
		Content = Self::Content,
		Strong = Self::Strong,
		Weak = Self,
	>;

	/// Create empty weak pointer
	fn new() -> Self;

	/// Try to convert back to strong reference.  Fails if data is already gone.
	fn upgrade(&self) -> Option<Self::Strong>;
}
