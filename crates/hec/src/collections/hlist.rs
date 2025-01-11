use crate::{
    internal, never::HNever, AcceptHMap, AcceptHVisit, AcceptHVisitMut, HMap, HVisit, HVisitMut,
};

pub trait HListClass: internal::Sealed {}

impl HListClass for HNever {}

pub struct HList<T, L: HListClass> {
    item: T,
    rest: L,
}

impl<T, L: HListClass> internal::Sealed for HList<T, L> {}
impl<T, L: HListClass> HListClass for HList<T, L> {}

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
