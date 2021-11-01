//! ternary tree with structural sharing.
//! a bit like 2-3 finger tree, however this library does not handle balancing well.
//! meanwhile, it is also an interesting library displaying with triples:
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

mod util;

use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::sync::Arc;

use util::{divide_ternary_sizes, rough_int_pow};

#[derive(Clone, Debug)]
pub enum TernaryTreeList<T> {
  Empty,
  Leaf(Arc<T>),
  Branch {
    size: usize,
    /// TODO currently depth could be inconsistent
    depth: u8,
    left: Arc<TernaryTreeList<T>>,
    middle: Arc<TernaryTreeList<T>>,
    right: Arc<TernaryTreeList<T>>,
  },
}

use TernaryTreeList::*;

impl<'a, T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> TernaryTreeList<T> {
  /// just get, will not compute recursively
  pub fn get_depth(&self) -> u8 {
    match self {
      Empty => 0,
      Leaf { .. } => 0,
      Branch { depth, .. } => depth.to_owned(),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Empty => true,
      Leaf { .. } => false,
      Branch {
        left,
        middle,
        right,
        ..
      } => left.is_empty() && middle.is_empty() && right.is_empty(), // TODO might be special structures
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Empty => 0,
      Leaf { .. } => 1,
      Branch { size, .. } => size.to_owned(),
    }
  }

  // make list again from existed
  fn rebuild_list(size: usize, offset: usize, xs: &[TernaryTreeList<T>]) -> Self {
    match size {
      0 => Empty,
      1 => xs[offset].to_owned(),
      2 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        Branch {
          size: left.len() + middle.len(),
          left: Arc::new(left.to_owned()),
          middle: Arc::new(middle.to_owned()),
          right: Arc::new(Empty),
          depth: decide_parent_depth_2(left, middle),
        }
      }
      3 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        let right = &xs[offset + 2];
        Branch {
          size: left.len() + middle.len() + right.len(),
          left: Arc::new(left.to_owned()),
          middle: Arc::new(middle.to_owned()),
          right: Arc::new(right.to_owned()),
          depth: decide_parent_depth_3(left, middle, right),
        }
      }
      _ => {
        let divided = divide_ternary_sizes(size);

        let left = Self::rebuild_list(divided.0, offset, xs);
        let middle = Self::rebuild_list(divided.1, offset + divided.0, xs);
        let right = Self::rebuild_list(divided.2, offset + divided.0 + divided.1, xs);
        Branch {
          size: left.len() + middle.len() + right.len(),
          depth: decide_parent_depth_3(&left, &middle, &right),
          left: Arc::new(left),
          middle: Arc::new(middle),
          right: Arc::new(right),
        }
      }
    }
  }
  pub fn from(xs: &[T]) -> Self {
    let mut ys: Vec<Self> = Vec::with_capacity(xs.len());
    for x in xs {
      ys.push(Leaf(Arc::new((*x).to_owned())))
    }

    Self::rebuild_list(xs.len(), 0, &ys)
  }

  pub fn is_leaf(self) -> bool {
    matches!(self, Leaf { .. })
  }

  pub fn is_branch(&self) -> bool {
    matches!(self, Branch { .. })
  }

  /// turn into a compare representation, with `_` for holes
  pub fn format_inline(&self) -> String {
    match self {
      Empty => String::from("_"),
      Leaf(value) => value.to_string(),
      Branch {
        left,
        middle,
        right,
        ..
      } => {
        // TODO maybe need more informations here
        format!(
          "({} {} {})",
          left.format_inline(),
          middle.format_inline(),
          right.format_inline()
        )
      }
    }
  }

  pub fn get(&self, idx: usize) -> Option<&T> {
    if self.is_empty() || idx >= self.len() {
      None
    } else {
      Some(self.ref_get(idx))
    }
  }

  // returns -1 if (not found)
  pub fn find_index(&self, f: Arc<dyn Fn(&T) -> bool>) -> i64 {
    match self {
      Empty => -1,
      Leaf(value) => {
        if f(value) {
          0
        } else {
          -1
        }
      }

      Branch {
        left,
        middle,
        right,
        ..
      } => {
        let v = left.find_index(f.clone());
        if v >= 0 {
          return v;
        }

        let v = middle.find_index(f.clone());
        if v >= 0 {
          return v + left.len() as i64;
        }

        let v = right.find_index(f.clone());
        if v >= 0 {
          return v + (left.len() as i64) + (middle.len() as i64);
        }

        -1
      }
    }
  }
  // returns -1 if (not foun)
  pub fn index_of(&self, item: &T) -> i64 {
    match self {
      Empty => -1,
      Leaf(value) => {
        if item == &**value {
          0
        } else {
          -1
        }
      }
      Branch {
        left,
        middle,
        right,
        ..
      } => {
        {
          let v = left.index_of(item);
          if v >= 0 {
            return v;
          }
        }
        {
          let v = middle.index_of(item);
          if v >= 0 {
            return v + left.len() as i64;
          }
        }

        {
          let v = right.index_of(item);
          if v >= 0 {
            return v + left.len() as i64 + middle.len() as i64;
          }
        }
        -1
      }
    }
  }

  /// recursively check structure
  pub fn is_shape_same(&self, ys: &Self) -> bool {
    if self.is_empty() {
      return ys.is_empty();
    }

    if ys.is_empty() {
      return false;
    }

    if self.len() != ys.len() {
      return false;
    }

    match (self, ys) {
      (Leaf(value), Leaf(v2)) => value == v2,
      (
        Branch {
          left,
          middle,
          right,
          ..
        },
        Branch {
          left: left2,
          middle: middle2,
          right: right2,
          ..
        },
      ) => left == left2 && middle == middle2 && right == right2,

      (_, _) => false,
    }
  }

  pub fn to_leaves(&self) -> Vec<TernaryTreeList<T>> {
    let mut acc: Vec<TernaryTreeList<T>> = Vec::with_capacity(self.len());
    let counter: RefCell<usize> = RefCell::new(0);
    write_leaves(self, &mut acc, &counter);
    assert_eq!(acc.len(), self.len());
    acc
  }

  pub fn ref_get(&self, idx: usize) -> &T {
    // println!("get: {} {}", self.format_inline(), idx);
    match self {
      Empty => unreachable!("looking at empty"),
      Leaf(value) => {
        if idx == 0 {
          value
        } else {
          unreachable!("expected 0")
        }
      }
      Branch {
        left,
        middle,
        right,
        ..
      } => {
        if idx < left.len() {
          left.ref_get(idx)
        } else if idx < left.len() + middle.len() {
          middle.ref_get(idx - left.len())
        } else {
          right.ref_get(idx - left.len() - middle.len())
        }
      }
    }
  }

  pub fn unsafe_get(&self, original_idx: usize) -> T {
    let mut tree_parent = self.to_owned();
    let mut idx = original_idx;
    while tree_parent != Empty {
      match tree_parent {
        Empty => unreachable!("empty"),
        Leaf(value) => {
          if idx == 0 {
            return (*value).to_owned();
          } else {
            unreachable!("Cannot get from leaf with index ${idx}")
          }
        }
        Branch {
          left,
          middle,
          right,
          size,
          ..
        } => {
          if idx > size - 1 {
            unreachable!("Index too large")
          }

          if left.len() + middle.len() + right.len() != size {
            unreachable!("tree.size does not match sum case branch sizes");
          }

          if idx < left.len() {
            tree_parent = (*left).to_owned();
          } else if idx < left.len() + middle.len() {
            tree_parent = (*middle).to_owned();
            idx -= left.len();
          } else {
            tree_parent = (*right).to_owned();
            idx -= left.len() + middle.len();
          }
        }
      }
    }

    unreachable!("Failed to get ${idx}")
  }

  pub fn first(&self) -> T {
    if self.is_empty() {
      unreachable!("Cannot get from empty list")
    } else {
      self.unsafe_get(0)
    }
  }

  pub fn last(&self) -> T {
    if self.is_empty() {
      unreachable!("Cannot get from empty list")
    } else {
      self.unsafe_get(self.len() - 1)
    }
  }
  pub fn assoc(&self, idx: usize, item: T) -> Self {
    if idx > self.len() - 1 {
      unreachable!("Index too large");
    }

    match self {
      Empty => {
        unreachable!("Cannot assoc into empty")
      }
      Leaf { .. } => {
        if idx == 0 {
          Leaf(Arc::new(item))
        } else {
          unreachable!("Cannot assoc leaf into index ${idx}")
        }
      }
      Branch {
        left,
        middle,
        right,
        size,
        ..
      } => {
        if left.len() + middle.len() + right.len() != *size {
          unreachable!("tree.size does not match sum case branch sizes");
        }

        if idx < left.len() {
          let changed_branch = left.assoc(idx, item);
          Branch {
            size: size.to_owned(),
            depth: decide_parent_depth_3(&changed_branch, middle, right),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          }
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.assoc(idx - left.len(), item);
          Branch {
            size: size.to_owned(),
            depth: decide_parent_depth_3(left, &changed_branch, right),
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
            right: right.to_owned(),
          }
        } else {
          let changed_branch = right.assoc(idx - left.len() - middle.len(), item);
          Branch {
            size: size.to_owned(),
            depth: decide_parent_depth_3(left, middle, &changed_branch),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch.to_owned()),
          }
        }
      }
    }
  }
  pub fn dissoc(&self, idx: usize) -> Self {
    if self.is_empty() {
      unreachable!("Cannot remove from empty list");
    }

    if idx > self.len() - 1 {
      unreachable!("Index too large ${idx}");
    }

    if self.len() == 1 {
      return Empty;
    }

    match self {
      Empty => unreachable!("dissoc out of bound"),
      Leaf { .. } => unreachable!("dissoc should be handled at branches"),
      Branch {
        left,
        middle,
        right,
        size,
        ..
      } => {
        if left.len() + middle.len() + right.len() != *size {
          unreachable!("tree.size does not match sum from branch sizes");
        }

        let result: Self;

        if idx < left.len() {
          let changed_branch = left.dissoc(idx);
          result = if changed_branch.is_empty() {
            Branch {
              size: *size - 1,
              depth: decide_parent_depth_2(middle, right),
              left: middle.to_owned(),
              middle: right.to_owned(),
              right: Arc::new(Empty),
            }
          } else {
            Branch {
              size: *size - 1,
              depth: decide_parent_depth_3(&changed_branch, middle, right),
              left: Arc::new(changed_branch.to_owned()),
              middle: middle.to_owned(),
              right: right.to_owned(),
            }
          }
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.dissoc(idx - left.len());
          result = if changed_branch.is_empty() {
            Branch {
              size: *size - 1,
              depth: decide_parent_depth_3(left, &changed_branch, right),
              left: left.to_owned(),
              middle: right.to_owned(),
              right: Arc::new(Empty),
            }
          } else {
            Branch {
              size: *size - 1,
              depth: decide_parent_depth_3(left, &changed_branch, right),
              left: left.to_owned(),
              middle: Arc::new(Empty),
              right: right.to_owned(),
            }
          }
        } else {
          let changed_branch = right.dissoc(idx - left.len() - middle.len());
          result = Branch {
            size: *size - 1,
            depth: decide_parent_depth_3(left, middle, &changed_branch),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch.to_owned()),
          }
        }

        match &result {
          Branch {
            left,
            middle,
            right,
            depth,
            size,
          } => {
            if **middle == Empty {
              (**left).to_owned()
            } else {
              Branch {
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: right.to_owned(),
                depth: *depth,
                size: *size,
              }
            }
          }
          Empty => unreachable!("unexpected empty"),
          Leaf { .. } => unreachable!("should not found leaf"),
        }
      }
    }
  }
  pub fn rest(&self) -> Self {
    if self.is_empty() {
      unreachable!("calling rest on empty")
    }

    self.dissoc(0)
  }
  pub fn butlast(&self) -> Self {
    if self.is_empty() {
      unreachable!("calling rest on empty")
    }
    self.dissoc(self.len() - 1)
  }

  pub fn insert(&self, idx: usize, item: T, after: bool) -> Self {
    match self {
      Empty => {
        if idx == 0 {
          Leaf(Arc::new(item))
        } else {
          unreachable!("Empty node is not a correct position for inserting")
        }
      }
      Leaf { .. } => {
        if after {
          Branch {
            depth: 1,
            size: 2,
            left: Arc::new(self.to_owned()),
            middle: Arc::new(Leaf(Arc::new(item))),
            right: Arc::new(Empty),
          }
        } else {
          Branch {
            depth: 1,
            size: 2,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: Arc::new(self.to_owned()),
            right: Arc::new(Empty),
          }
        }
      }
      Branch {
        left,
        middle,
        right,
        size,
        depth,
        ..
      } => {
        if self.len() == 1 {
          if after {
            // in compact mode, values placed at left
            return Branch {
              size: 2,
              depth: 1,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
              right: Arc::new(Empty),
            };
          } else {
            return Branch {
              size: 2,
              depth: 1,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
              right: Arc::new(Empty),
            };
          }
        }

        if self.len() == 2 {
          if after {
            if idx == 0 {
              return Branch {
                size: 3,
                depth: 1,
                left: left.to_owned(),
                middle: Arc::new(Leaf(Arc::new(item))),
                right: middle.to_owned(),
              };
            }
            if idx == 1 {
              return Branch {
                size: 3,
                depth: 1,
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: Arc::new(Leaf(Arc::new(item))),
              };
            } else {
              unreachable!("cannot insert after position 2 since only 2 elements here");
            }
          } else if idx == 0 {
            return Branch {
              size: 3,
              depth: 1,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
              right: middle.to_owned(),
            };
          } else if idx == 1 {
            return Branch {
              size: 3,
              depth: 1,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
              right: middle.to_owned(),
            };
          } else {
            unreachable!("cannot insert before position 2 since only 2 elements here")
          }
        }

        if left.len() + middle.len() + right.len() != *size {
          unreachable!("tree.size does not match sum case branch sizes");
        }

        // echo "picking: ", idx, " ", left.len(), " ", middle.len(), " ", right.len()

        if idx == 0 && !after && left.len() >= middle.len() && left.len() >= right.len() {
          return Branch {
            size: *size + 1,
            depth: depth + 1,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: Arc::new(self.to_owned()),
            right: Arc::new(Empty),
          };
        }

        if idx == *size - 1 && after && right.len() >= middle.len() && right.len() >= left.len() {
          return Branch {
            size: *size + 1,
            depth: depth + 1,
            left: Arc::new(self.to_owned()),
            middle: Arc::new(Leaf(Arc::new(item))),
            right: Arc::new(Empty),
          };
        }

        if after && idx == *size - 1 && right.len() == 0 && middle.len() >= left.len() {
          return Branch {
            size: *size + 1,
            depth: depth.to_owned(),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(Leaf(Arc::new(item))),
          };
        }

        if !after && idx == 0 && right.len() == 0 && middle.len() >= right.len() {
          return Branch {
            size: *size + 1,
            depth: depth.to_owned(),
            left: Arc::new(Leaf(Arc::new(item))),
            middle: left.to_owned(),
            right: middle.to_owned(),
          };
        }

        if idx < left.len() {
          let changed_branch = left.insert(idx, item, after);
          Branch {
            size: *size + 1,
            depth: decide_parent_depth_3(&changed_branch, middle, right),
            left: Arc::new(changed_branch.to_owned()),
            middle: middle.to_owned(),
            right: right.to_owned(),
          }
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.insert(idx - left.len(), item, after);

          Branch {
            size: *size + 1,
            depth: decide_parent_depth_3(left, &changed_branch.to_owned(), right),
            left: left.to_owned(),
            middle: Arc::new(changed_branch.to_owned()),
            right: right.to_owned(),
          }
        } else {
          let changed_branch = right.insert(idx - left.len() - middle.len(), item, after);

          Branch {
            size: *size + 1,
            depth: decide_parent_depth_3(left, middle, &changed_branch),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch.to_owned()),
          }
        }
      }
    }
  }
  pub fn assoc_before(&self, idx: usize, item: T) -> Self {
    self.insert(idx, item, false)
  }
  pub fn assoc_after(&self, idx: usize, item: T) -> Self {
    self.insert(idx, item, true)
  }
  // this function mutates original tree to make it more balanced
  pub fn force_inplace_balancing(&mut self) {
    let ys = self.to_owned().to_leaves();
    match self {
      Empty => {}
      Leaf { .. } => {}
      Branch {
        ref mut left,
        ref mut middle,
        ref mut right,
        ref mut depth,
        ..
      } => {
        // echo "Force inplace balancing case list: ", tree.size
        let new_tree = Self::rebuild_list(ys.len(), 0, &ys);
        // let new_tree = initTernaryTreeList(ys)
        match new_tree {
          Branch {
            left: left2,
            middle: middle2,
            right: right2,
            ..
          } => {
            *left = left2.to_owned();
            *middle = middle2.to_owned();
            *right = right2.to_owned();
            *depth = decide_parent_depth_3(&left2, &middle2, &right2);
          }
          Empty => unreachable!("expected some data"),
          Leaf { .. } => {
            unreachable!("expected leaf data")
          }
        }
      }
    }
  }
  // TODO, need better strategy for detecting
  pub fn maybe_reblance(&mut self) {
    match self {
      Empty => {}
      Leaf(..) => {}
      Branch { size, .. } => {
        if *size < 81 {
          // guessed number
          return;
        }
        let current_depth = self.get_depth();
        if current_depth > 20 && rough_int_pow(3, current_depth - 20) > self.len() {
          self.force_inplace_balancing()
        }
      }
    }
  }

  pub fn unshift(&self, item: T) -> Self {
    self.prepend(item, false)
  }
  pub fn prepend(&self, item: T, disable_balancing: bool) -> Self {
    if self.is_empty() {
      return Leaf(Arc::new(item));
    }

    let mut result = self.insert(0, item, false);

    if !disable_balancing {
      result.maybe_reblance();
    }

    result
  }
  pub fn push(&self, item: T) -> Self {
    self.append(item, false)
  }
  pub fn append(&self, item: T, disable_balancing: bool) -> Self {
    if self.is_empty() {
      return Leaf(Arc::new(item));
    }
    let mut result = self.insert(self.len() - 1, item, true);

    if !disable_balancing {
      result.maybe_reblance();
    }
    result
  }
  pub fn concat(xs_groups: &[TernaryTreeList<T>]) -> Self {
    let mut ys: Vec<TernaryTreeList<T>> = vec![];
    for x in xs_groups {
      if !x.is_empty() {
        ys.push(x.to_owned())
      }
    }
    let mut result = Self::rebuild_list(ys.len(), 0, &ys);
    result.maybe_reblance();
    result
  }
  pub fn check_structure(&self) -> Result<(), String> {
    if self.is_empty() {
      Ok(())
    } else {
      match self {
        Empty => Ok(()),
        Leaf { .. } => Ok(()),
        Branch {
          left,
          middle,
          right,
          size,
          depth,
        } => {
          if !self.is_empty() && *size == 0 {
            unreachable!("branch but has size")
          }

          if *size != left.len() + middle.len() + right.len() {
            unreachable!("Bad size at branch ${formatListInline(tree)}");
          }

          if *depth != decide_parent_depth_3(left, middle, right) {
            return Err(format!("Bad depth at branch {}", self.format_inline()));
          }

          left.check_structure()?;
          middle.check_structure()?;
          right.check_structure()?;

          Ok(())
        }
      }
    }
  }
  // excludes value at end_idx, kept aligned with JS & Clojure
  pub fn slice(&self, start_idx: usize, end_idx: usize) -> Self {
    // echo "slice {tree.formatListInline}: {start_idx}..{end_idx}"
    if end_idx > self.len() {
      unreachable!("Slice range too large {end_idx} for {tree}");
    }
    if start_idx > end_idx {
      unreachable!(
        "Invalid slice range {}..{} for {}",
        start_idx, end_idx, self
      );
    }
    if start_idx == end_idx {
      return Empty;
    }

    match self {
      Empty => unreachable!("slicing from empty"),
      Leaf { .. } => {
        if start_idx == 0 && end_idx == 1 {
          self.to_owned()
        } else {
          unreachable!("Invalid slice range for a leaf: ${start_idx} ${end_idx}");
        }
      }
      Branch {
        left,
        right,
        middle,
        ..
      } => {
        if start_idx == 0 && end_idx == self.len() {
          return self.to_owned();
        }

        // echo "sizes: {left.len()} {middle.len()} {right.len()}"

        if start_idx >= left.len() + middle.len() {
          return right.slice(
            start_idx - left.len() - middle.len(),
            end_idx - left.len() - middle.len(),
          );
        }
        if start_idx >= left.len() {
          if end_idx <= left.len() + middle.len() {
            return middle.slice(start_idx - left.len(), end_idx - left.len());
          } else {
            let middle_cut = middle.slice(start_idx - left.len(), middle.len());
            let right_cut = right.slice(0, end_idx - left.len() - middle.len());
            return Self::concat(&[middle_cut, right_cut]);
          }
        }

        if end_idx <= left.len() {
          return left.slice(start_idx, end_idx);
        }

        if end_idx <= left.len() + middle.len() {
          let left_cut = left.slice(start_idx, left.len());
          let middle_cut = middle.slice(0, end_idx - left.len());
          return Self::concat(&[left_cut, middle_cut]);
        }

        if end_idx <= left.len() + middle.len() + right.len() {
          let left_cut = left.slice(start_idx, left.len());
          let right_cut = right.slice(0, end_idx - left.len() - middle.len());
          match &**middle {
            Empty => {
              return Self::concat(&[left_cut, right_cut]);
            }
            _ => {
              return Self::concat(&[left_cut, (**middle).to_owned(), right_cut]);
            }
          }
        }
        unreachable!("Unknown");
      }
    }
  }

  pub fn reverse(&self) -> Self {
    if self.is_empty() {
      return self.to_owned();
    }

    match self {
      Empty => Empty,
      Leaf { .. } => self.to_owned(),
      Branch {
        left,
        middle,
        right,
        size,
        depth,
      } => Branch {
        size: *size,
        depth: *depth,
        left: Arc::new(right.reverse()),
        middle: Arc::new(middle.reverse()),
        right: Arc::new(left.reverse()),
      },
    }
  }
  pub fn map<V>(&self, f: Arc<dyn Fn(&T) -> V>) -> TernaryTreeList<V> {
    match self {
      Empty => Empty,
      Leaf(value) => Leaf(Arc::new(f(value))),
      Branch {
        left,
        middle,
        right,
        size,
        depth,
      } => Branch {
        size: *size,
        depth: *depth,
        left: Arc::new(left.map(f.clone())),
        middle: Arc::new(middle.map(f.clone())),
        right: Arc::new(right.map(f.clone())),
      },
    }
  }

  pub fn to_vec(&self) -> Vec<T> {
    let mut xs = vec![];

    // TODO
    for item in &self.to_owned() {
      xs.push(item.to_owned());
    }

    xs
  }
}

// pass several children here
fn decide_parent_depth_2<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash>(
  x0: &TernaryTreeList<T>,
  x1: &TernaryTreeList<T>,
) -> u8 {
  cmp::max(x0.get_depth(), x1.get_depth()) + 1
}

// pass several children here
fn decide_parent_depth_3<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash>(
  x0: &TernaryTreeList<T>,
  x1: &TernaryTreeList<T>,
  x2: &TernaryTreeList<T>,
) -> u8 {
  cmp::max(cmp::max(x0.get_depth(), x1.get_depth()), x2.get_depth()) + 1
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Display
  for TernaryTreeList<T>
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TernaryTreeList[{}, ...]", self.len())
  }
}

// code to turn `TernaryTreeList<_>` into iterator
// impl<'a, T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> IntoIterator
//   for TernaryTreeList<T>
// {
//   type Item = &'a T;
//   type IntoIter = TernaryTreeIntoIterator<'a, T>;

//   fn into_iter(self) -> Self::IntoIter {
//     TernaryTreeIntoIterator {
//       value: self,
//       index: 0,
//     }
//   }
// }

// pub struct TernaryTreeIntoIterator<'a, T> {
//   value: TernaryTreeList<T>,
//   index: usize,
// }

// impl<'a, T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Iterator
//   for TernaryTreeIntoIterator<'a, T>
// {
//   type Item = &'a T;
//   fn next(&mut self) -> Option<Self::Item> {
//     if self.index < self.value.len() {
//       let ret = self.value.get(self.index);
//       self.index += 1;
//       ret
//     } else {
//       None
//     }
//   }
// }

// experimental code to turn `&TernaryTreeList<_>` into iterator
impl<'a, T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> IntoIterator
  for &'a TernaryTreeList<T>
{
  type Item = &'a T;
  type IntoIter = TernaryTreeRefIntoIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    TernaryTreeRefIntoIterator {
      value: self,
      index: 0,
    }
  }
}

pub struct TernaryTreeRefIntoIterator<'a, T> {
  value: &'a TernaryTreeList<T>,
  index: usize,
}

impl<'a, T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Iterator
  for TernaryTreeRefIntoIterator<'a, T>
{
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.value.len() {
      // println!("get: {} {}", self.value.format_inline(), self.index);
      let ret = self.value.ref_get(self.index);
      self.index += 1;
      Some(ret)
    } else {
      None
    }
  }
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> PartialEq
  for TernaryTreeList<T>
{
  fn eq(&self, ys: &Self) -> bool {
    if self.len() != ys.len() {
      return false;
    }

    for idx in 0..ys.len() {
      if self.unsafe_get(idx) != ys.unsafe_get(idx) {
        return false;
      }
    }

    true
  }
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Eq
  for TernaryTreeList<T>
{
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> PartialOrd
  for TernaryTreeList<T>
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Ord
  for TernaryTreeList<T>
{
  fn cmp(&self, other: &Self) -> Ordering {
    if self.len() == other.len() {
      for idx in 0..self.len() {
        match self.unsafe_get(idx).cmp(&other.unsafe_get(idx)) {
          Ordering::Equal => {}
          a => return a,
        }
      }

      Ordering::Equal
    } else {
      self.len().cmp(&other.len())
    }
  }
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Index<usize>
  for TernaryTreeList<T>
{
  type Output = T;

  fn index<'b>(&self, idx: usize) -> &Self::Output {
    // println!("get: {} {}", self.format_inline(), idx);
    self.ref_get(idx)
  }
}

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> Hash
  for TernaryTreeList<T>
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    "ternary".hash(state);
    match self {
      Empty => {}
      Leaf(value) => {
        "leaf".hash(state);
        value.hash(state);
      }
      Branch {
        left,
        middle,
        right,
        ..
      } => {
        "branch".hash(state);
        left.hash(state);
        middle.hash(state);
        right.hash(state);
      }
    }
  }
}

/// internal function for mutable writing
fn write_leaves<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash>(
  xs: &TernaryTreeList<T>,
  acc: &mut Vec<TernaryTreeList<T>>,
  counter: &RefCell<usize>,
) {
  if xs.is_empty() {
    return;
  }

  match xs {
    Empty => {}
    Leaf { .. } => {
      let idx = counter.take();
      acc.push(xs.to_owned());

      counter.replace(idx + 1);
    }
    Branch {
      left,
      middle,
      right,
      ..
    } => {
      write_leaves(left, acc, counter);
      write_leaves(middle, acc, counter);
      write_leaves(right, acc, counter);
    }
  }
}
