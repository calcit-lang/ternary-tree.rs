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

// https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
impl<T, const N: usize> From<&[T; N]> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn from(xs: &[T; N]) -> Self {
    let mut ys: Vec<Self> = Vec::with_capacity(xs.len());
    for x in xs {
      ys.push(Leaf(Arc::new((*x).to_owned())))
    }
    Self::rebuild_list(xs.len(), 0, &ys)
  }
}
