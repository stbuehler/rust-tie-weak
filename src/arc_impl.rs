use core::marker::PhantomData;
use std::sync::{Arc, Weak};

use traits::*;

unsafe impl<T> SharedPointerTag for PhantomData<Arc<T>> {
	type Content = T;
	type Weak = Weak<T>;
	type Strong = Arc<T>;
}

unsafe impl<T> SharedStrongPointer for Arc<T> {
	type Content = T;
	type Weak = Weak<T>;
	type Tag = PhantomData<Arc<T>>;

	fn new(data: Self::Content) -> Self {
		Arc::new(data)
	}

	fn downgrade(ptr: &Self) -> Self::Weak {
		Arc::downgrade(ptr)
	}
}

unsafe impl<T> SharedWeakPointer for Weak<T> {
	type Content = T;
	type Strong = Arc<T>;
	type Tag = PhantomData<Arc<T>>;

	fn new() -> Self {
		Weak::new()
	}

	fn upgrade(&self) -> Option<Self::Strong> {
		Weak::upgrade(self)
	}
}
