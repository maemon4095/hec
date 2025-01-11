use crate::{
    internal, never::HNever, AcceptHMap, AcceptHVisit, AcceptHVisitMut, HMap, HVisit, HVisitMut,
};

pub trait HOrClass: internal::Sealed {}

impl HOrClass for HNever {}

#[derive(Debug, Clone)]
pub enum HOr<T, L: HOrClass> {
    Item(T),
    Else(L),
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
            Self::Item(v) => {
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
            Self::Item(v) => {
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
            Self::Item(v) => HOr::Item(map.map(v)),
            Self::Else(l) => HOr::Else(l.accept_hmap(map)),
        }
    }
}
