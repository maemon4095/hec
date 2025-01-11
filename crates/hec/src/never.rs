use crate::{internal, AcceptHMap, AcceptHVisit, AcceptHVisitMut};

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
