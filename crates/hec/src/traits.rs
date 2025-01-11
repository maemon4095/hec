pub trait HVisit<T: ?Sized> {
    fn visit(&mut self, value: &T);
}

pub trait HVisitMut<T: ?Sized> {
    fn visit_mut(&mut self, value: &mut T);
}

pub trait HMap<T> {
    type Return;

    fn map(&mut self, value: T) -> Self::Return;
}

pub trait AcceptHVisit<V> {
    fn accept_hvisit(&self, visitor: V);
}

pub trait AcceptHVisitMut<V> {
    fn accept_hvisit_mut(&mut self, visitor: V);
}

pub trait AcceptHMap<M> {
    type Return;
    fn accept_hmap(self, map: M) -> Self::Return;
}
