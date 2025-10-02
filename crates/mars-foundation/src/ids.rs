use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU64;
use core::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Copy, Clone)]
pub struct Id<T> {
    raw: NonZeroU64,
    _pd: PhantomData<T>,
}

impl<T> Id<T> {
    #[inline]
    pub fn new() -> Self {
        let v = NEXT_ID.fetch_add(1, Ordering::Relaxed).saturating_add(1);
        let raw = NonZeroU64::new(v).unwrap();
        Self { raw, _pd: PhantomData }
    }

    #[inline] pub fn from_raw(raw: NonZeroU64) -> Self { Self { raw, _pd: PhantomData } }
    #[inline] pub fn raw(self) -> NonZeroU64 { self.raw }
}

impl<T> PartialEq for Id<T> {
    #[inline] fn eq(&self, other: &Self) -> bool { self.raw == other.raw }
}
impl<T> Eq for Id<T> {}
impl<T> Hash for Id<T> {
    #[inline] fn hash<H: Hasher>(&self, state: &mut H) { self.raw.hash(state) }
}
impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&self.raw).finish()
    }
}

pub type Handle<T> = Id<T>;

#[cfg(test)]
mod tests {
    use super::*;
    struct Foo;
    struct Bar;

    #[test]
    fn ids_are_typed_and_unique() {
        let a: Id<Foo> = Id::new();
        let b: Id<Foo> = Id::new();
        assert_ne!(a, b);

        let x: Id<Bar> = Id::new();
        assert_ne!(a.raw(), x.raw());
    }
}