/// Trait implemented for Option to allow easy checking of whether the Option has a particular Some
/// value.
///
/// Ideally a form of this functionality would be provided by the standard library. It existed as
/// Option::contains() (https://github.com/rust-lang/rust/issues/62358, which also discusses other
/// naming options) but is being dropped in 1.70 (https://github.com/rust-lang/rust/pull/108095);
/// alternatives like is_some_with (https://github.com/rust-lang/rust/pull/93051) are still
/// unstable. The use of "wraps" instead of "contains" (as in https://crates.io/crates/option-ext)
/// avoids potential collisions with the unstable function in the standard library.
pub trait Wrapper<T> {
    fn wraps<U>(&self, x: &U) -> bool where U: PartialEq<T>;
}

impl<T> Wrapper<T> for Option<T> {
    fn wraps<U>(&self, x: &U) -> bool where U: PartialEq<T> {
        match self {
            Some(y) => x == y,
            None => false,
        }
    }
}
