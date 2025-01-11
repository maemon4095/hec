use std::fmt::Debug;

use super::{DebugHList, HList, HListClass};
use crate::{internal, AcceptHMap, AcceptHSink, AcceptHVisit, AcceptHVisitMut};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HNil;

impl internal::Sealed for HNil {}

impl HListClass for HNil {}

impl DebugHList for HNil {
    fn debug_items(&self, _f: &mut std::fmt::DebugList<'_, '_>) {}
}

impl Debug for HNil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().finish()
    }
}

impl HNil {
    pub fn push<T>(self, item: T) -> HList<T, Self> {
        HList::new(item, self)
    }
}

impl<V> AcceptHVisit<V> for HNil {
    fn accept_hvisit(&self, _visitor: V) {}
}

impl<V> AcceptHVisitMut<V> for HNil {
    fn accept_hvisit_mut(&mut self, _visitor: V) {}
}

impl<M> AcceptHMap<M> for HNil {
    type Return = Self;

    fn accept_hmap(self, _map: M) -> Self::Return {
        self
    }
}

impl<S> AcceptHSink<S> for HNil {
    type Return = HNil;

    fn accept_hsink(self, _sink: S) -> Self::Return {
        self
    }
}
