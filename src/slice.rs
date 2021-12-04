use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::tree::*;
use crate::TernaryTreeList;

use crate::tree::TernaryTree::*;

impl<T> From<Vec<T>> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn from(xs: Vec<T>) -> Self {
    if xs.is_empty() {
      TernaryTreeList::Empty
    } else {
      let mut ys: Vec<TernaryTree<T>> = Vec::with_capacity(xs.len());
      for x in &xs {
        ys.push(Leaf(x.to_owned()))
      }

      TernaryTreeList::Tree(TernaryTree::rebuild_list(xs.len(), 0, &ys))
    }
  }
}

impl<T> From<&Vec<T>> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn from(xs: &Vec<T>) -> Self {
    if xs.is_empty() {
      TernaryTreeList::Empty
    } else {
      let mut ys: Vec<TernaryTree<T>> = Vec::with_capacity(xs.len());
      for x in xs {
        ys.push(Leaf(x.to_owned()))
      }

      TernaryTreeList::Tree(TernaryTree::rebuild_list(xs.len(), 0, &ys))
    }
  }
}

// https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
impl<T, const N: usize> From<&[T; N]> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn from(xs: &[T; N]) -> Self {
    if xs.is_empty() {
      TernaryTreeList::Empty
    } else {
      let mut ys: Vec<TernaryTree<T>> = Vec::with_capacity(xs.len());
      for x in xs {
        ys.push(Leaf(x.to_owned()))
      }

      TernaryTreeList::Tree(TernaryTree::rebuild_list(xs.len(), 0, &ys))
    }
  }
}
