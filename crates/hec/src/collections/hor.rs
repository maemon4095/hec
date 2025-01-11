use std::fmt::Debug;

use crate::{
    internal, AcceptHMap, AcceptHSink, AcceptHVisit, AcceptHVisitMut, HMap, HSink, HVisit,
    HVisitMut,
};

pub trait HOrClass: internal::Sealed {}

impl HOrClass for HNever {}

#[derive(Clone)]
pub enum HOr<T, L: HOrClass> {
    Just(T),
    Else(L),
}

impl<T: Debug, L: Debug + HOrClass> Debug for HOr<T, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Just(v) => f.debug_tuple("HOr").field(v).finish(),
            Self::Else(v) => v.fmt(f),
        }
    }
}

impl<T, L: HOrClass> internal::Sealed for HOr<T, L> {}
impl<T, L: HOrClass> HOrClass for HOr<T, L> {}

impl<V, T, L> AcceptHVisit<V> for HOr<T, L>
where
    L: HOrClass + AcceptHVisit<V>,
    V: HVisit<T>,
{
    fn accept_hvisit(&self, mut visitor: V) {
        match self {
            Self::Just(v) => {
                visitor.visit(v);
            }
            Self::Else(e) => {
                e.accept_hvisit(visitor);
            }
        }
    }
}

impl<V, T, L> AcceptHVisitMut<V> for HOr<T, L>
where
    L: HOrClass + AcceptHVisitMut<V>,
    V: HVisitMut<T>,
{
    fn accept_hvisit_mut(&mut self, mut visitor: V) {
        match self {
            Self::Just(v) => {
                visitor.visit_mut(v);
            }
            Self::Else(e) => {
                e.accept_hvisit_mut(visitor);
            }
        }
    }
}

impl<M, T, L> AcceptHMap<M> for HOr<T, L>
where
    L: HOrClass + AcceptHMap<M>,
    L::Return: HOrClass,
    M: HMap<T>,
{
    type Return = HOr<M::Return, L::Return>;

    fn accept_hmap(self, mut map: M) -> Self::Return {
        match self {
            Self::Just(v) => HOr::Just(map.map(v)),
            Self::Else(l) => HOr::Else(l.accept_hmap(map)),
        }
    }
}

impl<T, L, S> AcceptHSink<S> for HOr<T, L>
where
    L: HOrClass + AcceptHSink<S>,
    S: HSink<T>,
{
    type Return = ();

    fn accept_hsink(self, mut sink: S) -> Self::Return {
        match self {
            HOr::Just(v) => {
                sink.sink(v);
            }
            HOr::Else(e) => {
                e.accept_hsink(sink);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HNever {}

impl internal::Sealed for HNever {}

impl<V> AcceptHVisit<V> for HNever {
    fn accept_hvisit(&self, _visitor: V) {
        unreachable!()
    }
}

impl<V> AcceptHVisitMut<V> for HNever {
    fn accept_hvisit_mut(&mut self, _visitor: V) {
        unreachable!()
    }
}

impl<M> AcceptHMap<M> for HNever {
    type Return = Self;

    fn accept_hmap(self, _map: M) -> Self::Return {
        unreachable!()
    }
}

impl<S> AcceptHSink<S> for HNever {
    type Return = ();

    fn accept_hsink(self, _sink: S) -> Self::Return {
        unreachable!()
    }
}

#[macro_export]
macro_rules! HOr {
    [$head: ty $(,$rest: ty)* $(,)*] => {
        $crate::HOr<$head, HOr!($($rest),*)>
    };
    [$(,)*] => {
        $crate::HNever
    };
}
