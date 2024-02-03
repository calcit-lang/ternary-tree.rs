//! internal implementation of this tree.
//! this file should be optimized for speed, even to skip some checks repeatedly down the tree,
//! checks can be added at the struct that wraps this tree

mod finger;

use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::sync::Arc;

use crate::util::{divide_ternary_sizes, triple_size};

/// internal tree structure, it can't be empty
#[derive(Clone, Debug)]
pub enum TernaryTree<T> {
  Leaf(Arc<T>),
  Branch2 {
    size: usize,
    left: Arc<TernaryTree<T>>,
    middle: Arc<TernaryTree<T>>,
  },
  Branch3 {
    size: usize,
    left: Arc<TernaryTree<T>>,
    middle: Arc<TernaryTree<T>>,
    right: Arc<TernaryTree<T>>,
  },
}

use TernaryTree::*;

impl<'a, T> TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  pub fn len(&self) -> usize {
    match self {
      Leaf { .. } => 1,
      Branch2 { size, .. } => *size,
      Branch3 { size, .. } => *size,
    }
  }

  /// never empty in this tree
  pub fn is_empty(&self) -> bool {
    false
  }

  /// make list again from existed
  /// use a factor to control side branches to be shallow with smaller depth
  /// root node has a factor of 2
  pub fn rebuild_list(size: usize, offset: usize, xs: &[TernaryTree<T>], factor: u8) -> Self {
    match size {
      0 => unreachable!("Does not work for empty list"),
      1 => xs[offset].to_owned(),
      2 => Self::rebuild_list_side(size, offset, xs),
      3 => Self::rebuild_list_side(size, offset, xs),
      _ => {
        let side_capacity = triple_size(factor - 1);
        if side_capacity * 2 < size {
          let divided = (side_capacity, size - side_capacity - side_capacity, side_capacity);

          let left = Self::rebuild_list_side(divided.0, offset, xs);
          let middle = Self::rebuild_list(divided.1, offset + divided.0, xs, factor + 1);
          let right = Self::rebuild_list_side(divided.2, offset + divided.0 + divided.1, xs);

          Branch3 {
            size,
            left: Arc::new(left),
            middle: Arc::new(middle),
            right: Arc::new(right),
          }
        } else {
          Self::rebuild_list_side(size, offset, xs)
        }
      }
    }
  }

  // sides have different algorithm
  pub fn rebuild_list_side(size: usize, offset: usize, xs: &[TernaryTree<T>]) -> Self {
    match size {
      0 => unreachable!("Does not work for empty list"),
      1 => xs[offset].to_owned(),
      2 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        Branch2 {
          size: left.len() + middle.len(),
          left: Arc::new(left.to_owned()),
          middle: Arc::new(middle.to_owned()),
        }
      }
      3 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        let right = &xs[offset + 2];
        Branch3 {
          size: 3,
          left: Arc::new(left.to_owned()),
          middle: Arc::new(middle.to_owned()),
          right: Arc::new(right.to_owned()),
        }
      }
      _ => {
        let divided = divide_ternary_sizes(size);

        let left = Self::rebuild_list_side(divided.0, offset, xs);
        let middle = Self::rebuild_list_side(divided.1, offset + divided.0, xs);
        let right = Self::rebuild_list_side(divided.2, offset + divided.0 + divided.1, xs);

        Branch3 {
          size: left.len() + middle.len() + right.len(),
          left: Arc::new(left),
          middle: Arc::new(middle),
          right: Arc::new(right),
        }
      }
    }
  }

  /// turn into a representation in triples, with `_` for holes
  pub fn format_inline(&self) -> String {
    match self {
      Leaf(value) => value.to_string(),
      Branch2 { left, middle, .. } => {
        // TODO maybe need more informations here
        format!("({} {})", left.format_inline(), middle.format_inline())
      }
      Branch3 { left, middle, right, .. } => {
        // TODO maybe need more informations here
        format!("({} {} {})", left.format_inline(), middle.format_inline(), right.format_inline())
      }
    }
  }

  pub fn find_index(&self, f: Arc<dyn Fn(&T) -> bool>) -> Option<i64> {
    match self {
      Leaf(value) => {
        if f(value) {
          Some(0)
        } else {
          None
        }
      }

      Branch2 { left, middle, .. } => left
        .find_index(f.to_owned())
        .or_else(|| middle.find_index(f.to_owned()).map(|pos| pos + left.len() as i64)),

      Branch3 { left, middle, right, .. } => left
        .find_index(f.to_owned())
        .or_else(|| middle.find_index(f.to_owned()).map(|pos| pos + left.len() as i64))
        .or_else(|| {
          right
            .find_index(f.to_owned())
            .map(|pos| pos + left.len() as i64 + middle.len() as i64)
        }),
    }
  }

  pub fn index_of(&self, item: &T) -> Option<usize> {
    match self {
      Leaf(value) => {
        if item == &**value {
          Some(0)
        } else {
          None
        }
      }
      Branch2 { left, middle, .. } => left.index_of(item).or_else(|| middle.index_of(item).map(|pos| pos + left.len())),
      Branch3 { left, middle, right, .. } => left
        .index_of(item)
        .or_else(|| middle.index_of(item).map(|pos| pos + left.len()))
        .or_else(|| right.index_of(item).map(|pos| pos + left.len() + middle.len())),
    }
  }

  /// recursively check structure
  pub fn eq_shape(&self, ys: &Self) -> bool {
    if self.len() != ys.len() {
      return false;
    }

    match (self, ys) {
      (Leaf(value), Leaf(v2)) => value == v2,
      (
        Branch2 { left, middle, .. },
        Branch2 {
          left: left2,
          middle: middle2,
          ..
        },
      ) => left.eq_shape(left2) && middle.eq_shape(middle2),
      (
        Branch3 { left, middle, right, .. },
        Branch3 {
          left: left2,
          middle: middle2,
          right: right2,
          ..
        },
      ) => left.eq_shape(left2) && middle.eq_shape(middle2) && right.eq_shape(right2),

      (_, _) => false,
    }
  }

  /// internal usages for rebuilding tree
  fn to_leaves(&self) -> Vec<TernaryTree<T>> {
    let mut acc: Vec<TernaryTree<T>> = Vec::with_capacity(self.len());
    let counter: Cell<usize> = Cell::new(0);
    write_leaves(self, &mut acc, &counter);
    assert_eq!(acc.len(), self.len());
    acc
  }

  /// get with reference, but index is not checked, returns last element if too large
  pub fn ref_get(&self, idx: usize) -> &T {
    // println!("get: {} {}", self.format_inline(), idx);
    // if idx >= self.len() {
    //   println!("get from out of bound: {} {}", idx, self.len());
    //   return None;
    // }
    match self {
      Branch3 { left, middle, right, .. } => {
        let base = left.len();
        if idx < base {
          return left.ref_get(idx);
        }
        let idx_in_middle = idx - base;
        if idx_in_middle < middle.len() {
          middle.ref_get(idx_in_middle)
        } else {
          right.ref_get(idx_in_middle - middle.len())
        }
      }
      Branch2 { left, middle, .. } => {
        let base = left.len();
        if idx < base {
          left.ref_get(idx)
        } else {
          middle.ref_get(idx - base)
        }
      }
      Leaf(value) => value,
    }
  }

  /// get am element via drilling down the branch with a mutable loop,
  /// supposed to be faster than `ref_get` since it's more like VM instructions
  pub fn loop_get(&'a self, original_idx: usize) -> &'a T {
    let tree_parent = Cell::new(self);
    let mut idx = original_idx;
    loop {
      match tree_parent.get() {
        Leaf(value) => {
          return value;
        }
        Branch2 { left, middle, .. } => {
          if idx < left.len() {
            tree_parent.set(left);
          } else {
            tree_parent.set(middle);
            idx -= left.len();
          }
        }
        Branch3 { left, middle, right, .. } => {
          if idx < left.len() {
            tree_parent.set(left);
            continue;
          }
          idx -= left.len();

          if idx < middle.len() {
            tree_parent.set(middle);
            continue;
          }

          tree_parent.set(right);
          idx -= middle.len();
        }
      }
    }
  }

  pub fn first(&self) -> Option<&T> {
    match self {
      Leaf(value) => Some(value),
      Branch2 { left, .. } => left.first(),
      Branch3 { left, .. } => left.first(),
    }
  }

  pub fn last(&self) -> Option<&T> {
    match self {
      Leaf(value) => Some(value),
      Branch2 { middle, .. } => middle.last(),
      Branch3 { right, .. } => right.last(),
    }
  }

  // insert new element at position, does not check whether the index is out of bound
  pub fn assoc(&self, idx: usize, item: T) -> Result<Self, String> {
    match self {
      Leaf { .. } => {
        if idx == 0 {
          Ok(Leaf(Arc::new(item)))
        } else {
          Err(format!("Cannot assoc leaf into index {}", idx))
        }
      }
      Branch2 { left, middle, size, .. } => {
        if idx < left.len() {
          let changed_branch = left.assoc(idx, item)?;
          Ok(Branch2 {
            size: size.to_owned(),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
          })
        } else {
          let changed_branch = middle.assoc(idx - left.len(), item)?;
          Ok(Branch2 {
            size: size.to_owned(),
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
          })
        }
      }
      Branch3 {
        left, middle, right, size, ..
      } => {
        if idx < left.len() {
          let changed_branch = left.assoc(idx, item)?;
          Ok(Branch3 {
            size: size.to_owned(),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          })
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.assoc(idx - left.len(), item)?;
          Ok(Branch3 {
            size: size.to_owned(),
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
            right: right.to_owned(),
          })
        } else {
          let changed_branch = right.assoc(idx - left.len() - middle.len(), item)?;
          Ok(Branch3 {
            size: size.to_owned(),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch),
          })
        }
      }
    }
  }

  // remove element from give position, does not check whether the index is out of bound
  pub fn dissoc(&self, idx: usize) -> Result<Self, String> {
    match self {
      Leaf { .. } => unreachable!("dissoc should be handled at branches"),
      Branch2 { left, middle, size, .. } => {
        if idx < left.len() {
          if left.len() == 1 {
            Ok((**middle).to_owned())
          } else {
            let changed_branch = left.dissoc(idx)?;
            Ok(Branch2 {
              size: *size - 1,
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
            })
          }
        } else if middle.len() == 1 {
          Ok((**left).to_owned())
        } else {
          let changed_branch = middle.dissoc(idx - left.len())?;
          Ok(Branch2 {
            size: *size - 1,
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
          })
        }
      }

      Branch3 {
        left, middle, right, size, ..
      } => {
        if left.len() + middle.len() + right.len() != *size {
          return Err(format!(
            "tree {} does not match sum from branch sizes {}",
            self.format_inline(),
            self.len()
          ));
        }

        if idx < left.len() {
          if left.len() == 1 {
            Ok(Branch2 {
              size: *size - 1,
              left: middle.to_owned(),
              middle: right.to_owned(),
            })
          } else {
            let changed_branch = left.dissoc(idx)?;
            Ok(Branch3 {
              size: *size - 1,
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
              right: right.to_owned(),
            })
          }
        } else if idx < left.len() + middle.len() {
          if middle.len() == 1 {
            Ok(Branch2 {
              size: *size - 1,
              left: left.to_owned(),
              middle: right.to_owned(),
            })
          } else {
            let changed_branch = middle.dissoc(idx - left.len())?;
            Ok(Branch3 {
              size: *size - 1,
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
              right: right.to_owned(),
            })
          }
        } else if right.len() == 1 {
          Ok(Branch2 {
            size: *size - 1,
            left: left.to_owned(),
            middle: middle.to_owned(),
          })
        } else {
          let changed_branch = right.dissoc(idx - left.len() - middle.len())?;
          Ok(Branch3 {
            size: *size - 1,
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch),
          })
        }
      }
    }
  }
  pub fn rest(&self) -> Result<Self, String> {
    if self.len() == 1 {
      Err(String::from("unexpected leaf for rest"))
    } else {
      self.dissoc(0)
    }
  }
  pub fn butlast(&self) -> Result<Self, String> {
    if self.len() == 1 {
      Err(String::from("unexpected leaf for butlast"))
    } else {
      self.dissoc(self.len() - 1)
    }
  }

  pub fn insert(&self, idx: usize, item: T, after: bool) -> Result<Self, String> {
    match self {
      Leaf { .. } => {
        if after {
          Ok(Branch2 {
            size: 2,
            left: Arc::new(self.to_owned()),
            middle: Arc::new(Leaf(Arc::new(item))),
          })
        } else {
          Ok(Branch2 {
            size: 2,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: Arc::new(self.to_owned()),
          })
        }
      }

      Branch2 { left, middle, size, .. } => {
        if self.len() == 1 {
          if after {
            // in compact mode, values placed at left
            return Ok(Branch2 {
              size: 2,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
            });
          } else {
            return Ok(Branch2 {
              size: 2,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
            });
          }
        }

        if self.len() == 2 {
          if after {
            if idx == 0 {
              return Ok(Branch3 {
                size: 3,
                left: left.to_owned(),
                middle: Arc::new(Leaf(Arc::new(item))),
                right: middle.to_owned(),
              });
            }
            if idx == 1 {
              return Ok(Branch3 {
                size: 3,
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: Arc::new(Leaf(Arc::new(item))),
              });
            } else {
              return Err(String::from("cannot insert after position 2 since only 2 elements here"));
            }
          } else if idx == 0 {
            return Ok(Branch3 {
              size: 3,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
              right: middle.to_owned(),
            });
          } else if idx == 1 {
            return Ok(Branch3 {
              size: 3,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
              right: middle.to_owned(),
            });
          } else {
            return Err(String::from("cannot insert before position 2 since only 2 elements here"));
          }
        }

        if left.len() + middle.len() != *size {
          return Err(String::from("tree.size does not match sum case branch sizes"));
        }

        // echo "picking: ", idx, " ", left.len(), " ", middle.len(), " ", right.len()

        if idx == 0 && !after {
          return Ok(Branch3 {
            size: *size + 1,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: left.to_owned(),
            right: middle.to_owned(),
          });
        }

        if idx == *size - 1 && after {
          return Ok(Branch3 {
            size: *size + 1,
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(Leaf(Arc::new(item))),
          });
        }

        if idx < left.len() {
          let changed_branch = left.insert(idx, item, after)?;
          Ok(Branch2 {
            size: *size + 1,
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
          })
        } else {
          let changed_branch = middle.insert(idx - left.len(), item, after)?;

          Ok(Branch2 {
            size: *size + 1,
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
          })
        }
      }
      Branch3 {
        left, middle, right, size, ..
      } => {
        if self.len() == 1 {
          if after {
            // in compact mode, values placed at left
            return Ok(Branch2 {
              size: 2,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
            });
          } else {
            return Ok(Branch2 {
              size: 2,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
            });
          }
        }

        if self.len() == 2 {
          if after {
            if idx == 0 {
              return Ok(Branch3 {
                size: 3,
                left: left.to_owned(),
                middle: Arc::new(Leaf(Arc::new(item))),
                right: middle.to_owned(),
              });
            }
            if idx == 1 {
              return Ok(Branch3 {
                size: 3,
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: Arc::new(Leaf(Arc::new(item))),
              });
            } else {
              return Err(String::from("cannot insert after position 2 since only 2 elements here"));
            }
          } else if idx == 0 {
            return Ok(Branch3 {
              size: 3,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
              right: middle.to_owned(),
            });
          } else if idx == 1 {
            return Ok(Branch3 {
              size: 3,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
              right: middle.to_owned(),
            });
          } else {
            return Err(String::from("cannot insert before position 2 since only 2 elements here"));
          }
        }

        if left.len() + middle.len() + right.len() != *size {
          return Err(String::from("tree.size does not match sum case branch sizes"));
        }

        // echo "picking: ", idx, " ", left.len(), " ", middle.len(), " ", right.len()

        if idx == 0 && !after && left.len() >= middle.len() && left.len() >= right.len() {
          return Ok(Branch2 {
            size: *size + 1,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: Arc::new(self.to_owned()),
          });
        }

        if idx == *size - 1 && after && right.len() >= middle.len() && right.len() >= left.len() {
          return Ok(Branch2 {
            size: *size + 1,
            left: Arc::new(self.to_owned()),
            middle: Arc::new(Leaf(Arc::new(item))),
          });
        }

        if after && idx == *size - 1 && right.len() == 0 && middle.len() >= left.len() {
          return Ok(Branch3 {
            size: *size + 1,
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(Leaf(Arc::new(item))),
          });
        }

        if !after && idx == 0 && right.len() == 0 && middle.len() >= right.len() {
          return Ok(Branch3 {
            size: *size + 1,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: left.to_owned(),
            right: middle.to_owned(),
          });
        }

        if idx < left.len() {
          let changed_branch = left.insert(idx, item, after)?;
          Ok(Branch3 {
            size: *size + 1,
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          })
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.insert(idx - left.len(), item, after)?;

          Ok(Branch3 {
            size: *size + 1,
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
            right: right.to_owned(),
          })
        } else {
          let changed_branch = right.insert(idx - left.len() - middle.len(), item, after)?;

          Ok(Branch3 {
            size: *size + 1,
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch),
          })
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
    let ys = self.to_leaves();
    *self = Self::rebuild_list(ys.len(), 0, &ys, 2);
    Ok(())
  }

  pub fn unshift(&self, item: T) -> Self {
    self.prepend(item)
  }
  pub fn prepend(&self, item: T) -> Self {
    match self.insert(0, item, false) {
      Ok(v) => v,
      Err(e) => unreachable!("{}", e),
    }
  }
  pub fn push(&self, item: T) -> Self {
    self.append(item)
  }
  pub fn append(&self, item: T) -> Self {
    match self.insert(self.len() - 1, item, true) {
      Ok(v) => v,
      Err(e) => unreachable!("{}", e),
    }
  }

  pub fn concat(raw: &[TernaryTree<T>]) -> Self {
    let mut xs_groups: Vec<TernaryTree<T>> = Vec::with_capacity(raw.len());
    for x in raw {
      xs_groups.push(x.to_owned());
    }
    match xs_groups.len() {
      0 => unreachable!("does not work with empty list in ternary-tree"),
      1 => xs_groups[0].to_owned(),
      2 => {
        let left = xs_groups[0].to_owned();
        let middle = xs_groups[1].to_owned();
        TernaryTree::Branch2 {
          size: left.len() + middle.len(),
          left: Arc::new(left),
          middle: Arc::new(middle),
        }
      }
      3 => {
        let left = xs_groups[0].to_owned();
        let middle = xs_groups[1].to_owned();
        let right = xs_groups[2].to_owned();
        TernaryTree::Branch3 {
          size: left.len() + middle.len() + right.len(),
          left: Arc::new(left),
          middle: Arc::new(middle),
          right: Arc::new(right),
        }
      }
      _ => {
        let mut ys: Vec<TernaryTree<T>> = vec![];
        for x in xs_groups {
          ys.push(x.to_owned())
        }
        Self::rebuild_list(ys.len(), 0, &ys, 2)
      }
    }
  }
  pub fn check_structure(&self) -> Result<(), String> {
    match self {
      Leaf { .. } => Ok(()),
      Branch2 { left, middle, size } => {
        if *size != left.len() + middle.len() {
          return Err(format!("Bad size at branch {}", self.format_inline()));
        }

        left.check_structure()?;
        middle.check_structure()?;

        Ok(())
      }
      Branch3 { left, middle, right, size } => {
        if *size != left.len() + middle.len() + right.len() {
          return Err(format!("Bad size at branch {}", self.format_inline()));
        }

        left.check_structure()?;
        middle.check_structure()?;
        right.check_structure()?;

        Ok(())
      }
    }
  }

  /// excludes value at end_idx, kept aligned with JS & Clojure
  /// does not check at inside
  pub fn slice(&self, start_idx: usize, end_idx: usize) -> Result<Self, String> {
    // echo "slice {tree.formatListInline}: {start_idx}..{end_idx}"

    match self {
      Leaf { .. } => {
        if start_idx == 0 && end_idx == 1 {
          Ok(self.to_owned())
        } else {
          Err(format!("Invalid slice range for a leaf: {} {}", start_idx, end_idx))
        }
      }

      Branch2 { left, middle, .. } => {
        if start_idx == 0 && end_idx == self.len() {
          Ok(self.to_owned())
        } else if start_idx >= left.len() {
          // echo "sizes: {left.len()} {middle.len()} {right.len()}"
          middle.slice(start_idx - left.len(), end_idx - left.len())
        } else if end_idx <= left.len() {
          left.slice(start_idx, end_idx)
        } else {
          let left_cut = left.slice(start_idx, left.len())?;
          let middle_cut = middle.slice(0, end_idx - left.len())?;

          Ok(TernaryTree::Branch2 {
            size: left_cut.len() + middle_cut.len(),
            left: Arc::new(left_cut),
            middle: Arc::new(middle_cut),
          })
        }
      }
      Branch3 { left, right, middle, .. } => {
        let base1 = left.len();
        let base2 = base1 + middle.len();
        if start_idx == 0 && end_idx == self.len() {
          Ok(self.to_owned())
        } else if start_idx >= base2 {
          right.slice(start_idx - base2, end_idx - base2)
        } else if start_idx >= base1 {
          if end_idx <= base1 + middle.len() {
            middle.slice(start_idx - base1, end_idx - base1)
          } else {
            let middle_cut = middle.slice(start_idx - base1, middle.len())?;
            let right_cut = right.slice(0, end_idx - base2)?;

            Ok(TernaryTree::Branch2 {
              size: middle_cut.len() + right_cut.len(),
              left: Arc::new(middle_cut),
              middle: Arc::new(right_cut),
            })
          }
        } else if end_idx <= base1 {
          left.slice(start_idx, end_idx)
        } else if end_idx <= base2 {
          let left_cut = left.slice(start_idx, base1)?;
          let middle_cut = middle.slice(0, end_idx - base1)?;

          Ok(TernaryTree::Branch2 {
            size: left_cut.len() + middle_cut.len(),
            left: Arc::new(left_cut),
            middle: Arc::new(middle_cut),
          })
        } else {
          let left_cut = left.slice(start_idx, base1)?;
          let right_cut = right.slice(0, end_idx - base2)?;
          Ok(TernaryTree::Branch3 {
            size: left_cut.len() + middle.len() + right_cut.len(),
            left: Arc::new(left_cut),
            middle: middle.clone(),
            right: Arc::new(right_cut),
          })
        }
      }
    }
  }

  /// split at index, returns a tuple of two trees
  /// this function takes ownership of the tree, and returns two new trees, release the original one
  /// checks need to be done before calling this function
  pub fn split(self, idx: usize) -> (Self, Self) {
    match self {
      Leaf(_value) => unreachable!("Invalid split index for a leaf: {}", idx),
      Branch2 { left, middle, size } => {
        if idx == 0 {
          unreachable!("Invalid split index 0 for a branch2: {}", idx)
        } else if idx < left.len() {
          let (cut_a, cut_b) = (*left).clone().split(idx);

          (
            cut_a,
            Branch2 {
              size: size - idx,
              left: Arc::new(cut_b),
              middle: middle.to_owned(),
            },
          )
        } else if idx == left.len() {
          ((*left).to_owned(), (*middle).to_owned())
        } else if idx >= size {
          unreachable!("Invalid split index end for a branch2: {}", idx)
        } else if idx - left.len() < middle.len() {
          let (cut_a, cut_b) = (*middle).to_owned().split(idx - left.len());

          (
            Branch2 {
              size: left.len() + cut_a.len(),
              left: left.to_owned(),
              middle: Arc::new(cut_a),
            },
            cut_b,
          )
        } else {
          unreachable!("Invalid split index end for a branch2: {}", idx)
        }
      }
      Branch3 { left, middle, right, size } => {
        if idx == 0 {
          unreachable!("Invalid split index 0 for a branch3: {}", idx)
        } else if idx < left.len() {
          let (cut_a, cut_b) = (*left).to_owned().split(idx);

          (
            cut_a,
            Branch3 {
              size: size - idx,
              left: Arc::new(cut_b),
              middle: middle.to_owned(),
              right: right.to_owned(),
            },
          )
        } else if idx == left.len() {
          (
            (*left).to_owned(),
            Branch2 {
              size: middle.len() + right.len(),
              left: middle.to_owned(),
              middle: right.to_owned(),
            },
          )
        } else if idx - left.len() < middle.len() {
          let (cut_a, cut_b) = (*middle).to_owned().split(idx - left.len());
          (
            Branch2 {
              size: left.len() + cut_a.len(),
              left: left.to_owned(),
              middle: Arc::new(cut_a),
            },
            Branch2 {
              size: cut_b.len() + right.len(),
              left: Arc::new(cut_b),
              middle: right.to_owned(),
            },
          )
        } else if idx == left.len() + middle.len() {
          (
            Branch2 {
              size: left.len() + middle.len(),
              left: left.to_owned(),
              middle: middle.to_owned(),
            },
            (*right).to_owned(),
          )
        } else if idx >= size {
          unreachable!("Invalid split index end for a branch3: {}", idx)
        } else if idx - left.len() == middle.len() {
          (
            Branch2 {
              size: left.len() + middle.len(),
              left: left.to_owned(),
              middle: middle.to_owned(),
            },
            (*right).to_owned(),
          )
        } else {
          let (cut_a, cut_b) = (*right).to_owned().split(idx - left.len() - middle.len());

          (
            Branch3 {
              size: left.len() + middle.len() + cut_a.len(),
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(cut_a),
            },
            cut_b,
          )
        }
      }
    }
  }

  pub fn skip(&self, idx: usize) -> Result<Self, String> {
    self.slice(idx, self.len())
  }
  pub fn take(&self, idx: usize) -> Result<Self, String> {
    self.slice(0, idx)
  }

  pub fn reverse(&self) -> Self {
    match self {
      Leaf { .. } => self.to_owned(),
      Branch2 { left, middle, size } => Branch2 {
        size: *size,
        left: Arc::new(middle.reverse()),
        middle: Arc::new(left.reverse()),
      },
      Branch3 { left, middle, right, size } => Branch3 {
        size: *size,

        left: Arc::new(right.reverse()),
        middle: Arc::new(middle.reverse()),
        right: Arc::new(left.reverse()),
      },
    }
  }
  pub fn map<V>(&self, f: Arc<dyn Fn(&T) -> V>) -> TernaryTree<V> {
    match self {
      Leaf(value) => Leaf(Arc::new(f(value))),
      Branch2 { left, middle, size } => Branch2 {
        size: *size,
        left: Arc::new(left.map(f.clone())),
        middle: Arc::new(middle.map(f.clone())),
      },
      Branch3 { left, middle, right, size } => Branch3 {
        size: *size,
        left: Arc::new(left.map(f.clone())),
        middle: Arc::new(middle.map(f.clone())),
        right: Arc::new(right.map(f.clone())),
      },
    }
  }

  pub fn to_vec(&self) -> Vec<T> {
    let mut xs = Vec::with_capacity(self.len());
    for item in self {
      xs.push(item.to_owned());
    }
    xs
  }

  pub fn iter(&self) -> TernaryTreeRefIntoIterator<T> {
    TernaryTreeRefIntoIterator { value: self, index: 0 }
  }
}

impl<T> Display for TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TernaryTree[{}, ...]", self.len())
  }
}

// experimental code to turn `&TernaryTree<_>` into iterator
impl<'a, T> IntoIterator for &'a TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  type Item = &'a T;
  type IntoIter = TernaryTreeRefIntoIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    TernaryTreeRefIntoIterator { value: self, index: 0 }
  }
}

pub struct TernaryTreeRefIntoIterator<'a, T> {
  value: &'a TernaryTree<T>,
  index: usize,
}

impl<'a, T> Iterator for TernaryTreeRefIntoIterator<'a, T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
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

impl<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash> PartialEq for TernaryTree<T> {
  fn eq(&self, ys: &Self) -> bool {
    if self.len() != ys.len() {
      return false;
    }

    for idx in 0..ys.len() {
      if self.ref_get(idx) != ys.ref_get(idx) {
        return false;
      }
    }

    true
  }
}

impl<T> Eq for TernaryTree<T> where T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash {}

impl<T> PartialOrd for TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn cmp(&self, other: &Self) -> Ordering {
    if self.len() == other.len() {
      for idx in 0..self.len() {
        match self.ref_get(idx).cmp(other.ref_get(idx)) {
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

impl<T> Index<usize> for TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  type Output = T;

  fn index<'b>(&self, idx: usize) -> &Self::Output {
    // println!("get: {} {}", self.format_inline(), idx);
    self.ref_get(idx)
  }
}

impl<T> Hash for TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Leaf(value) => {
        value.hash(state);
      }
      Branch2 { left, middle, .. } => {
        left.hash(state);
        middle.hash(state);
      }
      Branch3 { left, middle, right, .. } => {
        left.hash(state);
        middle.hash(state);
        right.hash(state);
      }
    }
  }
}

/// internal function for mutable writing
fn write_leaves<T>(xs: &TernaryTree<T>, acc: &mut Vec<TernaryTree<T>>, counter: &Cell<usize>)
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  match xs {
    Leaf { .. } => {
      let idx = counter.take();
      acc.push(xs.to_owned());

      counter.replace(idx + 1);
    }
    Branch2 { left, middle, .. } => {
      write_leaves(left, acc, counter);
      write_leaves(middle, acc, counter);
    }
    Branch3 { left, middle, right, .. } => {
      write_leaves(left, acc, counter);
      write_leaves(middle, acc, counter);
      write_leaves(right, acc, counter);
    }
  }
}
