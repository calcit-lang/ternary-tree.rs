//! contains tricks for faster operating on both ends,
//! the trick is learnt from finger-tree trying to maintain shallow branches near both ends,
//! thus adding/removing there can be cheap and somehow reaching `O(1)` at best cases.
//! branches in the middle of this "pyramid" is the deepest.
//!
//! Finger-tree encodes tree structure in its ADT, however this file uses a dynamic solution,
//! i.e. detects sizes of branches, and to decide where to put new elements.
//! it's not a perfect structure for best speed, but trying to be reaching.
//!
//! ![](https://pbs.twimg.com/media/FRc3gB7aQAA1pBb?format=jpg&name=4096x4096)
//!
//! Tree layout from 0 to 159 watch [video](https://www.bilibili.com/video/BV1F34y147V7) or try [live demo](https://github.com/calcit-lang/explain-ternary-tree).

use super::TernaryTree::{self, *};

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;

use crate::util::triple_size;

impl<'a, T> TernaryTree<T>
where
  T: Clone + Display + Eq + PartialEq + Debug + Ord + PartialOrd + Hash,
{
  // for main branches detect keep a finger-tree like shallow-deep-shallow shape
  fn push_right_main(&self, item: Self, n: u8) -> Self {
    // println!("  iter: {} {:?}", self.format_inline(), mark);

    if self.len() + item.len() <= triple_size(n) {
      self.push_right_side(item)
    } else {
      match self {
        Leaf(_) => self.push_right_side(item),
        Branch2 { size, left, middle, .. } => {
          if middle.len() + item.len() > triple_size(n) {
            Branch3 {
              size: size + item.len(),
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(item),
            }
          } else {
            // pile items in the compact way like in sides
            // println!("    try n: {}", n);
            let item_size = item.len();
            let changed_branch = middle.push_right_side(item);

            Branch2 {
              size: size + item_size,
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
            }
          }
        }
        Branch3 {
          size, left, middle, right, ..
        } => {
          // println!("    b3 n: {}", n);
          if right.len() + item.len() > triple_size(n - 1) {
            let changed_branch = middle.push_right_main((**right).to_owned(), n + 1);
            Branch3 {
              size: left.len() + changed_branch.len() + item.len(),
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
              right: Arc::new(item),
            }
          } else {
            let item_size = item.len();
            let changed_branch = right.push_right_side(item);
            Branch3 {
              size: size + item_size,
              left: left.to_owned(),
              middle: middle.to_owned(),
              right: Arc::new(changed_branch),
            }
          }
        }
      }
    }
  }

  // just pile items in the compact way
  fn push_right_side(&self, item: Self) -> Self {
    // println!("  iter: {} {:?}", self.format_inline(), mark);
    match self {
      Leaf(a) => Branch2 {
        size: 1 + item.len(),
        left: Arc::new(Leaf(a.to_owned())),
        middle: Arc::new(item),
      },
      Branch2 { size, left, middle, .. } => {
        if middle.len() + item.len() > left.len() {
          Branch3 {
            size: size + item.len(),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(item),
          }
        } else {
          let changed_branch = middle.push_right_side(item.to_owned());
          Branch2 {
            size: size + item.len(),
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
            left: Arc::new(self.to_owned()),
            middle: Arc::new(item),
          }
        } else {
          let changed_branch = right.push_right_side(item.to_owned());
          Branch3 {
            size: size + item.len(),
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch),
          }
        }
      }
    }
  }

  pub fn push_right(&self, item: T) -> Self {
    // start with 2 so its left child branch has capability of only 3^1
    self.push_right_main(Leaf(Arc::new(item)), 2)
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
              left: b_left,
              middle: b_middle,
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
                left: b_middle,
                middle: b_right,
              };
              Branch3 {
                size: size - 1,
                left: b_left,
                middle: Arc::new(internal_branch),
                right: middle.to_owned(),
              }
            }
            _ => Branch2 {
              size: size - 1,
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
                left: b_middle.to_owned(),
                middle: b_right.to_owned(),
              };
              Branch3 {
                size: size - 1,
                left: b_left.to_owned(),
                middle: Arc::new(internal_branch),
                right: right.to_owned(),
              }
            }
            _ => Branch2 {
              size: size - 1,
              left: middle.to_owned(),
              middle: right.to_owned(),
            },
          }
        } else {
          let changed_branch = left.drop_left();
          Branch3 {
            size: size - 1,
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          }
        }
      }
    }
  }
}
