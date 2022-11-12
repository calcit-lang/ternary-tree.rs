use std::sync::Arc;

use super::TernaryTree;

#[derive(Clone)]
pub struct TernaryTreeIntoIter<T> {
  // we use a VecDeque because it allows
  // removing elements from the front efficiently [1]
  children: Option<Arc<TernaryTree<T>>>,
  parent: Option<Arc<TernaryTreeIntoIter<T>>>,
}

impl<T> TernaryTreeIntoIter<T> {
  pub fn new(tree: TernaryTree<T>) -> Self {
    TernaryTreeIntoIter {
      children: Some(Arc::new(tree)),
      parent: None,
    }
  }
}

impl<T> Default for TernaryTreeIntoIter<T>
where
  T: Clone,
{
  fn default() -> Self {
    TernaryTreeIntoIter {
      children: None,
      parent: None,
    }
  }
}

impl<T> Iterator for TernaryTreeIntoIter<T>
where
  T: Clone,
{
  type Item = Arc<T>;

  fn next(&mut self) -> Option<Self::Item> {
    let c = self.children.to_owned();
    match &c {
      None => None,
      Some(target) => match &**target {
        TernaryTree::Leaf(x) => match &self.parent {
          None => {
            self.children = None;
            Some(x.to_owned())
          }
          Some(p) => {
            *self = (**p).to_owned();
            Some(x.to_owned())
          }
        },
        TernaryTree::Branch2 { size: _size, left, middle } => {
          let mut parent = self.to_owned();
          parent.children = Some(middle.to_owned());
          *self = TernaryTreeIntoIter {
            children: Some(left.to_owned()),
            parent: Some(Arc::new(parent)),
          };
          self.next()
        }
        TernaryTree::Branch3 { size, left, middle, right } => {
          let mut parent = self.to_owned();
          parent.children = Some(Arc::new(TernaryTree::Branch2 {
            size: size - left.len(),
            left: middle.to_owned(),
            middle: right.to_owned(),
          }));
          *self = TernaryTreeIntoIter {
            children: Some(left.to_owned()),
            parent: Some(Arc::new(parent)),
          };
          self.next()
        }
      },
    }
  }
}

impl<T> IntoIterator for TernaryTree<T>
where
  T: Clone,
{
  type Item = Arc<T>;

  type IntoIter = TernaryTreeIntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    TernaryTreeIntoIter {
      children: Some(Arc::new(self)),
      parent: None,
    }
  }
}
