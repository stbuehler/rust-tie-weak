use core::marker::PhantomData;
use std::rc::{Rc, Weak};

use traits::*;

unsafe impl<T> SharedPointerTag for PhantomData<Rc<T>> {
	type Content = T;
	type Weak = Weak<T>;
	type Strong = Rc<T>;
}

unsafe impl<T> SharedStrongPointer for Rc<T> {
	type Content = T;
	type Weak = Weak<T>;
	type Tag = PhantomData<Rc<T>>;

	fn new(data: Self::Content) -> Self {
		Rc::new(data)
	}

	fn downgrade(ptr: &Self) -> Self::Weak {
		Rc::downgrade(ptr)
	}
}

unsafe impl<T> SharedWeakPointer for Weak<T> {
	type Content = T;
	type Strong = Rc<T>;
	type Tag = PhantomData<Rc<T>>;

	fn new() -> Self {
		Weak::new()
	}

	fn upgrade(&self) -> Option<Self::Strong> {
		Weak::upgrade(self)
	}
}
