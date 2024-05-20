use std::sync::Arc;

use self::view::View;

use super::*;

pub trait ViewTuple {
    fn for_each<F: FnMut(&dyn View)>(&self, f: F);
}

macro_rules! impl_view_tuple {
  ($($name:ident),+) => {
      #[allow(non_snake_case)]
      impl<$($name: View),+> ViewTuple for ($($name,)+) {
          // NOTE: What's the deal with the `mut f:``
          fn for_each<F: FnMut(&dyn View)>(&self, mut f:  F) {
              let ($($name,)+) = self;
              $(f($name);)+
          }
      }
  };
}

impl<V: View + Clone> ViewTuple for V {
    fn for_each<F: FnMut(&dyn View)>(&self, mut f: F) {
        f(self)
    }
}

#[macro_export]
macro_rules! views {
    ($($name:expr),+) => {
        ViewSeq::new(vec![$(Arc::new($name) as Arc<dyn View>,)+])
    }
}

#[derive(Clone)]
pub struct Views<V> {
    views: Vec<V>,
}

impl<V> Views<V> {
    pub fn new(views: Vec<V>) -> Self {
        Self { views }
    }
}

impl<V: View> ViewTuple for Views<V> {
    fn for_each<F: FnMut(&dyn View)>(&self, mut f: F) {
        for view in &self.views {
            f(view)
        }
    }
}

#[derive(Clone)]
pub struct ViewSeq {
    views: Vec<Arc<dyn View>>,
}

impl ViewSeq {
    pub fn new(views: Vec<Arc<dyn View>>) -> Self {
        Self { views }
    }
}

impl ViewTuple for ViewSeq {
    fn for_each<F: FnMut(&dyn View)>(&self, mut f: F) {
        for view in &self.views {
            f(view.as_ref())
        }
    }
}

impl_view_tuple!(V1);
impl_view_tuple!(V1, V2);
impl_view_tuple!(V1, V2, V3);
impl_view_tuple!(V1, V2, V3, V4);
impl_view_tuple!(V1, V2, V3, V4, V5);
impl_view_tuple!(V1, V2, V3, V4, V5, V6);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17);
impl_view_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18);
impl_view_tuple!(
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19
);
impl_view_tuple!(
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20
);
impl_view_tuple!(
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21
);
impl_view_tuple!(
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21,
    V22
);
impl_view_tuple!(
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21,
    V22, V23
);
impl_view_tuple!(
    V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21,
    V22, V23, V24
);
