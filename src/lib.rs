mod util;

use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;

use util::divide_ternary_sizes;

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
    None

    // TODO
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

  fn write_leaves(&mut self, acc: /* var */ &mut [TernaryTreeList<T>], counter: &RefCell<usize>) {
    if self.is_empty() {
      return;
    }

    match self {
      Leaf { value, .. } => {
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

  pub fn to_leaves(&self) -> Vec<TernaryTreeList<T>> {
    let mut acc: Vec<TernaryTreeList<T>> = Vec::with_capacity(self.len());
    let counter: RefCell<usize> = RefCell::new(5);
    Self::write_leaves(&mut self, &mut acc, &counter);
    return acc;
  }

  pub fn unsafe_get(&self, original_idx: usize) -> T {
    let mut tree_parent = Some(Arc::new(*self));
    let mut idx = original_idx;
    while let Some(tree) = tree_parent {
      if idx < 0 {
        unreachable!("Cannot index negative number")
      }

      match *tree {
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

          if left_size + middle_size + right_size != size {
            unreachable!("tree.size does not match sum case branch sizes");
          }

          if idx < left_size {
            tree_parent = left.to_owned();
          } else if idx < left_size + middle_size {
            tree_parent = middle.to_owned();
            idx = idx - left_size;
          } else {
            tree_parent = right.to_owned();
            idx = idx - left_size - middle_size;
          }
        }
      }
    }

    unreachable!("Failed to get ${idx}")
  }
}

// pub fn first(&self) -> T {
//   if (listLen(tree) > 0) {
//     return listGet(tree, 0);
//   } else {
//     throw new Error("Cannot get from empty list");
//   }
// }

// pub fn last(&self) -> T {
//   if (listLen(tree) > 0) {
//     return listGet(tree, listLen(tree) - 1);
//   } else {
//     throw new Error("Cannot get from empty list");
//   }
// }

// pub fn assocList(&self, idx: number, item: T): TernaryTreeList<T> {
//   if (idx < 0) {
//     throw new Error("Cannot index negative number");
//   }
//   if (idx > tree.size - 1) {
//     throw new Error("Index too large");
//   }

//   if (tree.kind === TernaryTreeKind.ternaryTreeLeaf) {
//     if (idx === 0) {
//       return { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>;
//     } else {
//       throw new Error(`Cannot get from leaf with index ${idx}`);
//     }
//   }

//   let leftSize = listLen(tree.left);
//   let middleSize = listLen(tree.middle);
//   let rightSize = listLen(tree.right);

//   if (leftSize + middleSize + rightSize !== tree.size) throw new Error("tree.size does not match sum case branch sizes");

//   if (idx <= leftSize - 1) {
//     let changedBranch = assocList(tree.left, idx, item);
//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size,
//       depth: decideParentDepth(changedBranch, tree.middle, tree.right),
//       left: changedBranch,
//       middle: tree.middle,
//       right: tree.right,
//     };
//     return result;
//   } else if (idx <= leftSize + middleSize - 1) {
//     let changedBranch = assocList(tree.middle, idx - leftSize, item);
//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size,
//       depth: decideParentDepth(tree.left, changedBranch, tree.right),
//       left: tree.left,
//       middle: changedBranch,
//       right: tree.right,
//     };
//     return result;
//   } else {
//     let changedBranch = assocList(tree.right, idx - leftSize - middleSize, item);
//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size,
//       depth: decideParentDepth(tree.left, tree.middle, changedBranch),
//       left: tree.left,
//       middle: tree.middle,
//       right: changedBranch,
//     };
//     return result;
//   }
// }

// pub fn dissocList(&self, idx: number): TernaryTreeList<T> {
//   if (tree == null) {
//     throw new Error("dissoc does not work on null");
//   }

//   if (idx < 0) {
//     throw new Error(`Index is negative ${idx}`);
//   }

//   if (listLen(tree) === 0) {
//     throw new Error("Cannot remove from empty list");
//   }

//   if (idx > listLen(tree) - 1) {
//     throw new Error(`Index too large ${idx}`);
//   }

//   if (listLen(tree) === 1) {
//     return emptyBranch;
//   }

//   if (tree.kind === TernaryTreeKind.ternaryTreeLeaf) {
//     throw new Error("dissoc should be handled at branches");
//   }

//   let leftSize = listLen(tree.left);
//   let middleSize = listLen(tree.middle);
//   let rightSize = listLen(tree.right);

//   if (leftSize + middleSize + rightSize !== tree.size) {
//     throw new Error("tree.size does not match sum from branch sizes");
//   }

//   let result: TernaryTreeList<T> = emptyBranch;

//   if (idx <= leftSize - 1) {
//     let changedBranch = dissocList(tree.left, idx);
//     if (changedBranch == null || changedBranch.size === 0) {
//       result = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size - 1,
//         depth: decideParentDepth(tree.middle, tree.right),
//         left: tree.middle,
//         middle: tree.right,
//         right: emptyBranch,
//       };
//     } else {
//       result = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size - 1,
//         depth: decideParentDepth(changedBranch, tree.middle, tree.right),
//         left: changedBranch,
//         middle: tree.middle,
//         right: tree.right,
//       };
//     }
//   } else if (idx <= leftSize + middleSize - 1) {
//     let changedBranch = dissocList(tree.middle, idx - leftSize);
//     if (changedBranch == null || changedBranch.size === 0) {
//       result = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size - 1,
//         depth: decideParentDepth(tree.left, changedBranch, tree.right),
//         left: tree.left,
//         middle: tree.right,
//         right: emptyBranch,
//       };
//     } else {
//       result = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size - 1,
//         depth: decideParentDepth(tree.left, changedBranch, tree.right),
//         left: tree.left,
//         middle: changedBranch,
//         right: tree.right,
//       };
//     }
//   } else {
//     let changedBranch = dissocList(tree.right, idx - leftSize - middleSize);
//     if (changedBranch == null || changedBranch.size === 0) {
//       changedBranch = emptyBranch;
//     }
//     result = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size - 1,
//       depth: decideParentDepth(tree.left, tree.middle, changedBranch),
//       left: tree.left,
//       middle: tree.middle,
//       right: changedBranch,
//     };
//   }
//   if (result.middle == null) {
//     return result.left;
//   }
//   return result;
// }

// pub fn rest(&self) -> TernaryTreeList<T> {
//   if (tree == null) {
//     throw new Error("Cannot call rest on null");
//   }
//   if (listLen(tree) < 1) {
//     throw new Error("Cannot call rest on empty list");
//   }

//   return dissocList(tree, 0);
// }

// pub fn butlast(&self) -> TernaryTreeList<T> {
//   if (tree == null) {
//     throw new Error("Cannot call butlast on null");
//   }
//   if (listLen(tree) < 1) {
//     throw new Error("Cannot call butlast on empty list");
//   }

//   return dissocList(tree, listLen(tree) - 1);
// }

// pub fn insert(&self, idx: number, item: T, after: bool = false): TernaryTreeList<T> {
//   if (tree == null) {
//     throw new Error("Cannot insert into null");
//   }
//   if (listLen(tree) === 0) {
//     throw new Error("Empty node is not a correct position for inserting");
//   }

//   if (tree.kind === TernaryTreeKind.ternaryTreeLeaf) {
//     if (after) {
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         depth: getDepth(tree) + 1,
//         size: 2,
//         left: tree,
//         middle: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         right: emptyBranch,
//       };
//       return result;
//     } else {
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         depth: getDepth(tree) + 1,
//         size: 2,
//         left: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         middle: tree,
//         right: emptyBranch,
//       };
//       return result;
//     }
//   }

//   if (listLen(tree) === 1) {
//     if (after) {
//       // in compact mode, values placed at left
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: 2,
//         depth: 2,
//         left: tree.left,
//         middle: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         right: emptyBranch,
//       };
//       return result;
//     } else {
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: 2,
//         depth: getDepth(tree.middle) + 1,
//         left: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         middle: tree.left,
//         right: emptyBranch,
//       };
//       return result;
//     }
//   }

//   if (listLen(tree) === 2 && tree.middle != null) {
//     if (after) {
//       if (idx === 0) {
//         let result: TernaryTreeList<T> = {
//           kind: TernaryTreeKind.ternaryTreeBranch,
//           size: 3,
//           depth: 2,
//           left: tree.left,
//           middle: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//           right: tree.middle,
//         };
//         return result;
//       }
//       if (idx === 1) {
//         let result: TernaryTreeList<T> = {
//           kind: TernaryTreeKind.ternaryTreeBranch,
//           size: 3,
//           depth: 2,
//           left: tree.left,
//           middle: tree.middle,
//           right: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         };
//         return result;
//       } else {
//         throw new Error("cannot insert after position 2 since only 2 elements here");
//       }
//     } else {
//       if (idx === 0) {
//         let result: TernaryTreeList<T> = {
//           kind: TernaryTreeKind.ternaryTreeBranch,
//           size: 3,
//           depth: 2,
//           left: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//           middle: tree.left,
//           right: tree.middle,
//         };
//         return result;
//       } else if (idx === 1) {
//         let result: TernaryTreeList<T> = {
//           kind: TernaryTreeKind.ternaryTreeBranch,
//           size: 3,
//           depth: 2,
//           left: tree.left,
//           middle: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//           right: tree.middle,
//         };
//         return result;
//       } else {
//         throw new Error("cannot insert before position 2 since only 2 elements here");
//       }
//     }
//   }

//   let leftSize = listLen(tree.left);
//   let middleSize = listLen(tree.middle);
//   let rightSize = listLen(tree.right);

//   if (leftSize + middleSize + rightSize !== tree.size) {
//     throw new Error("tree.size does not match sum case branch sizes");
//   }

//   // echo "picking: ", idx, " ", leftSize, " ", middleSize, " ", rightSize

//   if (idx === 0 && !after) {
//     if (listLen(tree.left) >= listLen(tree.middle) && listLen(tree.left) >= listLen(tree.right)) {
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size + 1,
//         depth: tree.depth + 1,
//         left: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         middle: tree,
//         right: emptyBranch,
//       };
//       return result;
//     }
//   }

//   if (idx === listLen(tree) - 1 && after) {
//     if (listLen(tree.right) >= listLen(tree.middle) && listLen(tree.right) >= listLen(tree.left)) {
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size + 1,
//         depth: tree.depth + 1,
//         left: tree,
//         middle: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//         right: emptyBranch,
//       };
//       return result;
//     }
//   }

//   if (after && idx === listLen(tree) - 1 && rightSize === 0 && middleSize >= leftSize) {
//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size + 1,
//       depth: tree.depth,
//       left: tree.left,
//       middle: tree.middle,
//       right: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//     };
//     return result;
//   }

//   if (!after && idx === 0 && rightSize === 0 && middleSize >= rightSize) {
//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size + 1,
//       depth: tree.depth,
//       left: { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>,
//       middle: tree.left,
//       right: tree.middle,
//     };
//     return result;
//   }

//   if (idx <= leftSize - 1) {
//     let changedBranch = insert(tree.left, idx, item, after);

//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size + 1,
//       depth: decideParentDepth(changedBranch, tree.middle, tree.right),
//       left: changedBranch,
//       middle: tree.middle,
//       right: tree.right,
//     };
//     return result;
//   } else if (idx <= leftSize + middleSize - 1) {
//     let changedBranch = insert(tree.middle, idx - leftSize, item, after);

//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size + 1,
//       depth: decideParentDepth(tree.left, changedBranch, tree.right),
//       left: tree.left,
//       middle: changedBranch,
//       right: tree.right,
//     };

//     return result;
//   } else {
//     let changedBranch = insert(tree.right, idx - leftSize - middleSize, item, after);

//     let result: TernaryTreeList<T> = {
//       kind: TernaryTreeKind.ternaryTreeBranch,
//       size: tree.size + 1,
//       depth: decideParentDepth(tree.left, tree.middle, changedBranch),
//       left: tree.left,
//       middle: tree.middle,
//       right: changedBranch,
//     };
//     return result;
//   }
// }

// pub fn assocBefore(&self, idx: number, item: T, after: bool = false): TernaryTreeList<T> {
//   return insert(tree, idx, item, false);
// }

// pub fn assocAfter(&self, idx: number, item: T, after: bool = false): TernaryTreeList<T> {
//   return insert(tree, idx, item, true);
// }

// // this function mutates original tree to make it more balanced
// pub fn forceListInplaceBalancing(&self) -> void {
//   if (tree.kind === TernaryTreeKind.ternaryTreeBranch) {
//     // echo "Force inplace balancing case list: ", tree.size
//     let ys = toLeavesArray(tree);
//     let newTree = makeTernaryTreeList(ys.length, 0, ys) as TernaryTreeListTheBranch<T>;
//     // let newTree = initTernaryTreeList(ys)
//     tree.left = newTree.left;
//     tree.middle = newTree.middle;
//     tree.right = newTree.right;
//     tree.depth = decideParentDepth(tree.left, tree.middle, tree.right);
//   } else {
//     //
//   }
// }

// // TODO, need better strategy for detecting
// function maybeReblance(&self) -> void {
//   let currentDepth = getDepth(tree);
//   if (currentDepth > 50) {
//     if (roughIntPow(3, currentDepth - 50) > tree.size) {
//       forceListInplaceBalancing(tree);
//     }
//   }
// }

// pub fn prepend(&self, item: T, disableBalancing: bool = false): TernaryTreeList<T> {
//   if (tree == null || listLen(tree) === 0) {
//     return { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>;
//   }
//   let result = insert(tree, 0, item, false);

//   if (!disableBalancing) {
//     maybeReblance(result);
//   }
//   return result;
// }

// pub fn append(&self, item: T, disableBalancing: bool = false): TernaryTreeList<T> {
//   if (tree == null || listLen(tree) === 0) {
//     return { kind: TernaryTreeKind.ternaryTreeLeaf, size: 1, value: item } as TernaryTreeList<T>;
//   }
//   let result = insert(tree, listLen(tree) - 1, item, true);

//   if (!disableBalancing) {
//     maybeReblance(result);
//   }
//   return result;
// }

// pub fn concat<T>(...xsGroups: Array<TernaryTreeList<T>>): TernaryTreeList<T> {
//   xsGroups = xsGroups.filter((xs) => listLen(xs) > 0);
//   let result = makeTernaryTreeList(xsGroups.length, 0, xsGroups);
//   maybeReblance(result);
//   return result;
// }

// pub fn listEqual<T>(xs: TernaryTreeList<T>, ys: TernaryTreeList<T>) -> bool {
//   if (xs === ys) {
//     return true;
//   }
//   if (listLen(xs) !== listLen(ys)) {
//     return false;
//   }

//   for (let idx = 0; idx < listLen(xs); idx++) {
//     if (!dataEqual(listGet(xs, idx), listGet(ys, idx))) {
//       return false;
//     }
//   }

//   return true;
// }

// pub fn checkListStructure(&self) -> bool {
//   if (tree == null) {
//     return true;
//   } else {
//     match (tree.kind) {
//       case TernaryTreeKind.ternaryTreeLeaf:
//         if (tree.size !== 1) {
//           throw new Error(`Bad size at node ${formatListInline(tree)}`);
//         }
//         break;
//       case TernaryTreeKind.ternaryTreeBranch: {
//         if (tree.size !== listLen(tree.left) + listLen(tree.middle) + listLen(tree.right)) {
//           throw new Error(`Bad size at branch ${formatListInline(tree)}`);
//         }

//         if (tree.depth !== decideParentDepth(tree.left, tree.middle, tree.right)) {
//           let x = decideParentDepth(tree.left, tree.middle, tree.right);
//           throw new Error(`Bad depth at branch ${formatListInline(tree)}`);
//         }

//         checkListStructure(tree.left);
//         checkListStructure(tree.middle);
//         checkListStructure(tree.right);
//         break;
//       }
//     }

//     return true;
//   }
// }

// // excludes value at endIdx, kept aligned with JS & Clojure
// pub fn slice(&self, startIdx: number, endIdx: number): TernaryTreeList<T> {
//   // echo "slice {tree.formatListInline}: {startIdx}..{endIdx}"
//   if (endIdx > listLen(tree)) {
//     throw new Error("Slice range too large {endIdx} for {tree}");
//   }
//   if (startIdx < 0) {
//     throw new Error("Slice range too small {startIdx} for {tree}");
//   }
//   if (startIdx > endIdx) {
//     throw new Error("Invalid slice range {startIdx}..{endIdx} for {tree}");
//   }
//   if (startIdx === endIdx) {
//     return { kind: TernaryTreeKind.ternaryTreeBranch, size: 0, depth: 0 } as TernaryTreeList<T>;
//   }

//   if (tree.kind === TernaryTreeKind.ternaryTreeLeaf)
//     if (startIdx === 0 && endIdx === 1) {
//       return tree;
//     } else {
//       throw new Error(`Invalid slice range for a leaf: ${startIdx} ${endIdx}`);
//     }

//   if (startIdx === 0 && endIdx === listLen(tree)) {
//     return tree;
//   }

//   let leftSize = listLen(tree.left);
//   let middleSize = listLen(tree.middle);
//   let rightSize = listLen(tree.right);

//   // echo "sizes: {leftSize} {middleSize} {rightSize}"

//   if (startIdx >= leftSize + middleSize) {
//     return slice(tree.right, startIdx - leftSize - middleSize, endIdx - leftSize - middleSize);
//   }
//   if (startIdx >= leftSize)
//     if (endIdx <= leftSize + middleSize) {
//       return slice(tree.middle, startIdx - leftSize, endIdx - leftSize);
//     } else {
//       let middleCut = slice(tree.middle, startIdx - leftSize, middleSize);
//       let rightCut = slice(tree.right, 0, endIdx - leftSize - middleSize);
//       return concat(middleCut, rightCut);
//     }

//   if (endIdx <= leftSize) {
//     return slice(tree.left, startIdx, endIdx);
//   }

//   if (endIdx <= leftSize + middleSize) {
//     let leftCut = slice(tree.left, startIdx, leftSize);
//     let middleCut = slice(tree.middle, 0, endIdx - leftSize);
//     return concat(leftCut, middleCut);
//   }

//   if (endIdx <= leftSize + middleSize + rightSize) {
//     let leftCut = slice(tree.left, startIdx, leftSize);
//     let rightCut = slice(tree.right, 0, endIdx - leftSize - middleSize);
//     return concat(concat(leftCut, tree.middle), rightCut);
//   }
//   throw new Error("Unknown");
// }

// pub fn reverse(&self) -> TernaryTreeList<T> {
//   if (tree == null) {
//     return tree;
//   }

//   match (tree.kind) {
//     case TernaryTreeKind.ternaryTreeLeaf:
//       return tree;
//     case TernaryTreeKind.ternaryTreeBranch: {
//       let result: TernaryTreeList<T> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size,
//         depth: tree.depth,
//         left: reverse(tree.right),
//         middle: reverse(tree.middle),
//         right: reverse(tree.left),
//       };
//       return result;
//     }
//   }
// }

// pub fn listMapValues<T, V>(&self, f: (x: T) => V): TernaryTreeList<V> {
//   if (tree == null) {
//     return tree;
//   }

//   match (tree.kind) {
//     case TernaryTreeKind.ternaryTreeLeaf: {
//       let result: TernaryTreeList<V> = {
//         kind: TernaryTreeKind.ternaryTreeLeaf,
//         size: tree.size,
//         value: f(tree.value),
//       };
//       return result;
//     }
//     case TernaryTreeKind.ternaryTreeBranch: {
//       let result: TernaryTreeList<V> = {
//         kind: TernaryTreeKind.ternaryTreeBranch,
//         size: tree.size,
//         depth: tree.depth,
//         left: tree.left == null ? emptyBranch : listMapValues(tree.left, f),
//         middle: tree.middle == null ? emptyBranch : listMapValues(tree.middle, f),
//         right: tree.right == null ? emptyBranch : listMapValues(tree.right, f),
//       };
//       return result;
//     }
//   }
// }

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
