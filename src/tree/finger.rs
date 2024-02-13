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

impl<T> TernaryTree<T>
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

  // for main branches detect keep a finger-tree like shallow-deep-shallow shape
  fn push_left_main(&self, item: Self, n: u8) -> Self {
    // println!("  iter: {} {:?}", self.format_inline(), mark);

    if self.len() + item.len() <= triple_size(n) {
      self.push_left_side(item)
    } else {
      match self {
        Leaf(_) => self.push_left_side(item),
        Branch2 { size, left, middle, .. } => {
          if left.len() + item.len() > triple_size(n) {
            Branch3 {
              size: size + item.len(),
              left: Arc::new(item),
              middle: left.to_owned(),
              right: middle.to_owned(),
            }
          } else {
            // pile items in the compact way like in sides
            // println!("    try n: {}", n);
            let item_size = item.len();
            let changed_branch = left.push_left_side(item);

            Branch2 {
              size: size + item_size,
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
            }
          }
        }
        Branch3 {
          size, right, middle, left, ..
        } => {
          // println!("    b3 n: {}", n);
          if left.len() + item.len() > triple_size(n - 1) {
            let changed_branch = middle.push_left_main((**left).to_owned(), n + 1);
            Branch3 {
              size: right.len() + changed_branch.len() + item.len(),
              left: Arc::new(item),
              middle: Arc::new(changed_branch),
              right: right.to_owned(),
            }
          } else {
            let item_size = item.len();
            let changed_branch = left.push_left_side(item);
            Branch3 {
              size: size + item_size,
              left: Arc::new(changed_branch),
              middle: middle.to_owned(),
              right: right.to_owned(),
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

  // just pile items in the compact way
  fn push_left_side(&self, item: Self) -> Self {
    // println!("  iter: {} {:?}", self.format_inline(), mark);
    match self {
      Leaf(a) => Branch2 {
        size: 1 + item.len(),
        left: Arc::new(item),
        middle: Arc::new(Leaf(a.to_owned())),
      },
      Branch2 { size, left, middle, .. } => {
        if left.len() + item.len() > middle.len() {
          Branch3 {
            size: size + item.len(),
            left: Arc::new(item),
            middle: left.to_owned(),
            right: middle.to_owned(),
          }
        } else {
          let changed_branch = left.push_left_side(item.to_owned());
          Branch2 {
            size: size + item.len(),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
          }
        }
      }
      Branch3 {
        size, right, middle, left, ..
      } => {
        if left.len() + item.len() > middle.len() {
          Branch2 {
            size: size + item.len(),
            left: Arc::new(item),
            middle: Arc::new(self.to_owned()),
          }
        } else {
          let changed_branch = left.push_left_side(item.to_owned());
          Branch3 {
            size: size + item.len(),
            left: Arc::new(changed_branch),
            middle: middle.to_owned(),
            right: right.to_owned(),
          }
        }
      }
    }
  }

  pub fn push_right(&self, item: T) -> Self {
    // start with 2 so its left child branch has capability of only 3^1
    self.push_right_main(Leaf(item), 2)
  }

  pub fn push_left(&self, item: T) -> Self {
    // start with 2 so its left child branch has capability of only 3^1
    self.push_left_main(Leaf(item), 2)
  }

  /// try to split a small bunch of elements under(or equal) a bound size,
  /// meanwhile also maintain the left branches relatively shallow
  /// if all token, the rest part returns None
  /// `bound` is `1, 3, 9, 27, ...`
  pub fn split_left_some(&self, bound: usize) -> (Self, Option<Self>) {
    if self.len() <= bound {
      return (self.to_owned(), None);
    }
    match &self {
      Leaf(_) => (self.to_owned(), None),
      Branch2 {
        size: root_size,
        left,
        middle,
      } => {
        if left.len() <= bound {
          let (next_left_branch, rest_part) = middle.split_left_some(bound * 3);
          match rest_part {
            Some(branch) => (
              (**left).to_owned(),
              Some(Branch2 {
                size: middle.len(),
                left: Arc::new(next_left_branch),
                middle: Arc::new(branch),
              }),
            ),
            None => ((**left).to_owned(), Some((**middle).to_owned())),
          }
        } else {
          match &**left {
            Leaf(_) => unreachable!("leaf should already fall into prev case"),
            Branch2 {
              size: _child_size,
              left: left_child,
              middle: middle_child,
            } => {
              let (small_bunch, rest_node) = left_child.split_left_some(bound);
              match rest_node {
                Some(branch) => (
                  small_bunch.to_owned(),
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: Arc::new(branch),
                    middle: middle_child.to_owned(),
                    right: middle.to_owned(),
                  }),
                ),
                None => (
                  (**left_child).to_owned(),
                  Some(Branch2 {
                    size: root_size - left_child.len(),
                    left: middle_child.to_owned(),
                    middle: middle.to_owned(),
                  }),
                ),
              }
            }
            Branch3 {
              size: child_size,
              left: left_child,
              middle: middle_child,
              right: right_child,
            } => {
              let (small_bunch, rest_node) = left_child.split_left_some(bound);
              match rest_node {
                Some(branch) => (
                  small_bunch.to_owned(),
                  Some(Branch2 {
                    size: root_size - small_bunch.len(),
                    left: Arc::new(Branch3 {
                      size: child_size - small_bunch.len(),
                      left: Arc::new(branch),
                      middle: middle_child.to_owned(),
                      right: right_child.to_owned(),
                    }),
                    middle: middle.to_owned(),
                  }),
                ),
                None => (
                  small_bunch.to_owned(),
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: middle_child.to_owned(),
                    middle: right_child.to_owned(),
                    right: middle.to_owned(),
                  }),
                ),
              }
            }
          }
        }
      }
      Branch3 {
        size: root_size,
        right,
        middle,
        left,
      } => {
        if left.len() <= bound {
          let (next_left_branch, rest_part) = middle.split_left_some(bound * 3);
          match rest_part {
            Some(branch) => (
              (**left).to_owned(),
              Some(Branch3 {
                size: root_size - left.len(),
                left: Arc::new(next_left_branch),
                middle: Arc::new(branch),
                right: right.to_owned(),
              }),
            ),
            None => (
              (**left).to_owned(),
              Some(Branch2 {
                size: root_size - left.len(),
                left: middle.to_owned(),
                middle: right.to_owned(),
              }),
            ),
          }
        } else {
          match &**left {
            Leaf(_) => unreachable!("leaf should already fall into prev case"),
            Branch2 {
              size: child_size,
              left: left_child,
              middle: middle_child,
            } => {
              let (small_bunch, rest_node) = left_child.split_left_some(bound);
              match rest_node {
                Some(branch) => (
                  small_bunch.to_owned(),
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: Arc::new(Branch2 {
                      size: child_size - small_bunch.len(),
                      left: Arc::new(branch),
                      middle: middle_child.to_owned(),
                    }),
                    middle: middle.to_owned(),
                    right: right.to_owned(),
                  }),
                ),
                None => (
                  (**left_child).to_owned(),
                  Some(Branch3 {
                    size: root_size - left_child.len(),
                    left: middle_child.to_owned(),
                    middle: middle.to_owned(),
                    right: right.to_owned(),
                  }),
                ),
              }
            }
            Branch3 {
              size: child_size,
              left: left_child,
              middle: middle_child,
              right: right_child,
            } => {
              let (small_bunch, rest_node) = left_child.split_left_some(bound);
              match rest_node {
                Some(branch) => (
                  small_bunch.to_owned(),
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: Arc::new(Branch3 {
                      size: child_size - small_bunch.len(),
                      left: Arc::new(branch),
                      middle: middle_child.to_owned(),
                      right: right_child.to_owned(),
                    }),
                    middle: middle.to_owned(),
                    right: right.to_owned(),
                  }),
                ),
                None => (
                  small_bunch.to_owned(),
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: Arc::new(Branch2 {
                      size: child_size - small_bunch.len(),
                      left: middle_child.to_owned(),
                      middle: right_child.to_owned(),
                    }),
                    middle: middle.to_owned(),
                    right: right.to_owned(),
                  }),
                ),
              }
            }
          }
        }
      }
    }
  }

  pub fn split_right_some(&self, bound: usize) -> (Option<Self>, Self) {
    if self.len() <= bound {
      return (None, self.to_owned());
    }
    match &self {
      Leaf(_) => (None, self.to_owned()),
      Branch2 {
        size: root_size,
        left,
        middle,
      } => {
        if middle.len() <= bound {
          let (rest_part, next_right_branch) = left.split_right_some(bound * 3);
          match rest_part {
            Some(branch) => (
              Some(Branch2 {
                size: left.len(),
                left: Arc::new(branch),
                middle: Arc::new(next_right_branch),
              }),
              (**middle).to_owned(),
            ),
            None => (Some((**left).to_owned()), (**middle).to_owned()),
          }
        } else {
          match &**middle {
            Leaf(_) => unreachable!("leaf should already fall into prev case"),
            Branch2 {
              size: _child_size,
              left: left_child,
              middle: middle_child,
            } => {
              let (rest_node, small_bunch) = middle_child.split_right_some(bound);
              match rest_node {
                Some(branch) => (
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: left.to_owned(),
                    middle: left_child.to_owned(),
                    right: Arc::new(branch),
                  }),
                  small_bunch,
                ),
                None => (
                  Some(Branch2 {
                    size: root_size - middle_child.len(),
                    left: left.to_owned(),
                    middle: left_child.to_owned(),
                  }),
                  (**middle_child).to_owned(),
                ),
              }
            }
            Branch3 {
              size: child_size,
              left: left_child,
              middle: middle_child,
              right: right_child,
            } => {
              let (rest_node, small_bunch) = right_child.split_right_some(bound);
              match rest_node {
                Some(branch) => (
                  Some(Branch2 {
                    size: root_size - small_bunch.len(),
                    left: left.to_owned(),
                    middle: Arc::new(Branch3 {
                      size: child_size - small_bunch.len(),
                      left: left_child.to_owned(),
                      middle: middle_child.to_owned(),
                      right: Arc::new(branch),
                    }),
                  }),
                  small_bunch,
                ),
                None => (
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: left.to_owned(),
                    middle: left_child.to_owned(),
                    right: middle_child.to_owned(),
                  }),
                  small_bunch,
                ),
              }
            }
          }
        }
      }
      Branch3 {
        size: root_size,
        left,
        middle,
        right,
      } => {
        if right.len() <= bound {
          let (rest_part, next_right_branch) = middle.split_right_some(bound * 3);
          match rest_part {
            Some(branch) => (
              Some(Branch3 {
                size: root_size - right.len(),
                left: left.to_owned(),
                middle: Arc::new(branch),
                right: Arc::new(next_right_branch),
              }),
              (**right).to_owned(),
            ),
            None => (
              Some(Branch2 {
                size: root_size - right.len(),
                left: left.to_owned(),
                middle: middle.to_owned(),
              }),
              (**right).to_owned(),
            ),
          }
        } else {
          match &**right {
            Leaf(_) => unreachable!("leaf should already fall into prev case"),
            Branch2 {
              size: child_size,
              left: left_child,
              middle: middle_child,
            } => {
              let (rest_node, small_bunch) = middle_child.split_right_some(bound);
              match rest_node {
                Some(branch) => (
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: left.to_owned(),
                    middle: middle.to_owned(),
                    right: Arc::new(Branch2 {
                      size: child_size - small_bunch.len(),
                      left: left_child.to_owned(),
                      middle: Arc::new(branch),
                    }),
                  }),
                  small_bunch,
                ),
                None => (
                  Some(Branch3 {
                    size: root_size - middle_child.len(),
                    left: left.to_owned(),
                    middle: middle.to_owned(),
                    right: left_child.to_owned(),
                  }),
                  (**middle_child).to_owned(),
                ),
              }
            }
            Branch3 {
              size: child_size,
              left: left_child,
              middle: middle_child,
              right: right_child,
            } => {
              let (rest_node, small_bunch) = right_child.split_right_some(bound);
              match rest_node {
                Some(branch) => (
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: left.to_owned(),
                    middle: middle.to_owned(),
                    right: Arc::new(Branch3 {
                      size: child_size - small_bunch.len(),
                      left: left_child.to_owned(),
                      middle: middle_child.to_owned(),
                      right: Arc::new(branch),
                    }),
                  }),
                  small_bunch,
                ),
                None => (
                  Some(Branch3 {
                    size: root_size - small_bunch.len(),
                    left: left.to_owned(),
                    middle: middle.to_owned(),
                    right: Arc::new(Branch2 {
                      size: child_size - small_bunch.len(),
                      left: left_child.to_owned(),
                      middle: middle_child.to_owned(),
                    }),
                  }),
                  small_bunch,
                ),
              }
            }
          }
        }
      }
    }
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

  pub fn drop_right(&self) -> Self {
    match self {
      Leaf(_) => {
        unreachable!("not expected empty node inside tree")
      }
      Branch2 { size, left, middle, .. } => {
        if middle.len() == 1 {
          (**left).to_owned()
        } else {
          let changed_branch = middle.drop_right();
          match changed_branch {
            Branch2 {
              left: b_left,
              middle: b_middle,
              ..
            } => Branch3 {
              size: size - 1,
              left: left.to_owned(),
              middle: b_left,
              right: b_middle,
            },
            Branch3 {
              left: b_left,
              middle: b_middle,
              right: b_right,
              ..
            } => {
              let internal_branch = Branch2 {
                size: b_middle.len() + b_left.len(),
                left: b_left,
                middle: b_middle,
              };
              Branch3 {
                size: size - 1,
                left: left.to_owned(),
                middle: Arc::new(internal_branch),
                right: b_right,
              }
            }
            _ => Branch2 {
              size: size - 1,
              left: left.to_owned(),
              middle: Arc::new(changed_branch),
            },
          }
        }
      }
      Branch3 {
        size, right, middle, left, ..
      } => {
        if right.len() == 1 {
          match &**middle {
            Branch2 {
              left: b_left,
              middle: b_middle,
              ..
            } => Branch3 {
              size: size - 1,
              left: left.to_owned(),
              middle: b_left.to_owned(),
              right: b_middle.to_owned(),
            },
            Branch3 {
              left: b_left,
              middle: b_middle,
              right: b_right,
              ..
            } => {
              let internal_branch = Branch2 {
                size: b_middle.len() + b_left.len(),
                left: b_left.to_owned(),
                middle: b_middle.to_owned(),
              };
              Branch3 {
                size: size - 1,
                left: left.to_owned(),
                middle: Arc::new(internal_branch),
                right: b_right.to_owned(),
              }
            }
            _ => Branch2 {
              size: size - 1,
              left: left.to_owned(),
              middle: middle.to_owned(),
            },
          }
        } else {
          let changed_branch = right.drop_right();
          Branch3 {
            size: size - 1,
            left: left.to_owned(),
            middle: middle.to_owned(),
            right: Arc::new(changed_branch),
          }
        }
      }
    }
  }
}
