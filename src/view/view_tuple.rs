use self::view::View;
use super::*;
use std::hash::{DefaultHasher, Hash, Hasher};

pub trait ViewTuple {
    fn make_iterator(&self) -> impl Iterator<Item = IdentifiedView<&dyn View>>;
    fn length(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct ForEachView<V> {
    values: Vec<IdentifiedView<V>>,
}

impl<V: View> ViewTuple for ForEachView<V> {
    fn make_iterator(&self) -> impl Iterator<Item = IdentifiedView<&dyn View>> {
        self.values.iter().map(|value| IdentifiedView {
            id: value.id,
            value: &value.value as &dyn View,
        })
    }

    fn length(&self) -> usize {
        self.values.len()
    }
}

fn do_hash<A: Hash>(value: &A) -> u64 {
    let default_hasher = DefaultHasher::new();
    let mut hasher = default_hasher;
    value.hash(&mut hasher);
    hasher.finish()
}

pub fn for_each_view<Values, A, V: View, F>(values: Values, func: F) -> ForEachView<V>
where
    Values: IntoIterator<Item = A>,
    A: Hash,
    F: Fn(A) -> V,
{
    ForEachView {
        values: values
            .into_iter()
            .map(|value| IdentifiedView {
                id: do_hash(&value),
                value: func(value),
            })
            .collect(),
    }
}

impl<V> ViewTuple for Vec<IdentifiedView<V>>
where
    V: View,
{
    fn make_iterator(&self) -> impl Iterator<Item = IdentifiedView<&dyn View>> {
        self.into_iter().map(|value| IdentifiedView {
            id: value.id,
            value: &value.value as &dyn View,
        })
    }

    fn length(&self) -> usize {
        self.len()
    }
}

macro_rules! impl_view_tuple {
    ($length: expr; $($name:ident, $id:expr),+) => {
        #[allow(non_snake_case)]
        impl<$($name: View),+> ViewTuple for ($($name,)+) {

            fn make_iterator(&self) -> impl Iterator<Item = IdentifiedView<&dyn View>> {
                let ($($name,)+) = self;
                let views: Vec<IdentifiedView<&dyn View>> = vec![
                    $(IdentifiedView { id: $id, value: $name }),+
                ];
                views.into_iter()
            }

            fn length(&self) -> usize {
                $length
            }
        }
    };
}

impl_view_tuple!(1; V1, 1);
impl_view_tuple!(2; V1, 1, V2, 2);
impl_view_tuple!(3; V1, 1, V2, 2, V3, 3);
impl_view_tuple!(4; V1, 1, V2, 2, V3, 3, V4, 4);
impl_view_tuple!(5; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5);
impl_view_tuple!(6; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6);
impl_view_tuple!(7; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7);
impl_view_tuple!(8; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8);
impl_view_tuple!(9; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9);
impl_view_tuple!(10; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10);
impl_view_tuple!(11; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11);
impl_view_tuple!(12; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12);
impl_view_tuple!(13; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13);
impl_view_tuple!(14; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14);
impl_view_tuple!(15; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15);
impl_view_tuple!(16; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16);
impl_view_tuple!(17; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17);
impl_view_tuple!(18; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18);
impl_view_tuple!(19; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18, V19, 19);
impl_view_tuple!(20; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18, V19, 19, V20, 20);
impl_view_tuple!(21; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18, V19, 19, V20, 20, V21, 21);
impl_view_tuple!(
    22; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18, V19, 19, V20, 20, V21, 21, V22, 22
);
impl_view_tuple!(
    23; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18, V19, 19, V20, 20, V21, 21, V22, 22, V23, 23
);
impl_view_tuple!(
    24; V1, 1, V2, 2, V3, 3, V4, 4, V5, 5, V6, 6, V7, 7, V8, 8, V9, 9, V10, 10, V11, 11, V12, 12, V13, 13, V14, 14, V15, 15, V16, 16, V17, 17, V18, 18, V19, 19, V20, 20, V21, 21, V22, 22, V23, 23, V24, 24
);
