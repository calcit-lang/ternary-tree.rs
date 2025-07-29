//! A variant of 2-3 tree, with enhancements on ternary branching, optimized with tricks like finger-tree.
//! `t.push_right(..)` is optimized to be amortized `O(1)` at best cases and `O(log n)` when restructuring involed.
//!
//! ![](https://pbs.twimg.com/media/FRc3gB7aQAA1pBb?format=jpg&name=4096x4096)
//!
//! it is also interesting to display it with triples:
//!
//! ```text
//! (((0 1 2) (3 4 5) (6 7 8)) ((9 10 11) (12 13 14) (15 16 17)) (18 19 _))
//! ```
//!
//! or with more holes:
//!
//! ```text
//! (((0 1 _) (2 3 4) (5 6 _)) ((7 8 _) (9 10 _) (11 12 _)) ((13 14 _) (15 16 17) (18 19 _)))
//! ```

mod tree;
mod util;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::sync::Arc;

use tree::TernaryTree::{self, *};

/// wraps TerarnaryTreeList with support for empty
#[derive(Clone, Debug)]
pub enum TernaryTreeList<T> {
  Empty,
  Tree(TernaryTree<T>),
}

use TernaryTreeList::*;

impl<T> TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  pub fn is_empty(&self) -> bool {
    match self {
      Empty => true,
      Tree(_) => false,
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Empty => 0,
      Tree(t) => t.len(),
    }
  }

  /// turn into a representation in triples, `_` for holes
  pub fn format_inline(&self) -> String {
    match self {
      Empty => String::from("_"),
      Tree(t) => t.format_inline(),
    }
  }

  /// items in debug display
  pub fn format_debug(&self) -> String {
    let mut s = String::from("(TernaryTreeList debug");
    for x in self.iter() {
      s.push_str(&format!(" {:?}", x));
    }
    s.push(')');
    s
  }

  /// get element in list by reference
  /// PERF: recursive function is slower than iterative loop with Cell in bench(using `usize`),
  /// however, Calcit is heavy in cloning(reference though... according real practice),
  /// so here we still choose `ref_get` for speed in Calcit project.
  pub fn get(&self, idx: usize) -> Option<&T> {
    let l = self.len();
    if l == 0 || idx >= l {
      None
    } else if idx == 0 {
      match self {
        Empty => None,
        Tree(t) => Some(t.loop_first()),
      }
    } else if idx == l - 1 {
      match self {
        Empty => None,
        Tree(t) => Some(t.loop_last()),
      }
    } else {
      self.loop_get(idx)
    }
  }

  /// find position of matched element in list(if exists)
  pub fn find_index(&self, f: Arc<dyn Fn(&T) -> bool>) -> Option<i64> {
    match self {
      Empty => None,
      Tree(t) => t.find_index(f),
    }
  }

  /// find position of element
  pub fn index_of(&self, item: &T) -> Option<usize> {
    match self {
      Empty => None,
      Tree(t) => t.index_of(item),
    }
  }

  /// index of element from end, return 0 if found at last
  pub fn last_index_of(&self, item: &T) -> Option<usize> {
    match self {
      Empty => None,
      Tree(t) => t.last_index_of(item),
    }
  }

  /// recursively check structure
  pub fn eq_shape(&self, ys: &Self) -> bool {
    match (self, ys) {
      (Empty, Empty) => true,
      (Empty, _) => false,
      (_, Empty) => false,
      (Tree(x), Tree(y)) => x.eq_shape(y),
    }
  }

  /// unchecked get reference of element
  pub fn ref_get(&self, idx: usize) -> Option<&T> {
    match self {
      Empty => None,
      Tree(t) => Some(t.ref_get(idx)),
    }
  }

  /// unchecked get via go down the branch with a mutable loop
  /// this function is SLOWER compared to `ref_get`, not used by default
  pub fn loop_get(&self, original_idx: usize) -> Option<&T> {
    match self {
      Empty => None,
      Tree(t) => Some(t.loop_get(original_idx)),
    }
  }

  pub fn first(&self) -> Option<&T> {
    match self {
      Empty => None,
      Tree(t) => t.first(),
    }
  }

  pub fn last(&self) -> Option<&T> {
    match self {
      Empty => None,
      Tree(t) => t.last(),
    }
  }

  // at known index, update value
  pub fn assoc(&self, idx: usize, item: T) -> Result<Self, String> {
    match self {
      Empty => Err(String::from("empty")),
      Tree(t) => {
        if idx > self.len() - 1 {
          Err(format!("Index too large {} for {}", idx, self.format_inline()))
        } else {
          Ok(TernaryTreeList::Tree(t.assoc(idx, item)?))
        }
      }
    }
  }
  pub fn dissoc(&self, idx: usize) -> Result<Self, String> {
    match self {
      Empty => Err(String::from("calling dissoc from empty")),
      Tree(t) => {
        if t.len() == 1 {
          if idx == 0 {
            Ok(Empty)
          } else {
            Err(format!("Index too large {} for {}", idx, self.format_inline()))
          }
        } else if idx < t.len() {
          Ok(TernaryTreeList::Tree(t.dissoc(idx)?))
        } else {
          Err(format!("Index too large {} for {}", idx, self.format_inline()))
        }
      }
    }
  }
  /// ternary tree operation of rest
  pub fn rest(&self) -> Result<Self, String> {
    let size = self.len();
    if size == 0 {
      Err(String::from("calling rest on empty"))
    } else if size == 1 {
      Ok(TernaryTreeList::Empty)
    } else {
      self.dissoc(0)
    }
  }
  pub fn butlast(&self) -> Result<Self, String> {
    let size = self.len();
    if size == 0 {
      Err(String::from("calling butlast on empty"))
    } else if size == 1 {
      Ok(TernaryTreeList::Empty)
    } else {
      self.dissoc(size - 1)
    }
  }

  pub fn insert(&self, idx: usize, item: T, after: bool) -> Result<Self, String> {
    match self {
      Empty => {
        if idx == 0 {
          Ok(TernaryTreeList::Tree(TernaryTree::Leaf(item)))
        } else {
          Err(String::from("inserting into empty, but index is not 0"))
        }
      }

      Tree(t) => {
        if after {
          Ok(TernaryTreeList::Tree(t.insert_after(idx, item)?))
        } else {
          Ok(TernaryTreeList::Tree(t.insert_before(idx, item)?))
        }
      }
    }
  }
  pub fn assoc_before(&self, idx: usize, item: T) -> Result<Self, String> {
    self.insert(idx, item, false)
  }
  pub fn assoc_after(&self, idx: usize, item: T) -> Result<Self, String> {
    self.insert(idx, item, true)
  }
  // this function mutates original tree to make it more balanced
  pub fn force_inplace_balancing(&mut self) -> Result<(), String> {
    match self {
      Empty => Ok(()),
      Tree(t) => t.force_inplace_balancing(),
    }
  }

  pub fn unshift(&self, item: T) -> Self {
    self.prepend(item)
  }
  pub fn prepend(&self, item: T) -> Self {
    match self {
      Empty => TernaryTreeList::Tree(TernaryTree::Leaf(item)),
      Tree(t) => TernaryTreeList::Tree(t.prepend(item)),
    }
  }
  pub fn push(&self, item: T) -> Self {
    self.append(item)
  }
  /// insert_after last element, this not optimzed for performance
  pub fn append(&self, item: T) -> Self {
    match self {
      Empty => TernaryTreeList::Tree(TernaryTree::Leaf(item)),
      Tree(t) => TernaryTreeList::Tree(t.push_right(item)),
    }
  }
  /// optimized for amortized `O(1)` performance at best cases
  pub fn push_right(&self, item: T) -> Self {
    match self {
      Empty => TernaryTreeList::Tree(TernaryTree::Leaf(item)),
      Tree(t) => TernaryTreeList::Tree(t.push_right(item)),
    }
  }
  /// optimized for amortized `O(1)` performance at best cases
  pub fn push_left(&self, item: T) -> Self {
    match self {
      Empty => TernaryTreeList::Tree(TernaryTree::Leaf(item)),
      Tree(t) => TernaryTreeList::Tree(t.push_left(item)),
    }
  }

  pub fn drop_left(&self) -> Self {
    match self {
      Empty => TernaryTreeList::Empty,
      Tree(t) => {
        if t.len() == 1 {
          Self::Empty
        } else {
          Self::Tree(t.drop_left())
        }
      }
    }
  }

  /// optimized for amortized `O(1)` at best cases, but copies a lot
  pub fn drop_left_shallow(&self) -> Self {
    match self {
      Empty => TernaryTreeList::Empty,
      Tree(t) => {
        if t.len() == 1 {
          Self::Empty
        } else {
          match t.split_left_some(1).1 {
            Some(v) => Self::Tree(v),
            None => Self::Empty,
          }
        }
      }
    }
  }

  pub fn drop_right(&self) -> Self {
    match self {
      Empty => Self::Empty,
      Tree(t) => {
        if t.len() == 1 {
          Self::Empty
        } else {
          Self::Tree(t.drop_right())
        }
      }
    }
  }

  /// split into 2 lists, either could be Empty
  /// notice if index is too large, (Self, Empty) is returned, not providing index out of bound error
  pub fn split(self, idx: usize) -> (Self, Self) {
    if idx == 0 {
      (Self::Empty, self)
    } else if idx >= self.len() {
      (self, Self::Empty)
    } else {
      match self {
        Empty => (Self::Empty, Self::Empty),
        Tree(t) => {
          let (l, r) = t.split(idx);
          (Self::Tree(l), Self::Tree(r))
        }
      }
    }
  }

  /// optimized for amortized `O(1)` at best cases, but copies a lot
  pub fn drop_right_shallow(&self) -> Self {
    match self {
      Empty => Self::Empty,
      Tree(t) => {
        if t.len() == 1 {
          Self::Empty
        } else {
          match t.split_right_some(1).0 {
            Some(v) => Self::Tree(v),
            None => Self::Empty,
          }
        }
      }
    }
  }

  pub fn concat(raw: &[TernaryTreeList<T>]) -> Self {
    let mut trees: Vec<TernaryTree<T>> = vec![];
    for x in raw {
      match x {
        Empty => (),
        Tree(t) => trees.push(t.clone()),
      }
    }
    if trees.is_empty() {
      TernaryTreeList::Empty
    } else {
      TernaryTreeList::Tree(TernaryTree::concat(&trees))
    }
  }
  pub fn concat_dumb(raw: &[TernaryTreeList<T>]) -> Self {
    let mut trees: Vec<TernaryTree<T>> = vec![];
    for x in raw {
      match x {
        Empty => (),
        Tree(t) => trees.push(t.clone()),
      }
    }
    if trees.is_empty() {
      TernaryTreeList::Empty
    } else {
      TernaryTreeList::Tree(TernaryTree::concat_dumb(&trees))
    }
  }
  pub fn check_structure(&self) -> Result<(), String> {
    match self {
      Empty => Ok(()),
      Tree(t) => t.check_structure(),
    }
  }
  // excludes value at end_idx, kept aligned with JS & Clojure
  pub fn slice(&self, start_idx: usize, end_idx: usize) -> Result<Self, String> {
    if start_idx == end_idx {
      return Ok(TernaryTreeList::Empty);
    }
    match self {
      Empty => Err(String::from("empty")),
      Tree(t) => {
        // echo "slice {tree.formatListInline}: {start_idx}..{end_idx}"
        if end_idx > self.len() {
          return Err(format!("Slice range too large {} for {}", end_idx, self.format_inline()));
        }
        if start_idx > end_idx {
          return Err(format!("Invalid slice range {}..{} for {}", start_idx, end_idx, self));
        }
        if start_idx == end_idx {
          return Ok(TernaryTreeList::Empty);
        }
        Ok(TernaryTreeList::Tree(t.slice(start_idx, end_idx)?))
      }
    }
  }

  pub fn skip(&self, idx: usize) -> Result<Self, String> {
    // self.slice(idx, self.len())

    match self {
      Empty => Ok(TernaryTreeList::Empty),
      Tree(t) => {
        let size = t.len();
        match idx.cmp(&size) {
          Ordering::Equal => Ok(TernaryTreeList::Empty),
          Ordering::Greater => Err(format!("Skip range too large {} for {}", idx, self.format_inline())),
          Ordering::Less => Ok(TernaryTreeList::Tree(t.take_right(idx)?)),
        }
      }
    }
  }
  pub fn take(&self, idx: usize) -> Result<Self, String> {
    match self {
      Empty => Ok(TernaryTreeList::Empty),
      Tree(t) => {
        if idx == 0 {
          Ok(TernaryTreeList::Empty)
        } else if idx > self.len() {
          Err(format!("Take range too large {} for {}", idx, self.format_inline()))
        } else {
          Ok(TernaryTreeList::Tree(t.take_left(idx)?))
        }
      }
    }
  }

  pub fn reverse(&self) -> Self {
    match self {
      Empty => TernaryTreeList::Empty,
      Tree(t) => TernaryTreeList::Tree(t.reverse()),
    }
  }
  pub fn map<V>(&self, f: Arc<dyn Fn(&T) -> V>) -> TernaryTreeList<V> {
    match self {
      Empty => TernaryTreeList::Empty,
      Tree(t) => TernaryTreeList::Tree(t.map(f)),
    }
  }

  pub fn to_vec(&self) -> Vec<T> {
    match self {
      Empty => Vec::new(),
      Tree(t) => t.to_vec(),
    }
  }

  /// traverse all elements in list, use referenced value
  pub fn traverse(&self, f: &mut dyn FnMut(&T)) {
    match self {
      Empty => (),
      Tree(t) => t.traverse(f),
    }
  }

  /// traverse elements in list, use referenced value,
  /// returns `Ok` when all elements are traversed
  pub fn traverse_result<S>(&self, f: &mut dyn FnMut(&T) -> Result<(), S>) -> Result<(), S> {
    match self {
      Empty => Ok(()),
      Tree(t) => t.traverse_result(f),
    }
  }

  pub fn iter(&self) -> TernaryTreeListRefIntoIterator<T> {
    TernaryTreeListRefIntoIterator {
      value: self,
      index: 0,
      size: self.len(),
    }
  }
}

impl<T> Display for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Empty => write!(f, "Empty"),
      Tree(t) => write!(f, "{}", t),
    }
  }
}

// experimental code to turn `&TernaryTree<_>` into iterator
impl<'a, T> IntoIterator for &'a TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  type Item = &'a T;
  type IntoIter = TernaryTreeListRefIntoIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    TernaryTreeListRefIntoIterator {
      value: self,
      index: 0,
      size: self.len(),
    }
  }
}

pub struct TernaryTreeListRefIntoIterator<'a, T> {
  value: &'a TernaryTreeList<T>,
  index: usize,
  size: usize,
}

impl<'a, T> Iterator for TernaryTreeListRefIntoIterator<'a, T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.size {
      // println!("get: {} {}", self.value.format_inline(), self.index);
      let ret = self.value.loop_get(self.index);
      self.index += 1;
      ret
    } else {
      None
    }
  }
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> PartialEq for TernaryTreeList<T> {
  fn eq(&self, ys: &Self) -> bool {
    match (self, ys) {
      (Empty, Empty) => true,
      (Tree(x), Tree(y)) => x == y,
      _ => false,
    }
  }
}

impl<T> Eq for TernaryTreeList<T> where T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash {}

impl<T> PartialOrd for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Empty, Empty) => Ordering::Equal,
      (Empty, _) => Ordering::Less,
      (_, Empty) => Ordering::Greater,
      (Tree(l), Tree(r)) => l.cmp(r),
    }
  }
}

impl<T> Index<usize> for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  type Output = T;

  fn index<'b>(&self, idx: usize) -> &Self::Output {
    if idx >= self.len() {
      panic!("{} is out of bound at length {}", idx, self.len())
    } else {
      match self {
        Empty => panic!("list is empty to index"),
        Tree(t) => t.loop_get(idx),
      }
    }
  }
}

impl<T> Hash for TernaryTreeList<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Empty => {}
      Tree(t) => t.hash(state),
    }
  }
}

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

      TernaryTreeList::Tree(TernaryTree::rebuild_list(xs.len(), 0, &ys, 2))
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

      TernaryTreeList::Tree(TernaryTree::rebuild_list(xs.len(), 0, &ys, 2))
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

      TernaryTreeList::Tree(TernaryTree::rebuild_list(xs.len(), 0, &ys, 2))
    }
  }
}
