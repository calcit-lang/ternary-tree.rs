use im_ternary_tree::TernaryTreeList;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct CloneTracked {
  value: usize,
  clones: Arc<AtomicUsize>,
}

impl CloneTracked {
  fn new(value: usize) -> (Self, Arc<AtomicUsize>) {
    let clones = Arc::new(AtomicUsize::new(0));
    (
      Self {
        value,
        clones: Arc::clone(&clones),
      },
      clones,
    )
  }
}

impl Clone for CloneTracked {
  fn clone(&self) -> Self {
    self.clones.fetch_add(1, Ordering::Relaxed);
    Self {
      value: self.value,
      clones: Arc::clone(&self.clones),
    }
  }
}

impl fmt::Display for CloneTracked {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.value.fmt(f)
  }
}

impl PartialEq for CloneTracked {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

impl Eq for CloneTracked {}

impl PartialOrd for CloneTracked {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for CloneTracked {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.value.cmp(&other.value)
  }
}

impl std::hash::Hash for CloneTracked {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.value.hash(state);
  }
}

#[test]
fn owned_vec_construction_moves_every_element() {
  let mut values = Vec::new();
  let mut counters = Vec::new();
  for value in 0..100 {
    let (item, counter) = CloneTracked::new(value);
    values.push(item);
    counters.push(counter);
  }

  let list = TernaryTreeList::from(values);

  assert_eq!(list.len(), 100);
  assert!(counters.iter().all(|counter| counter.load(Ordering::Relaxed) == 0));
}

#[test]
fn push_right_moves_the_new_element_down_the_update_path() {
  let initial = (0..100).map(|value| CloneTracked::new(value).0).collect::<Vec<_>>();
  let list = TernaryTreeList::from(initial);
  let (new_item, new_item_clones) = CloneTracked::new(100);

  let updated = list.push_right(new_item);

  assert_eq!(updated.last().map(|item| item.value), Some(100));
  assert_eq!(new_item_clones.load(Ordering::Relaxed), 0);
}

#[test]
fn iterator_reports_the_exact_remaining_length() {
  let list = TernaryTreeList::from((0..100).collect::<Vec<_>>());
  let mut iter = list.iter();

  assert_eq!(iter.len(), 100);
  assert_eq!(iter.next(), Some(&0));
  assert_eq!(iter.len(), 99);
  assert_eq!(iter.count(), 99);
}
