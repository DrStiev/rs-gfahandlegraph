use crate::handle::Handle;

/// A unique identifier for a single path.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PathId(pub u64);

pub trait PathBase: Sized {
    type Step: Copy + Eq;
}

impl<'a, T> PathBase for &'a T
where
    T: PathBase,
{
    type Step = T::Step;
}

impl<'a, T> PathBase for &'a mut T
where
    T: PathBase,
{
    type Step = T::Step;
}

/// Abstraction of an immutable embedded path.
pub trait PathRef: Copy + PathBase {
    fn len(self) -> usize;

    fn circular(self) -> bool;

    fn first_step(self) -> Self::Step;

    fn last_step(self) -> Self::Step;
}

/// An embedded path that can also be mutated by appending or
/// prepending steps, or rewriting parts of it.
pub trait PathRefMut: PathBase {
    fn append(self, handle: Handle) -> Self::Step;

    fn prepend(self, handle: Handle) -> Self::Step;

    fn set_circularity(self, circular: bool);
}
