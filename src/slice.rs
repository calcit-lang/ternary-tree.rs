use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;

use crate::TernaryTreeList;
use crate::TernaryTreeList::*;

impl<T> From<Vec<T>> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn from(xs: Vec<T>) -> Self {
    let mut ys: Vec<Self> = Vec::with_capacity(xs.len());
    for x in &xs {
      ys.push(Leaf(Arc::new(x.to_owned())))
    }

    Self::rebuild_list(xs.len(), 0, &ys)
  }
}

impl<T> From<&Vec<T>> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn from(xs: &Vec<T>) -> Self {
    let mut ys: Vec<Self> = Vec::with_capacity(xs.len());
    for x in xs {
      ys.push(Leaf(Arc::new((*x).to_owned())))
    }

    Self::rebuild_list(xs.len(), 0, &ys)
  }
}

macro_rules! list_from_array {
  ($size_:expr) => {
    impl<T> From<&[T; $size_]> for TernaryTreeList<T>
    where
      T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
    {
      fn from(xs: &[T; $size_]) -> Self {
        let mut ys: Vec<Self> = Vec::with_capacity(xs.len());
        for x in xs {
          ys.push(Leaf(Arc::new((*x).to_owned())))
        }
        Self::rebuild_list(xs.len(), 0, &ys)
      }
    }
  };
}

list_from_array!(0);
list_from_array!(1);
list_from_array!(2);
list_from_array!(3);
list_from_array!(4);
list_from_array!(5);
list_from_array!(6);
list_from_array!(7);
list_from_array!(8);
list_from_array!(9);
list_from_array!(10);
list_from_array!(11);
list_from_array!(12);
list_from_array!(13);
list_from_array!(14);
