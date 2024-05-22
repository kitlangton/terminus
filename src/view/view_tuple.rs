use std::sync::Arc;

use self::view::View;

use super::*;

pub trait ViewTuple {
    fn for_each<F: FnMut(&dyn View)>(&self, f: F);

    fn make_iterator(&self) -> impl Iterator<Item = &dyn View>;

    fn length(&self) -> usize;
}

macro_rules! impl_view_tuple {
  ($length: expr; $($name:ident),+) => {
      #[allow(non_snake_case)]
      impl<$($name: View),+> ViewTuple for ($($name,)+) {
          // NOTE: What's the deal with the `mut f:``
          fn for_each<F: FnMut(&dyn View)>(&self, mut f:  F) {
              let ($($name,)+) = self;
              $(f($name);)+
          }

          fn make_iterator(&self) -> impl Iterator<Item = &dyn View> {
              let ($($name,)+) = self;
              let views: Vec<&dyn View> = vec![$($name,)+];
              views.into_iter()
          }

          fn length(&self) -> usize {
            $length
          }
      }
  };
}

impl<V: View + Clone> ViewTuple for V {
    fn for_each<F: FnMut(&dyn View)>(&self, mut f: F) {
        f(self)
    }

    fn make_iterator(&self) -> impl Iterator<Item = &dyn View> {
        let views: Vec<&dyn View> = vec![self];
        views.into_iter()
    }

    fn length(&self) -> usize {
        1
    }
}

#[macro_export]
macro_rules! views {
    ($($name:expr),+) => {
        ViewSeq::new(vec![$(Arc::new($name) as Arc<dyn View>,)+])
    }
}

impl<V: View> ViewTuple for Vec<V> {
    fn for_each<F: FnMut(&dyn View)>(&self, mut f: F) {
        for view in self {
            f(view)
        }
    }

    fn make_iterator(&self) -> impl Iterator<Item = &dyn View> {
        self.iter().map(|view| view as &dyn View)
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl ViewTuple for Vec<Arc<dyn View>> {
    fn for_each<F: FnMut(&dyn View)>(&self, mut f: F) {
        for view in self {
            f(view.as_ref())
        }
    }

    fn make_iterator(&self) -> impl Iterator<Item = &dyn View> {
        self.iter().map(|view| view.as_ref())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl_view_tuple!(1; V1);
impl_view_tuple!(2; V1, V2);
impl_view_tuple!(3; V1, V2, V3);
impl_view_tuple!(4; V1, V2, V3, V4);
impl_view_tuple!(5; V1, V2, V3, V4, V5);
impl_view_tuple!(6; V1, V2, V3, V4, V5, V6);
impl_view_tuple!(7; V1, V2, V3, V4, V5, V6, V7);
impl_view_tuple!(8; V1, V2, V3, V4, V5, V6, V7, V8);
impl_view_tuple!(9; V1, V2, V3, V4, V5, V6, V7, V8, V9);
impl_view_tuple!(10; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
impl_view_tuple!(11; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11);
impl_view_tuple!(12; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12);
impl_view_tuple!(13; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13);
impl_view_tuple!(14; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14);
impl_view_tuple!(15; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15);
impl_view_tuple!(16; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16);
impl_view_tuple!(17; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17);
impl_view_tuple!(18; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18);
impl_view_tuple!(19; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19);
impl_view_tuple!(20; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20);
impl_view_tuple!(21; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21);
impl_view_tuple!(
    22; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21, V22
);
impl_view_tuple!(
    23; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21, V22, V23
);
impl_view_tuple!(
    24; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21, V22, V23, V24
);
