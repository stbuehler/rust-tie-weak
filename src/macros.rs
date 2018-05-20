#[macro_export]
/// Allocate a shared struct and tie the knot (i.e. put a weak pointer to
/// itself in a field of type [`TieWeak`])
///
/// Call as `tie!(fieldname => data)`.
///
/// # Example
///
///     #[macro_use] extern crate tie_weak;
///     use tie_weak::TieWeak;
///     use std::rc::Weak;
///
///     struct Immutable {
///         value: i32,
///         myself: TieWeak<Weak<Immutable>>,
///     }
///
///     let v = tie!(myself => Immutable {
///         value: 42,
///         myself: Default::default(),
///     });
///     assert_eq!(v.value, 42);
///     assert_eq!(v.myself.upgrade().map(|v| v.value), Some(42));
///
/// [`TieWeak`]: struct.TieWeak.html
macro_rules! tie {
	($field:ident => $e:expr) => (
		{
			let data = $e;
			// prevent coercion to `TieWeak<_>` if field type doesn't match
			let tag = $crate::TieWeak::tag_ref(&&data.$field);
			let ptr = $crate::traits::SharedPointerTag::new(&tag, data);
			unsafe {
				$crate::TieWeak::init(&ptr.$field, &ptr);
			}
			ptr
		}
	);
}
