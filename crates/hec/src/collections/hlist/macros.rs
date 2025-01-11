#[macro_export]
macro_rules! hlist {
    [$($item: expr),* $(,)*] => {
        hlist!(@impl $($item),*)
    };

    (@impl $head: expr $(,$rest: expr)*) => {
        $crate::HList::new($head, hlist!($($rest),*))
    };

    (@impl) => {
        $crate::HNil
    };
}

pub use hlist;
