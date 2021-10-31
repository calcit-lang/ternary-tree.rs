mod util;

use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;

use util::{divide_ternary_sizes, rough_int_pow};

#[derive(Clone)]
pub enum TernaryTreeList<T> {
  Branch {
    size: usize,
    depth: usize,
    left: Option<Arc<TernaryTreeList<T>>>,
    middle: Option<Arc<TernaryTreeList<T>>>,
    right: Option<Arc<TernaryTreeList<T>>>,
  },
  Leaf {
    value: T,
    size: usize,
  },
}

// RefInt = {
//  value: number;
// };

use TernaryTreeList::*;

impl<T: Clone + fmt::Display + Eq + PartialEq> TernaryTreeList<T> {
  /// just get, will not compute recursively
  pub fn get_depth(&self) -> usize {
    match self {
      Branch { depth, .. } => depth.to_owned(),
      Leaf { .. } => 1,
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Branch {
        left,
        middle,
        right,
        ..
      } => left.is_none() && middle.is_none() && right.is_none(),
      Leaf { .. } => false,
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Branch { size, .. } => size.to_owned(),
      Leaf { size, .. } => size.to_owned(),
    }
  }

  // make list again
  pub fn make_list(size: usize, offset: usize, xs: &[TernaryTreeList<T>]) -> Self {
    match size {
      0 => Branch {
        size: 0,
        depth: 1,
        left: None,
        middle: None,
        right: None,
      },
      1 => xs[offset].to_owned(),
      2 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        Branch {
          size: left.len() + middle.len(),
          left: Some(Arc::new(left.to_owned())),
          middle: Some(Arc::new(middle.to_owned())),
          right: None,
          depth: decide_parent_depth(&[left, middle]),
        }
      }
      3 => {
        let left = &xs[offset];
        let middle = &xs[offset + 1];
        let right = &xs[offset + 2];
        Branch {
          size: left.len() + middle.len() + right.len(),
          left: Some(Arc::new(left.to_owned())),
          middle: Some(Arc::new(middle.to_owned())),
          right: Some(Arc::new(right.to_owned())),
          depth: decide_parent_depth(&[left, middle, right]),
        }
      }
      _ => {
        let divided = divide_ternary_sizes(size);

        let left = Self::make_list(divided.0, offset, xs);
        let middle = Self::make_list(divided.1, offset + divided.0, xs);
        let right = Self::make_list(divided.2, offset + divided.0 + divided.1, xs);
        Branch {
          size: left.len() + middle.len() + right.len(),
          depth: decide_parent_depth(&[&left, &middle, &right]),
          left: Some(Arc::new(left)),
          middle: Some(Arc::new(middle)),
          right: Some(Arc::new(right)),
        }
      }
    }
  }
  pub fn init_from(xs: &[T]) -> Self {
    let mut ys: Vec<Self> = Vec::with_capacity(xs.len());
    for x in xs {
      ys.push(Leaf {
        size: 1,
        value: x.to_owned(),
      })
    }

    Self::make_list(xs.len(), 0, &ys)
  }

  pub fn init_empty() -> Self {
    Branch {
      size: 0,
      depth: 1,
      middle: None,
      left: None,
      right: None,
    }
  }
  pub fn is_leaf(self) -> bool {
    matches!(self, Leaf { .. })
  }

  pub fn is_branch(&self) -> bool {
    matches!(self, Branch { .. })
  }

  pub fn format_inline(&self) -> String {
    match self {
      Leaf { value, .. } => value.to_string(),
      Branch {
        left,
        middle,
        right,
        ..
      } => {
        let left_text = match left {
          Some(x) => x.format_inline(),
          None => "_".to_owned(), // TODO actually inline more
        };
        let middle_text = match middle {
          Some(x) => x.format_inline(),
          None => "_".to_owned(),
        };
        let right_text = match right {
          Some(x) => x.format_inline(),
          None => "_".to_owned(),
        };
        // TODO maybe need more informations here
        format!("({} {} {}", left_text, middle_text, right_text)
      }
    }
  }

  pub fn get(&self, idx: usize) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      Some(self.unsafe_get(idx))
    }
  }

  // returns -1 if (not foun)
  pub fn find_index(&self, f: Arc<dyn Fn(&T) -> bool>) -> i64 {
    if self.is_empty() {
      return -1;
    }
    match self {
      Leaf { value, .. } => {
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
        let mut left_size = 0;
        let mut middle_size = 0;
        if let Some(br) = left {
          left_size = br.len();
          let v = br.find_index(f.clone());
          if v >= 0 {
            return v;
          }
        }
        if let Some(br) = middle {
          middle_size = br.len();
          let v = br.find_index(f.clone());
          if v >= 0 {
            return v + left_size as i64;
          }
        }
        if let Some(br) = right {
          let v = br.find_index(f.clone());
          if v >= 0 {
            return v + (left_size as i64) + (middle_size as i64);
          }
        }

        -1
      }
    }
  }
  // returns -1 if (not foun)
  pub fn index_of(&self, item: &T) -> i64 {
    if self.is_empty() {
      return -1;
    }
    match self {
      Leaf { value, .. } => {
        if item == value {
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
        let mut left_size = 0;
        let mut middle_size = 0;
        if let Some(br) = left {
          left_size = br.len();
          let v = br.index_of(item);
          if v >= 0 {
            return v;
          }
        }
        if let Some(br) = middle {
          middle_size = br.len();
          let v = br.index_of(item);
          if v >= 0 {
            return v + left_size as i64;
          }
        }

        if let Some(br) = right {
          let v = br.index_of(item);
          if v >= 0 {
            return v + left_size as i64 + middle_size as i64;
          }
        }
        -1
      }
    }
  }

  pub fn same_shape(xs: &Self, ys: &Self) -> bool {
    if xs.is_empty() {
      return ys.is_empty();
    }

    if ys.is_empty() {
      return false;
    }

    if xs.len() != ys.len() {
      return false;
    }

    if xs.get_depth() != ys.get_depth() {
      return false;
    }

    match (xs, ys) {
      (Leaf { value, .. }, Leaf { value: v2, .. }) => value == v2,

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

  fn write_leaves(&self, acc: /* var */ &mut [TernaryTreeList<T>], counter: &RefCell<usize>) {
    if self.is_empty() {
      return;
    }

    match self {
      Leaf { .. } => {
        let idx = counter.take();
        acc[idx] = self.to_owned();

        counter.replace(idx + 1);
      }
      Branch {
        left,
        middle,
        right,
        ..
      } => {
        if let Some(br) = left {
          Self::write_leaves(br, acc, counter);
        }
        if let Some(br) = middle {
          Self::write_leaves(br, acc, counter);
        }
        if let Some(br) = right {
          Self::write_leaves(br, acc, counter);
        }
      }
    }
  }

  pub fn to_leaves(&mut self) -> Vec<TernaryTreeList<T>> {
    let mut acc: Vec<TernaryTreeList<T>> = Vec::with_capacity(self.len());
    let counter: RefCell<usize> = RefCell::new(5);
    Self::write_leaves(self, &mut acc, &counter);
    acc
  }

  pub fn unsafe_get(&self, original_idx: usize) -> T {
    let mut tree_parent = Some(Arc::new((*self).to_owned()));
    let mut idx = original_idx;
    while let Some(tree) = tree_parent {
      match &*tree {
        Leaf { value, .. } => {
          if idx == 0 {
            return value.to_owned();
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
          let left_size = match &left {
            Some(br) => br.len(),
            None => 0,
          };
          let middle_size = match &middle {
            Some(br) => br.len(),
            None => 0,
          };
          let right_size = match &right {
            Some(br) => br.len(),
            None => 0,
          };

          if left_size + middle_size + right_size != *size {
            unreachable!("tree.size does not match sum case branch sizes");
          }

          if idx < left_size {
            tree_parent = left.to_owned();
          } else if idx < left_size + middle_size {
            tree_parent = middle.to_owned();
            idx -= left_size;
          } else {
            tree_parent = right.to_owned();
            idx -= left_size + middle_size;
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
      Leaf { value, .. } => {
        if idx == 0 {
          Leaf {
            size: 1,
            value: value.to_owned(),
          }
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
        let left_size = match left {
          Some(br) => br.len(),
          None => 0,
        };
        let middle_size = match middle {
          Some(br) => br.len(),
          None => 0,
        };
        let right_size = match right {
          Some(br) => br.len(),
          None => 0,
        };

        if left_size + middle_size + right_size != *size {
          unreachable!("tree.size does not match sum case branch sizes");
        }

        if idx < left_size {
          match left {
            Some(br) => {
              let changed_branch = Arc::new(br.assoc(idx, item));
              Branch {
                size: size.to_owned(),
                depth: decide_parent_depth_op(&[
                  &Some(changed_branch.to_owned()),
                  &*middle,
                  &*right,
                ]),
                left: Some(changed_branch.to_owned()),
                middle: middle.to_owned(),
                right: right.to_owned(),
              }
            }
            None => unreachable!("expected data in left branch"),
          }
        } else if idx < left_size + middle_size {
          match middle {
            Some(br) => {
              let changed_branch = Arc::new(br.assoc(idx - left_size, item));
              Branch {
                size: size.to_owned(),
                depth: decide_parent_depth_op(&[&*left, &Some(changed_branch.to_owned()), &*right]),
                left: left.to_owned(),
                middle: Some(changed_branch.to_owned()),
                right: right.to_owned(),
              }
            }
            None => unreachable!("expected data in middle branch"),
          }
        } else {
          match right {
            Some(br) => {
              let changed_branch = Arc::new(br.assoc(idx - left_size - middle_size, item));
              Branch {
                size: size.to_owned(),
                depth: decide_parent_depth_op(&[
                  &*left,
                  &*middle,
                  &Some(changed_branch.to_owned()),
                ]),
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: Some(changed_branch.to_owned()),
              }
            }
            None => unreachable!("expected data in right branch"),
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
      return Branch {
        size: 0,
        depth: 1,
        left: None,
        middle: None,
        right: None,
      };
    }

    match self {
      Leaf { .. } => unreachable!("dissoc should be handled at branches"),
      Branch {
        left,
        middle,
        right,
        size,
        ..
      } => {
        let left_size = match left {
          Some(br) => br.len(),
          None => 0,
        };
        let middle_size = match middle {
          Some(br) => br.len(),
          None => 0,
        };
        let right_size = match right {
          Some(br) => br.len(),
          None => 0,
        };

        if left_size + middle_size + right_size != *size {
          unreachable!("tree.size does not match sum from branch sizes");
        }

        let result: Self;

        if idx < left_size {
          match &left {
            Some(br) => {
              let changed_branch = br.dissoc(idx);
              result = if changed_branch.is_empty() {
                Branch {
                  size: *size - 1,
                  depth: decide_parent_depth_op(&[middle, right]),
                  left: middle.to_owned(),
                  middle: right.to_owned(),
                  right: Some(Arc::new(make_empty_node())),
                }
              } else {
                Branch {
                  size: *size - 1,
                  depth: decide_parent_depth_op(&[
                    &Some(Arc::new(changed_branch.to_owned())),
                    middle,
                    right,
                  ]),
                  left: Some(Arc::new(changed_branch.to_owned())),
                  middle: middle.to_owned(),
                  right: right.to_owned(),
                }
              }
            }
            None => unreachable!("expected data in left branch"),
          }
        } else if idx < left_size + middle_size {
          match &middle {
            Some(br) => {
              let changed_branch = br.dissoc(idx - left_size);
              result = if changed_branch.is_empty() {
                Branch {
                  size: *size - 1,
                  depth: decide_parent_depth_op(&[left, &Some(Arc::new(changed_branch)), right]),
                  left: left.to_owned(),
                  middle: right.to_owned(),
                  right: Some(Arc::new(make_empty_node())),
                }
              } else {
                Branch {
                  size: *size - 1,
                  depth: decide_parent_depth_op(&[left, &Some(Arc::new(changed_branch)), right]),
                  left: left.to_owned(),
                  middle: Some(Arc::new(make_empty_node())),
                  right: right.to_owned(),
                }
              }
            }
            None => unreachable!("expected data in middle branch"),
          }
        } else {
          match &right {
            Some(br) => {
              let changed_branch = br.dissoc(idx - left_size - middle_size);
              result = Branch {
                size: *size - 1,
                depth: decide_parent_depth_op(&[
                  left,
                  middle,
                  &Some(Arc::new(changed_branch.to_owned())),
                ]),
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: Some(Arc::new(changed_branch.to_owned())),
              }
            }
            None => unreachable!("expected data in right branch"),
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
            if middle == &None {
              match left {
                Some(br) => (**br).clone(),
                None => make_empty_node(),
              }
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
    if self.is_empty() {
      unreachable!("Empty node is not a correct position for inserting")
    }

    match self {
      Leaf { .. } => {
        if after {
          Branch {
            depth: self.get_depth() + 1,
            size: 2,
            left: Some(Arc::new(self.to_owned())),
            middle: Some(Arc::new(Leaf {
              size: 1,
              value: item,
            })),
            right: Some(Arc::new(make_empty_node())),
          }
        } else {
          Branch {
            depth: self.get_depth() + 1,
            size: 2,
            left: Some(Arc::new(Leaf {
              size: 1,
              value: item,
            })),
            middle: Some(Arc::new(self.to_owned())),
            right: Some(Arc::new(make_empty_node())),
          }
        }
      }
      Branch {
        left,
        middle,
        right,
        size,
        ..
      } => {
        if self.len() == 1 {
          if after {
            // in compact mode, values placed at left
            return Branch {
              size: 2,
              depth: 2,
              left: left.to_owned(),
              middle: Some(Arc::new(Leaf {
                size: 1,
                value: item,
              })),
              right: Some(Arc::new(make_empty_node())),
            };
          } else {
            return Branch {
              size: 2,
              depth: decide_parent_depth_op(&[middle]),
              left: Some(Arc::new(Leaf {
                size: 1,
                value: item,
              })),
              middle: left.to_owned(),
              right: Some(Arc::new(make_empty_node())),
            };
          }
        }

        if self.len() == 2 && middle.is_some() {
          if after {
            if idx == 0 {
              return Branch {
                size: 3,
                depth: 2,
                left: left.to_owned(),
                middle: Some(Arc::new(Leaf {
                  size: 1,
                  value: item,
                })),
                right: middle.to_owned(),
              };
            }
            if idx == 1 {
              return Branch {
                size: 3,
                depth: 2,
                left: left.to_owned(),
                middle: middle.to_owned(),
                right: Some(Arc::new(Leaf {
                  size: 1,
                  value: item,
                })),
              };
            } else {
              unreachable!("cannot insert after position 2 since only 2 elements here");
            }
          } else if idx == 0 {
            return Branch {
              size: 3,
              depth: 2,
              left: Some(Arc::new(Leaf {
                size: 1,
                value: item,
              })),
              middle: left.to_owned(),
              right: middle.to_owned(),
            };
          } else if idx == 1 {
            return Branch {
              size: 3,
              depth: 2,
              left: left.to_owned(),
              middle: Some(Arc::new(Leaf {
                size: 1,
                value: item,
              })),
              right: middle.to_owned(),
            };
          } else {
            unreachable!("cannot insert before position 2 since only 2 elements here")
          }
        }

        let left_size = match left {
          Some(br) => br.len(),
          None => 0,
        };
        let middle_size = match middle {
          Some(br) => br.len(),
          None => 0,
        };
        let right_size = match right {
          Some(br) => br.len(),
          None => 0,
        };

        if left_size + middle_size + right_size != *size {
          unreachable!("tree.size does not match sum case branch sizes");
        }

        // echo "picking: ", idx, " ", left_size, " ", middle_size, " ", right_size

        if idx == 0 && !after && left_size >= middle_size && left_size >= right_size {
          return Branch {
            size: *size + 1,
            depth: self.get_depth() + 1,
            left: Some(Arc::new(Leaf {
              size: 1,
              value: item,
            })),
            middle: Some(Arc::new(self.to_owned())),
            right: Some(Arc::new(make_empty_node())),
          };
        }

        if idx == *size - 1 && after && right_size >= middle_size && right_size >= left_size {
          return Branch {
            size: *size + 1,
            depth: self.get_depth() + 1,
            left: Some(Arc::new(self.to_owned())),
            middle: Some(Arc::new(Leaf {
              size: 1,
              value: item,
            })),
            right: Some(Arc::new(make_empty_node())),
          };
        }

        if after && idx == *size - 1 && right_size == 0 && middle_size >= left_size {
          return Branch {
            size: *size + 1,
            depth: self.get_depth(),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Some(Arc::new(Leaf {
              size: 1,
              value: item,
            })),
          };
        }

        if !after && idx == 0 && right_size == 0 && middle_size >= right_size {
          return Branch {
            size: *size + 1,
            depth: self.get_depth(),
            left: Some(Arc::new(Leaf {
              size: 1,
              value: item,
            })),
            middle: left.to_owned(),
            right: middle.to_owned(),
          };
        }

        if idx < left_size {
          let changed_branch = match left {
            Some(br) => br.insert(idx, item, after),
            None => unreachable!("expected data on left branch"),
          };
          Branch {
            size: *size + 1,
            depth: decide_parent_depth_op(&[
              &Some(Arc::new(changed_branch.to_owned())),
              middle,
              right,
            ]),
            left: Some(Arc::new(changed_branch.to_owned())),
            middle: middle.to_owned(),
            right: right.to_owned(),
          }
        } else if idx < left_size + middle_size {
          let changed_branch = match middle {
            Some(br) => br.insert(idx - left_size, item, after),
            None => unreachable!("expected data on middle branch"),
          };

          Branch {
            size: *size + 1,
            depth: decide_parent_depth_op(&[
              left,
              &Some(Arc::new(changed_branch.to_owned())),
              right,
            ]),
            left: left.to_owned(),
            middle: Some(Arc::new(changed_branch.to_owned())),
            right: right.to_owned(),
          }
        } else {
          let changed_branch = match right {
            Some(br) => br.insert(idx - left_size - middle_size, item, after),
            None => unreachable!("expected data on right branch"),
          };

          Branch {
            size: *size + 1,
            depth: decide_parent_depth_op(&[
              left,
              middle,
              &Some(Arc::new(changed_branch.to_owned())),
            ]),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Some(Arc::new(changed_branch.to_owned())),
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
      Branch {
        ref mut left,
        ref mut middle,
        ref mut right,
        ref mut depth,
        ..
      } => {
        // echo "Force inplace balancing case list: ", tree.size
        let new_tree = Self::make_list(ys.len(), 0, &ys);
        // let new_tree = initTernaryTreeList(ys)
        match new_tree {
          Branch {
            left: left2,
            right: right2,
            middle: middle2,
            ..
          } => {
            *left = left2.to_owned();
            *middle = middle2.to_owned();
            *right = right2.to_owned();
            *depth = decide_parent_depth_op(&[&left2, &middle2, &right2]);
          }
          Leaf { .. } => {
            unreachable!("expected leaf data")
          }
        }
      }
      Leaf { .. } => {}
    }
  }
  // TODO, need better strategy for detecting
  pub fn maybe_reblance(&mut self) {
    let current_depth = self.get_depth();
    if current_depth > 50 && rough_int_pow(3, current_depth - 50) > self.len() {
      self.force_inplace_balancing()
    }
  }

  pub fn prepend(&self, item: T, disable_balancing: bool) -> Self {
    if self.is_empty() {
      return Leaf {
        size: 1,
        value: item,
      };
    }

    let mut result = self.insert(0, item, false);

    if !disable_balancing {
      result.maybe_reblance();
    }

    result
  }
  pub fn append(&self, item: T, disable_balancing: bool) -> Self {
    if self.is_empty() {
      return Leaf {
        size: 1,
        value: item,
      };
    }
    let mut result = self.insert(self.len() - 1, item, true);

    if !disable_balancing {
      result.maybe_reblance();
    }
    result
  }
  pub fn concat(xs_groups: &[TernaryTreeList<T>]) -> Self {
    let mut result = Self::make_list(xs_groups.len(), 0, xs_groups);
    result.maybe_reblance();
    result
  }
  pub fn check_list_structure(&self) -> Result<(), String> {
    if self.is_empty() {
      Ok(())
    } else {
      match self {
        Leaf { size, .. } => {
          if *size != 1 {
            return Err(String::from("Bad size at node ${formatListInline(tree)}"));
          }
          Ok(())
        }
        Branch {
          left,
          middle,
          right,
          size,
          depth,
        } => {
          let left_size = match left {
            Some(br) => br.len(),
            None => 0,
          };
          let middle_size = match middle {
            Some(br) => br.len(),
            None => 0,
          };
          let right_size = match right {
            Some(br) => br.len(),
            None => 0,
          };
          if *size != left_size + middle_size + right_size {
            unreachable!("Bad size at branch ${formatListInline(tree)}");
          }

          if *depth != decide_parent_depth_op(&[left, middle, right]) {
            return Err(format!("Bad depth at branch {}", self.format_inline()));
          }

          if let Some(br) = left {
            br.check_list_structure()?;
          }
          if let Some(br) = middle {
            br.check_list_structure()?;
          }
          if let Some(br) = right {
            br.check_list_structure()?;
          }

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
      unreachable!("Invalid slice range {start_idx}..{end_idx} for {tree}");
    }
    if start_idx == end_idx {
      return Branch {
        size: 0,
        depth: 0,
        left: None,
        middle: None,
        right: None,
      };
    }

    match self {
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

        let left_size = match left {
          Some(br) => br.len(),
          None => 0,
        };
        let middle_size = match middle {
          Some(br) => br.len(),
          None => 0,
        };
        let right_size = match right {
          Some(br) => br.len(),
          None => 0,
        };

        // echo "sizes: {left_size} {middle_size} {right_size}"

        if start_idx >= left_size + middle_size {
          match right {
            Some(br) => {
              return br.slice(
                start_idx - left_size - middle_size,
                end_idx - left_size - middle_size,
              );
            }
            None => unreachable!("expected data on right branch"),
          }
        }
        if start_idx >= left_size {
          if end_idx <= left_size + middle_size {
            match middle {
              Some(br) => {
                return br.slice(start_idx - left_size, end_idx - left_size);
              }
              None => unreachable!("expected data on middle branch"),
            }
          } else {
            match (middle, right) {
              (Some(middle_br), Some(right_br)) => {
                let middle_cut = middle_br.slice(start_idx - left_size, middle_size);
                let right_cut = right_br.slice(0, end_idx - left_size - middle_size);
                return Self::concat(&[middle_cut, right_cut]);
              }
              (_, _) => unreachable!("expected data on middle and right branches"),
            }
          }
        }

        if end_idx <= left_size {
          match left {
            Some(br) => {
              return br.slice(start_idx, end_idx);
            }
            None => {
              unreachable!("expected data on right branch")
            }
          }
        }

        if end_idx <= left_size + middle_size {
          match (left, middle) {
            (Some(left_br), Some(middle_br)) => {
              let left_cut = left_br.slice(start_idx, left_size);
              let middle_cut = middle_br.slice(0, end_idx - left_size);
              return Self::concat(&[left_cut, middle_cut]);
            }
            (_, _) => unreachable!("expected some data on left and middle branches"),
          }
        }

        if end_idx <= left_size + middle_size + right_size {
          match (left, right) {
            (Some(left_br), Some(right_br)) => {
              let left_cut = left_br.slice(start_idx, left_size);
              let right_cut = right_br.slice(0, end_idx - left_size - middle_size);
              match middle {
                Some(br) => {
                  return Self::concat(&[left_cut, (**br).clone(), right_cut]);
                }
                None => {
                  return Self::concat(&[left_cut, right_cut]);
                }
              }
            }
            (_, _) => unreachable!("expected some data on left and right branches"),
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
        left: right.to_owned().map(|x| Arc::new(x.reverse())),
        middle: middle.to_owned().map(|x| Arc::new(x.reverse())),
        right: left.to_owned().map(|x| Arc::new(x.reverse())),
      },
    }
  }
  pub fn map<V>(&self, f: Arc<dyn Fn(&T) -> V>) -> TernaryTreeList<V> {
    if self.is_empty() {
      return Branch {
        size: 0,
        depth: 1,
        left: None,
        middle: None,
        right: None,
      };
    }

    match self {
      Leaf { value, .. } => Leaf {
        value: f(value),
        size: 1,
      },
      Branch {
        left,
        middle,
        right,
        size,
        depth,
      } => Branch {
        size: *size,
        depth: *depth,
        left: left.to_owned().map(|br| Arc::new(br.map(f.clone()))),
        middle: middle.to_owned().map(|br| Arc::new(br.map(f.clone()))),
        right: right.to_owned().map(|br| Arc::new(br.map(f.clone()))),
      },
    }
  }
}

// pass several children here
fn decide_parent_depth<T: Clone + fmt::Display + Eq + PartialEq>(
  xs: &[&TernaryTreeList<T>],
) -> usize {
  let mut depth = 0;
  for x in xs {
    let y = x.get_depth();
    if y > depth {
      depth = y;
    }
  }

  depth + 1
}

// pass several children here
fn decide_parent_depth_op<T: Clone + fmt::Display + Eq + PartialEq>(
  xs: &[&Option<Arc<TernaryTreeList<T>>>],
) -> usize {
  let mut depth = 0;
  for x in xs {
    match x {
      Some(x2) => {
        let y = x2.get_depth();
        if y > depth {
          depth = y;
        }
      }
      None => {}
    }
  }

  depth + 1
}

impl<T: Clone + fmt::Display + Eq + PartialEq> fmt::Display for TernaryTreeList<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TernaryTreeList[{}, ...]", self.len())
  }
}

pub struct TernaryTreeIntoIterator<T> {
  value: TernaryTreeList<T>,
  index: usize,
}

impl<T: Clone + fmt::Display + Eq + PartialEq> Iterator for TernaryTreeIntoIterator<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.value.get(self.index)
  }
}

impl<T: Clone + fmt::Display + Eq + PartialEq> PartialEq for TernaryTreeList<T> {
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
impl<T: Clone + fmt::Display + Eq + PartialEq> Eq for TernaryTreeList<T> {}

fn make_empty_node<T>() -> TernaryTreeList<T> {
  Branch {
    size: 0,
    depth: 1,
    left: None,
    middle: None,
    right: None,
  }
}

// /// TODO, Iterator
// pub fn listToItems(&self) -> Generator<T> {
//   if (tree != null) {
//     match (tree.kind) {
//       case TernaryTreeKind.ternaryTreeLeaf: {
//         yield tree.value;
//         break;
//       }
//       case TernaryTreeKind.ternaryTreeBranch: {
//         if (tree.left != null) {
//           for (let x of listToItems(tree.left)) {
//             yield x;
//           }
//         }
//         if (tree.middle != null) {
//           for (let x of listToItems(tree.middle)) {
//             yield x;
//           }
//         }
//         if (tree.right != null) {
//           for (let x of listToItems(tree.right)) {
//             yield x;
//           }
//         }
//         break;
//       }
//     }
//   }
// }
