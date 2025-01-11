mod list;
mod macros;
mod nil;

use std::fmt::DebugList;

pub use list::HList;
pub use macros::hlist;
pub use nil::HNil;

pub trait HListClass: crate::internal::Sealed {}

trait DebugHList {
    fn debug_items(&self, f: &mut DebugList<'_, '_>);
}
