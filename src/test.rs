mod sync {
	use std::sync::Weak;
	use TieWeak;

	struct Immutable {
		value: i32,
		myself: TieWeak<Weak<Immutable>>,
	}

	#[test]
	fn test1() {
		let v = tie!(myself => Immutable {
			value: 42,
			myself: Default::default(),
		});
		assert_eq!(v.value, 42);
		assert_eq!(v.myself.upgrade().map(|v| v.value), Some(42));
	}
}

mod unsync {
	use std::rc::Weak;
	use TieWeak;

	struct Immutable {
		value: i32,
		myself: TieWeak<Weak<Immutable>>,
	}

	#[test]
	fn test1() {
		let v = tie!(myself => Immutable {
			value: 42,
			myself: Default::default(),
		});
		assert_eq!(v.value, 42);
		assert_eq!(v.myself.upgrade().map(|v| v.value), Some(42));
	}
}

// this must not compile, otherwise `tie!` is not safe
#[cfg(feature = "test_break")]
mod unsync_broken {
	use std::rc::{Rc, Weak};
	use TieWeak;

	struct Immutable {
		value: i32,
		myself: Rc<TieWeak<Weak<Immutable>>>,
	}

	#[test]
	fn test1() {
		let w = Rc::new(TieWeak::new());
		let v1 = tie!(myself => Immutable {
			value: 23,
			myself: Rc::clone(&w),
		});
		let v2 = tie!(myself => Immutable {
			value: 42,
			myself: Rc::clone(&w),
		});
		assert_eq!(v2.value, 23);
		assert_eq!(v2.myself.upgrade().map(|v| v.value), Some(23));
		assert_eq!(v1.value, 42);
		assert_eq!(v1.myself.upgrade().map(|v| v.value), Some(42));
	}
}
