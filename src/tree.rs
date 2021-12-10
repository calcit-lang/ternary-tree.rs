use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::sync::Arc;

use crate::util::{divide_ternary_sizes, rough_int_pow};

#[derive(Clone, Debug)]
pub enum TernaryTree<T> {
  Leaf(Arc<T>),
  Branch2 {
    size: usize,
    depth: u16,
    left: Arc<TernaryTree<T>>,
    middle: Arc<TernaryTree<T>>,
  },
  Branch3 {
    size: usize,
    depth: u16,
    left: Arc<TernaryTree<T>>,
    middle: Arc<TernaryTree<T>>,
    right: Arc<TernaryTree<T>>,
  },
}

/// special mark for helping in simulating finger tree bahaviors to operate head tails
#[derive(Debug)]
pub enum FingerMark {
  Main(i8),
  Side(i8),
}

impl Default for FingerMark {
  fn default() -> Self {
    FingerMark::Main(2)
  }
}

use TernaryTree::*;

impl<'a, T> TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  /// just get, will not compute recursively
  pub fn get_depth(&self) -> u16 {
    match self {
      Leaf { .. } => 0,
      Branch2 { depth, .. } => *depth,
      Branch3 { depth, .. } => *depth,
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Leaf { .. } => false,
      Branch2 { size, .. } => *size == 0,
      Branch3 { size, .. } => *size == 0,
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Leaf { .. } => 1,
      Branch2 { size, .. } => *size,
      Branch3 { size, .. } => *size,
    }
  }

  /// make list again from existed
  /// use a factor to control side branches to be shallow with smaller depth
  /// root node has a factor of 2
  pub fn rebuild_list(size: usize, offset: usize, xs: &[TernaryTree<T>], mark: FingerMark) -> Self {
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
          depth: decide_parent_depth_2(left, middle),
        }
      }
      3 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        let right = &xs[offset + 2];
        Branch3 {
          size: left.len() + middle.len() + right.len(),
          left: Arc::new(left.to_owned()),
          middle: Arc::new(middle.to_owned()),
          right: Arc::new(right.to_owned()),
          depth: decide_parent_depth_3(left, middle, right),
        }
      }
      _ => {
        let (left, middle, right) = match mark {
          FingerMark::Main(factor) => {
            let side_capacity = rough_int_pow(3, factor as u16 - 1);
            if side_capacity * 2 < size {
              let divided = (side_capacity, size - side_capacity - side_capacity, side_capacity);

              let left = Self::rebuild_list(divided.0, offset, xs, FingerMark::Side(factor - 1));
              let middle = Self::rebuild_list(divided.1, offset + divided.0, xs, FingerMark::Main(factor + 1));
              let right = Self::rebuild_list(divided.2, offset + divided.0 + divided.1, xs, FingerMark::Side(factor - 1));
              (left, middle, right)
            } else {
              let divided = divide_ternary_sizes(size);

              let left = Self::rebuild_list(divided.0, offset, xs, FingerMark::Side(factor - 1));
              let middle = Self::rebuild_list(divided.1, offset + divided.0, xs, FingerMark::Side(factor - 1));
              let right = Self::rebuild_list(divided.2, offset + divided.0 + divided.1, xs, FingerMark::Side(factor - 1));
              (left, middle, right)
            }
          }
          FingerMark::Side(factor) => {
            let divided = divide_ternary_sizes(size);

            let left = Self::rebuild_list(divided.0, offset, xs, FingerMark::Side(factor - 1));
            let middle = Self::rebuild_list(divided.1, offset + divided.0, xs, FingerMark::Side(factor - 1));
            let right = Self::rebuild_list(divided.2, offset + divided.0 + divided.1, xs, FingerMark::Side(factor - 1));
            (left, middle, right)
          }
        };

        Branch3 {
          size: left.len() + middle.len() + right.len(),
          depth: decide_parent_depth_3(&left, &middle, &right),
          left: Arc::new(left),
          middle: Arc::new(middle),
          right: Arc::new(right),
        }
      }
    }
  }

  /// turn into a compare representation, with `_` for holes
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

  pub fn get(&self, idx: usize) -> Option<&T> {
    if self.is_empty() || idx >= self.len() {
      None
    } else {
      self.ref_get(idx)
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

      Branch2 { left, middle, .. } => {
        if let Some(pos) = left.find_index(f.clone()) {
          return Some(pos);
        }

        if let Some(pos) = middle.find_index(f.clone()) {
          return Some(pos + left.len() as i64);
        }

        None
      }

      Branch3 { left, middle, right, .. } => {
        if let Some(pos) = left.find_index(f.clone()) {
          return Some(pos);
        }

        if let Some(pos) = middle.find_index(f.clone()) {
          return Some(pos + left.len() as i64);
        }

        if let Some(pos) = right.find_index(f.clone()) {
          return Some(pos + (left.len() as i64) + (middle.len() as i64));
        }

        None
      }
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
      Branch2 { left, middle, .. } => {
        if let Some(pos) = left.index_of(item) {
          Some(pos)
        } else {
          middle.index_of(item).map(|pos| pos + left.len())
        }
      }
      Branch3 { left, middle, right, .. } => {
        if let Some(pos) = left.index_of(item) {
          Some(pos)
        } else if let Some(pos) = middle.index_of(item) {
          Some(pos + left.len())
        } else {
          right.index_of(item).map(|pos| pos + left.len() + middle.len())
        }
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
        Branch2 { left, middle, .. },
        Branch2 {
          left: left2,
          middle: middle2,
          ..
        },
      ) => left.is_shape_same(left2) && middle.is_shape_same(middle2),
      (
        Branch3 { left, middle, right, .. },
        Branch3 {
          left: left2,
          middle: middle2,
          right: right2,
          ..
        },
      ) => left.is_shape_same(left2) && middle.is_shape_same(middle2) && right.is_shape_same(right2),

      (_, _) => false,
    }
  }

  /// internal usages for rebuilding tree
  fn to_leaves(&self) -> Vec<TernaryTree<T>> {
    let mut acc: Vec<TernaryTree<T>> = Vec::with_capacity(self.len());
    let counter: RefCell<usize> = RefCell::new(0);
    write_leaves(self, &mut acc, &counter);
    assert_eq!(acc.len(), self.len());
    acc
  }

  pub fn ref_get(&self, idx: usize) -> Option<&T> {
    // println!("get: {} {}", self.format_inline(), idx);
    // if idx >= self.len() {
    //   println!("get from out of bound: {} {}", idx, self.len());
    //   return None;
    // }
    match self {
      Branch3 { left, middle, right, .. } => {
        let base = left.len();
        if idx < base {
          left.ref_get(idx)
        } else if idx < base + middle.len() {
          middle.ref_get(idx - base)
        } else {
          right.ref_get(idx - left.len() - middle.len())
        }
      }
      Branch2 { left, middle, .. } => {
        if idx < left.len() {
          left.ref_get(idx)
        } else {
          middle.ref_get(idx - left.len())
        }
      }
      Leaf(value) => Some(value),
    }
  }

  /// get via go down the branch with a mutable loop
  pub fn loop_get(&self, original_idx: usize) -> Option<T> {
    let mut tree_parent = self.to_owned();
    let mut idx = original_idx;
    loop {
      match tree_parent {
        Leaf(value) => {
          if idx == 0 {
            return Some((*value).to_owned());
          } else {
            println!("[warning] Cannot get from leaf with index {}", idx);
            return None;
          }
        }
        Branch2 { left, middle, size, .. } => {
          if idx > size - 1 {
            println!("[warning] Index too large at {} from {}", idx, size);
            return None;
          }

          if left.len() + middle.len() != size {
            unreachable!("tree.size does not match sum case branch sizes");
          }

          if idx < left.len() {
            tree_parent = (*left).to_owned();
          } else {
            tree_parent = (*middle).to_owned();
            idx -= left.len();
          }
        }
        Branch3 {
          left, middle, right, size, ..
        } => {
          if idx > size - 1 {
            println!("[warning] Index too large at {} from {}", idx, size);
            return None;
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
  }

  pub fn first(&self) -> Option<&T> {
    if self.is_empty() {
      None
    } else {
      self.ref_get(0)
    }
  }

  pub fn last(&self) -> Option<&T> {
    if self.is_empty() {
      None
    } else {
      self.ref_get(self.len() - 1)
    }
  }
  pub fn assoc(&self, idx: usize, item: T) -> Result<Self, String> {
    if idx > self.len() - 1 {
      return Err(format!("Index too large {} for {}", idx, self.format_inline()));
    }

    match self {
      Leaf { .. } => {
        if idx == 0 {
          Ok(Leaf(Arc::new(item)))
        } else {
          Err(format!("Cannot assoc leaf into index {}", idx))
        }
      }
      Branch2 { left, middle, size, .. } => {
        if left.len() + middle.len() != *size {
          return Err(format!(
            "tree size {} does not match sum case branch sizes, {}",
            size,
            self.format_inline()
          ));
        }

        if idx < left.len() {
          let changed_branch = left.assoc(idx, item)?;
          Ok(Branch2 {
            size: size.to_owned(),
            depth: decide_parent_depth_2(&changed_branch, middle),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
          })
        } else {
          let changed_branch = middle.assoc(idx - left.len(), item)?;
          Ok(Branch2 {
            size: size.to_owned(),
            depth: decide_parent_depth_2(left, &changed_branch),
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
            "tree size {} does not match sum case branch sizes, {}",
            size,
            self.format_inline()
          ));
        }

        if idx < left.len() {
          let changed_branch = left.assoc(idx, item)?;
          Ok(Branch3 {
            size: size.to_owned(),
            depth: decide_parent_depth_3(&changed_branch, middle, right),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          })
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.assoc(idx - left.len(), item)?;
          Ok(Branch3 {
            size: size.to_owned(),
            depth: decide_parent_depth_3(left, &changed_branch, right),
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
            right: right.to_owned(),
          })
        } else {
          let changed_branch = right.assoc(idx - left.len() - middle.len(), item)?;
          Ok(Branch3 {
            size: size.to_owned(),
            depth: decide_parent_depth_3(left, middle, &changed_branch),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch.to_owned()),
          })
        }
      }
    }
  }
  pub fn dissoc(&self, idx: usize) -> Result<Self, String> {
    if self.is_empty() {
      return Err(String::from("Cannot remove from empty list"));
    }

    if idx > self.len() - 1 {
      return Err(format!("Index too large {} for {}", idx, self.len()));
    } else if self.len() == 1 {
      // idx already == 0
      return Err(String::from("Cannot remove from singleton list"));
    }

    match self {
      Leaf { .. } => unreachable!("dissoc should be handled at branches"),
      Branch2 { left, middle, size, .. } => {
        if left.len() + middle.len() != *size {
          return Err(format!(
            "tree {} does not match sum from branch sizes {}",
            self.format_inline(),
            self.len()
          ));
        }

        if idx < left.len() {
          if left.len() == 1 {
            Ok((**middle).to_owned())
          } else {
            let changed_branch = left.dissoc(idx)?;
            Ok(Branch2 {
              size: *size - 1,
              depth: decide_parent_depth_2(&changed_branch, middle),
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
            })
          }
        } else if left.len() == 1 {
          Ok((**left).to_owned())
        } else {
          let changed_branch = middle.dissoc(idx - left.len())?;
          Ok(Branch2 {
            size: *size - 1,
            depth: decide_parent_depth_2(left, &changed_branch),
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
              depth: decide_parent_depth_2(middle, right),
              left: middle.to_owned(),
              middle: right.to_owned(),
            })
          } else {
            let changed_branch = left.dissoc(idx)?;
            Ok(Branch3 {
              size: *size - 1,
              depth: decide_parent_depth_3(&changed_branch, middle, right),
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
              right: right.to_owned(),
            })
          }
        } else if idx < left.len() + middle.len() {
          if middle.len() == 1 {
            Ok(Branch2 {
              size: *size - 1,
              depth: decide_parent_depth_2(left, right),
              left: left.to_owned(),
              middle: right.to_owned(),
            })
          } else {
            let changed_branch = middle.dissoc(idx - left.len())?;
            Ok(Branch3 {
              size: *size - 1,
              depth: decide_parent_depth_3(left, &changed_branch, right),
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
              right: right.to_owned(),
            })
          }
        } else if right.len() == 1 {
          Ok(Branch2 {
            size: *size - 1,
            depth: decide_parent_depth_2(left, middle),
            left: left.to_owned(),
            middle: middle.to_owned(),
          })
        } else {
          let changed_branch = right.dissoc(idx - left.len() - middle.len())?;
          Ok(Branch3 {
            size: *size - 1,
            depth: decide_parent_depth_3(left, middle, &changed_branch),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch.to_owned()),
          })
        }
      }
    }
  }
  pub fn rest(&self) -> Result<Self, String> {
    if self.is_empty() {
      Err(String::from("calling rest on empty"))
    } else {
      self.dissoc(0)
    }
  }
  pub fn butlast(&self) -> Result<Self, String> {
    if self.is_empty() {
      Err(String::from("calling butlast on empty"))
    } else {
      self.dissoc(self.len() - 1)
    }
  }

  pub fn insert(&self, idx: usize, item: T, after: bool) -> Result<Self, String> {
    match self {
      Leaf { .. } => {
        if after {
          Ok(Branch2 {
            depth: 1,
            size: 2,
            left: Arc::new(self.to_owned()),
            middle: Arc::new(Leaf(Arc::new(item))),
          })
        } else {
          Ok(Branch2 {
            depth: 1,
            size: 2,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: Arc::new(self.to_owned()),
          })
        }
      }

      Branch2 {
        left, middle, size, depth, ..
      } => {
        if self.len() == 1 {
          if after {
            // in compact mode, values placed at left
            return Ok(Branch2 {
              size: 2,
              depth: 1,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
            });
          } else {
            return Ok(Branch2 {
              size: 2,
              depth: 1,
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
                depth: 1,
                left: left.to_owned(),
                middle: Arc::new(Leaf(Arc::new(item))),
                right: middle.to_owned(),
              });
            }
            if idx == 1 {
              return Ok(Branch3 {
                size: 3,
                depth: 1,
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
              depth: 1,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
              right: middle.to_owned(),
            });
          } else if idx == 1 {
            return Ok(Branch3 {
              size: 3,
              depth: 1,
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
            depth: *depth,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: left.to_owned(),
            right: middle.to_owned(),
          });
        }

        if idx == *size - 1 && after {
          return Ok(Branch3 {
            size: *size + 1,
            depth: *depth,
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(Leaf(Arc::new(item))),
          });
        }

        if idx < left.len() {
          let changed_branch = left.insert(idx, item, after)?;
          Ok(Branch2 {
            size: *size + 1,
            depth: decide_parent_depth_2(&changed_branch, middle),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
          })
        } else {
          let changed_branch = middle.insert(idx - left.len(), item, after)?;

          Ok(Branch2 {
            size: *size + 1,
            depth: decide_parent_depth_2(left, &changed_branch.to_owned()),
            left: left.to_owned(),
            middle: Arc::new(changed_branch),
          })
        }
      }
      Branch3 {
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
            return Ok(Branch2 {
              size: 2,
              depth: 1,
              left: left.to_owned(),
              middle: Arc::new(Leaf(Arc::new(item))),
            });
          } else {
            return Ok(Branch2 {
              size: 2,
              depth: 1,
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
                depth: 1,
                left: left.to_owned(),
                middle: Arc::new(Leaf(Arc::new(item))),
                right: middle.to_owned(),
              });
            }
            if idx == 1 {
              return Ok(Branch3 {
                size: 3,
                depth: 1,
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
              depth: 1,
              left: Arc::new(Leaf(Arc::new(item))),
              middle: left.to_owned(),
              right: middle.to_owned(),
            });
          } else if idx == 1 {
            return Ok(Branch3 {
              size: 3,
              depth: 1,
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
            depth: depth + 1,
            left: Arc::new(Leaf(Arc::new(item))),
            middle: Arc::new(self.to_owned()),
          });
        }

        if idx == *size - 1 && after && right.len() >= middle.len() && right.len() >= left.len() {
          return Ok(Branch2 {
            size: *size + 1,
            depth: depth + 1,
            left: Arc::new(self.to_owned()),
            middle: Arc::new(Leaf(Arc::new(item))),
          });
        }

        if after && idx == *size - 1 && right.len() == 0 && middle.len() >= left.len() {
          return Ok(Branch3 {
            size: *size + 1,
            depth: depth.to_owned(),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(Leaf(Arc::new(item))),
          });
        }

        if !after && idx == 0 && right.len() == 0 && middle.len() >= right.len() {
          return Ok(Branch3 {
            size: *size + 1,
            depth: depth.to_owned(),
            left: Arc::new(Leaf(Arc::new(item))),
            middle: left.to_owned(),
            right: middle.to_owned(),
          });
        }

        if idx < left.len() {
          let changed_branch = left.insert(idx, item, after)?;
          Ok(Branch3 {
            size: *size + 1,
            depth: decide_parent_depth_3(&changed_branch, middle, right),
            left: Arc::new(changed_branch.to_owned()),
            middle: middle.to_owned(),
            right: right.to_owned(),
          })
        } else if idx < left.len() + middle.len() {
          let changed_branch = middle.insert(idx - left.len(), item, after)?;

          Ok(Branch3 {
            size: *size + 1,
            depth: decide_parent_depth_3(left, &changed_branch.to_owned(), right),
            left: left.to_owned(),
            middle: Arc::new(changed_branch.to_owned()),
            right: right.to_owned(),
          })
        } else {
          let changed_branch = right.insert(idx - left.len() - middle.len(), item, after)?;

          Ok(Branch3 {
            size: *size + 1,
            depth: decide_parent_depth_3(left, middle, &changed_branch),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch.to_owned()),
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
    *self = Self::rebuild_list(ys.len(), 0, &ys, FingerMark::default());
    Ok(())
  }
  // TODO, need better strategy for detecting
  pub fn maybe_reblance(&mut self) -> Result<(), String> {
    match self {
      Leaf(..) => Ok(()),
      Branch2 { .. } => Ok(()),
      Branch3 { size, .. } => {
        // guessed number
        if *size < 81 {
          Ok(())
        } else {
          let current_depth = self.get_depth();
          if current_depth > 20 && rough_int_pow(3, current_depth - 20) > self.len() {
            self.force_inplace_balancing()
          } else {
            Ok(())
          }
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

    let mut result = match self.insert(0, item, false) {
      Ok(v) => v,
      Err(e) => unreachable!(e),
    };

    if !disable_balancing {
      if let Err(msg) = result.maybe_reblance() {
        println!("[warning] {}", msg)
      }
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
    let mut result = match self.insert(self.len() - 1, item, true) {
      Ok(v) => v,
      Err(e) => unreachable!(e),
    };

    if !disable_balancing {
      if let Err(msg) = result.maybe_reblance() {
        println!("[warning] {}", msg)
      }
    }
    result
  }

  fn push_right_iter(&self, item: Self, mark: FingerMark) -> Self {
    // println!("    iter: {} {:?}", self.format_inline(), mark);
    match mark {
      FingerMark::Main(n) => match self {
        Leaf(a) => Branch2 {
          size: 1 + item.len(),
          depth: item.get_depth() + 1,
          left: Arc::new(Leaf(a.to_owned())),
          middle: Arc::new(item),
        },
        Branch2 { size, left, middle, .. } => {
          if middle.len() + item.len() > left.len() || middle.len() >= triple_size(n) {
            Branch3 {
              size: size + item.len(),
              depth: decide_parent_depth_3(left, middle, &item),
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(item),
            }
          } else {
            let item_size = item.len();
            let changed_branch = middle.push_right_iter(item, FingerMark::Side(n - 1));

            Branch2 {
              size: size + item_size,
              depth: decide_parent_depth_2(left, &changed_branch),
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
            }
          }
        }
        Branch3 {
          size, left, middle, right, ..
        } => {
          if right.len() + item.len() > middle.len() || middle.len() >= triple_size(n) {
            Branch2 {
              size: size + item.len(),
              depth: decide_parent_depth_2(self, &item),
              left: Arc::new(self.to_owned()),
              middle: Arc::new(item),
            }
          } else {
            let changed_branch = right.push_right_iter(item, FingerMark::Side(n - 1));
            Branch3 {
              size: size + 1,
              depth: decide_parent_depth_3(left, middle, &changed_branch),
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(changed_branch),
            }
          }
        }
      },
      FingerMark::Side(n) => match self {
        Leaf(a) => Branch2 {
          size: 1 + item.len(),
          depth: item.get_depth() + 1,
          left: Arc::new(Leaf(a.to_owned())),
          middle: Arc::new(item),
        },
        Branch2 { size, left, middle, .. } => {
          if middle.len() + item.len() > left.len() {
            Branch3 {
              size: size + item.len(),
              depth: decide_parent_depth_3(left, middle, &item),
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(item),
            }
          } else {
            let changed_branch = middle.push_right_iter(item.to_owned(), FingerMark::Side(n - 1));
            Branch2 {
              size: size + item.len(),
              depth: decide_parent_depth_3(left, middle, &changed_branch),
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
            }
          }
        }
        Branch3 {
          size, left, middle, right, ..
        } => {
          if right.len() + item.len() > middle.len() {
            Branch2 {
              size: size + item.len(),
              depth: decide_parent_depth_2(self, &item),
              left: Arc::new(self.to_owned()),
              middle: Arc::new(item),
            }
          } else {
            let changed_branch = right.push_right_iter(item.to_owned(), FingerMark::Side(n - 1));
            Branch3 {
              size: size + item.len(),
              depth: decide_parent_depth_3(left, middle, &changed_branch),
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(changed_branch),
            }
          }
        }
      },
    }
  }

  pub fn push_right(&self, item: T) -> Self {
    // start with 2 so its left child branch has capability of only 3^1
    self.push_right_iter(Leaf(Arc::new(item)), FingerMark::default())
  }

  pub fn drop_left(&self) -> Self {
    match self {
      Leaf(_) => {
        unreachable!("not expected empty node inside tree")
      }
      Branch2 { size, left, middle, .. } => {
        if left.len() == 1 {
          (**middle).to_owned()
        } else {
          let changed_branch = left.drop_left();
          match changed_branch {
            Branch2 {
              left: b_left,
              middle: b_middle,
              ..
            } => Branch3 {
              size: size - 1,
              depth: decide_parent_depth_3(&b_left, &b_middle, middle),
              left: b_left.to_owned(),
              middle: b_middle.to_owned(),
              right: middle.to_owned(),
            },
            Branch3 {
              left: b_left,
              middle: b_middle,
              right: b_right,
              ..
            } => {
              let internal_branch = Branch2 {
                size: b_middle.len() + b_right.len(),
                depth: decide_parent_depth_2(&b_middle, &b_right),
                left: b_middle,
                middle: b_right,
              };
              Branch3 {
                size: size - 1,
                depth: decide_parent_depth_3(&b_left, &internal_branch, middle),
                left: b_left.to_owned(),
                middle: Arc::new(internal_branch),
                right: middle.to_owned(),
              }
            }
            _ => Branch2 {
              size: size - 1,
              depth: decide_parent_depth_2(&changed_branch, middle),
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
            },
          }
        }
      }
      Branch3 {
        size, left, middle, right, ..
      } => {
        if left.len() == 1 {
          match &**middle {
            Branch2 {
              left: b_left,
              middle: b_middle,
              ..
            } => Branch3 {
              size: size - 1,
              depth: decide_parent_depth_3(b_left, b_middle, right),
              left: b_left.to_owned(),
              middle: b_middle.to_owned(),
              right: right.to_owned(),
            },
            Branch3 {
              left: b_left,
              middle: b_middle,
              right: b_right,
              ..
            } => {
              let internal_branch = Branch2 {
                size: b_middle.len() + b_right.len(),
                depth: decide_parent_depth_2(b_middle, b_right),
                left: b_middle.to_owned(),
                middle: b_right.to_owned(),
              };
              Branch3 {
                size: size - 1,
                depth: decide_parent_depth_3(b_left, &internal_branch, right),
                left: b_left.to_owned(),
                middle: Arc::new(internal_branch),
                right: right.to_owned(),
              }
            }
            _ => Branch2 {
              size: size - 1,
              depth: decide_parent_depth_2(middle, right),
              left: middle.to_owned(),
              middle: right.to_owned(),
            },
          }
        } else {
          let changed_branch = left.drop_left();
          Branch3 {
            size: size - 1,
            depth: decide_parent_depth_3(&changed_branch, middle, right),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          }
        }
      }
    }
  }

  pub fn concat(raw: &[TernaryTree<T>]) -> Self {
    let mut xs_groups: Vec<TernaryTree<T>> = Vec::with_capacity(raw.len());
    for x in raw {
      if !x.is_empty() {
        xs_groups.push(x.to_owned());
      }
    }
    match xs_groups.len() {
      0 => unreachable!("does not work with empty list in ternary-tree"),
      1 => xs_groups[0].to_owned(),
      2 => {
        let left = xs_groups[0].to_owned();
        let middle = xs_groups[1].to_owned();
        TernaryTree::Branch2 {
          size: left.len() + middle.len(),
          depth: decide_parent_depth_2(&left, &middle),
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
          depth: decide_parent_depth_3(&left, &middle, &right),
          left: Arc::new(left),
          middle: Arc::new(middle),
          right: Arc::new(right),
        }
      }
      _ => {
        let mut ys: Vec<TernaryTree<T>> = vec![];
        for x in xs_groups {
          if !x.is_empty() {
            ys.push(x.to_owned())
          }
        }
        let mut result = Self::rebuild_list(ys.len(), 0, &ys, FingerMark::default());
        if let Err(msg) = result.maybe_reblance() {
          println!("[warning] {}", msg)
        }
        result
      }
    }
  }
  pub fn check_structure(&self) -> Result<(), String> {
    if self.is_empty() {
      Ok(())
    } else {
      match self {
        Leaf { .. } => Ok(()),
        Branch2 { left, middle, size, depth } => {
          if !self.is_empty() && *size == 0 {
            return Err(String::from("branch but has size"));
          }

          if *size != left.len() + middle.len() {
            return Err(format!("Bad size at branch {}", self.format_inline()));
          }

          if *depth != decide_parent_depth_2(left, middle) {
            return Err(format!("Bad depth at branch {}", self.format_inline()));
          }

          left.check_structure()?;
          middle.check_structure()?;

          Ok(())
        }
        Branch3 {
          left,
          middle,
          right,
          size,
          depth,
        } => {
          if !self.is_empty() && *size == 0 {
            return Err(String::from("branch but has size"));
          }

          if *size != left.len() + middle.len() + right.len() {
            return Err(format!("Bad size at branch {}", self.format_inline()));
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
  pub fn slice(&self, start_idx: usize, end_idx: usize) -> Result<Self, String> {
    // echo "slice {tree.formatListInline}: {start_idx}..{end_idx}"
    if end_idx > self.len() {
      return Err(format!("Slice range too large {} for {}", end_idx, self.format_inline()));
    }
    if start_idx > end_idx {
      return Err(format!("Invalid slice range {}..{} for {}", start_idx, end_idx, self));
    }
    if start_idx == end_idx {
      return Err(format!("Slice range empty {}..{} for {}", start_idx, end_idx, self));
    }

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
            depth: decide_parent_depth_2(&left_cut, &middle_cut),
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
              depth: decide_parent_depth_2(&middle_cut, &right_cut),
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
            depth: decide_parent_depth_2(&left_cut, &middle_cut),
            left: Arc::new(left_cut),
            middle: Arc::new(middle_cut),
          })
        } else {
          let left_cut = left.slice(start_idx, base1)?;
          let right_cut = right.slice(0, end_idx - base2)?;
          Ok(TernaryTree::Branch3 {
            size: left_cut.len() + middle.len() + right_cut.len(),
            depth: decide_parent_depth_3(&left_cut, middle, &right_cut),
            left: Arc::new(left_cut),
            middle: middle.clone(),
            right: Arc::new(right_cut),
          })
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
    if self.is_empty() {
      unreachable!("don't call reverse on empty in ternary-tree");
    }

    match self {
      Leaf { .. } => self.to_owned(),
      Branch2 { left, middle, size, depth } => Branch2 {
        size: *size,
        depth: *depth,
        left: Arc::new(middle.reverse()),
        middle: Arc::new(left.reverse()),
      },
      Branch3 {
        left,
        middle,
        right,
        size,
        depth,
      } => Branch3 {
        size: *size,
        depth: *depth,
        left: Arc::new(right.reverse()),
        middle: Arc::new(middle.reverse()),
        right: Arc::new(left.reverse()),
      },
    }
  }
  pub fn map<V>(&self, f: Arc<dyn Fn(&T) -> V>) -> TernaryTree<V> {
    match self {
      Leaf(value) => Leaf(Arc::new(f(value))),
      Branch2 { left, middle, size, depth } => Branch2 {
        size: *size,
        depth: *depth,
        left: Arc::new(left.map(f.clone())),
        middle: Arc::new(middle.map(f.clone())),
      },
      Branch3 {
        left,
        middle,
        right,
        size,
        depth,
      } => Branch3 {
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
    for item in self {
      xs.push(item.to_owned());
    }

    xs
  }

  pub fn iter(&self) -> TernaryTreeRefIntoIterator<T> {
    TernaryTreeRefIntoIterator { value: self, index: 0 }
  }
}

// pass several children here
fn decide_parent_depth_2<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash>(
  x0: &TernaryTree<T>,
  x1: &TernaryTree<T>,
) -> u16 {
  cmp::max(x0.get_depth(), x1.get_depth()) + 1
}

// pass several children here
fn decide_parent_depth_3<T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash>(
  x0: &TernaryTree<T>,
  x1: &TernaryTree<T>,
  x2: &TernaryTree<T>,
) -> u16 {
  // println!("{} {} {}", x0.get_depth(), x1.get_depth(), x2.get_depth());
  cmp::max(cmp::max(x0.get_depth(), x1.get_depth()), x2.get_depth()) + 1
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
      ret
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
        match self.ref_get(idx).cmp(&other.ref_get(idx)) {
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
    self.ref_get(idx).expect("get from list")
  }
}

impl<T> Hash for TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    "ternary".hash(state);
    match self {
      Leaf(value) => {
        "leaf".hash(state);
        value.hash(state);
      }
      Branch2 { left, middle, .. } => {
        "branch".hash(state);
        left.hash(state);
        middle.hash(state);
      }
      Branch3 { left, middle, right, .. } => {
        "branch".hash(state);
        left.hash(state);
        middle.hash(state);
        right.hash(state);
      }
    }
  }
}

/// internal function for mutable writing
fn write_leaves<T>(xs: &TernaryTree<T>, acc: &mut Vec<TernaryTree<T>>, counter: &RefCell<usize>)
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  if xs.is_empty() {
    return;
  }

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

fn triple_size(t: i8) -> usize {
  let n: usize = 3;
  n.pow(t as u32)
}
