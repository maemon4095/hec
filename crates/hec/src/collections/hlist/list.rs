use super::{DebugHList, HListClass};
use crate::{
    internal, AcceptHMap, AcceptHSink, AcceptHVisit, AcceptHVisitMut, HMap, HSink, HVisit,
    HVisitMut,
};
use std::fmt::{Debug, DebugList};

pub struct HList<T, L: HListClass> {
    item: T,
    rest: L,
}

impl<T, L: HListClass> internal::Sealed for HList<T, L> {}
impl<T, L: HListClass> HListClass for HList<T, L> {}

impl<T: Debug, L: DebugHList + HListClass> DebugHList for HList<T, L> {
    fn debug_items(&self, f: &mut DebugList<'_, '_>) {
        f.entry(&self.item);
        self.rest.debug_items(f);
    }
}

impl<T: Debug, L: DebugHList + HListClass> Debug for HList<T, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut l = f.debug_list();
        self.debug_items(&mut l);
        l.finish()
    }
}

impl<T, L: HListClass> HList<T, L> {
    pub fn new(item: T, rest: L) -> Self {
        Self { item, rest }
    }

    pub fn push<U>(self, item: U) -> HList<U, Self> {
        HList::new(item, self)
    }

    pub fn pop(self) -> (T, L) {
        (self.item, self.rest)
    }

    pub fn pop_to<S: HSink<T>>(self, mut sink: S) -> L {
        sink.sink(self.item);
        self.rest
    }
}

impl<T, L, V> AcceptHVisit<V> for HList<T, L>
where
    L: HListClass + AcceptHVisit<V>,
    V: HVisit<T>,
{
    fn accept_hvisit(&self, mut visitor: V) {
        visitor.visit(&self.item);
        self.rest.accept_hvisit(visitor);
    }
}

impl<T, L, V> AcceptHVisitMut<V> for HList<T, L>
where
    L: HListClass + AcceptHVisitMut<V>,
    V: HVisitMut<T>,
{
    fn accept_hvisit_mut(&mut self, mut visitor: V) {
        visitor.visit_mut(&mut self.item);
        self.rest.accept_hvisit_mut(visitor);
    }
}

impl<T, L, M> AcceptHMap<M> for HList<T, L>
where
    L::Return: HListClass,
    L: HListClass + AcceptHMap<M>,
    M: HMap<T>,
{
    type Return = HList<M::Return, L::Return>;

    fn accept_hmap(self, mut map: M) -> Self::Return {
        let item = map.map(self.item);
        let rest = self.rest.accept_hmap(map);
        HList { item, rest }
    }
}

impl<T, L, S> AcceptHSink<S> for HList<T, L>
where
    L::Return: HListClass,
    L: HListClass + AcceptHSink<S>,
    S: HSink<T>,
{
    type Return = L::Return;

    fn accept_hsink(self, mut sink: S) -> Self::Return {
        sink.sink(self.item);
        self.rest.accept_hsink(sink)
    }
}
