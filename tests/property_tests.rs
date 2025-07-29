use im_ternary_tree::TernaryTreeList;
use proptest::prelude::*;

proptest! {
  #[test]
  fn test_push_right_matches_vec(initial_data in prop::collection::vec(any::<u8>(), 0..1000)) {
    let mut tree = TernaryTreeList::Empty;
    let mut vec = Vec::new();

    for item in &initial_data {
      tree = tree.push_right(*item);
      vec.push(*item);
    }

    prop_assert_eq!(tree.len(), vec.len(), "Length should be consistent");
    prop_assert_eq!(tree.to_vec(), vec, "TernaryTreeList should match Vec after a series of push_right operations");
  }

  #[test]
  fn test_push_left_matches_vec(initial_data in prop::collection::vec(any::<u8>(), 0..1000)) {
    let mut tree = TernaryTreeList::Empty;
    let mut vec = Vec::new();

    for item in &initial_data {
      tree = tree.push_left(*item);
      vec.insert(0, *item);
    }

    prop_assert_eq!(tree.len(), vec.len(), "Length should be consistent");
    prop_assert_eq!(tree.to_vec(), vec, "TernaryTreeList should match Vec after a series of push_left operations");
  }

  #[test]
  fn test_drop_left(initial_data in prop::collection::vec(any::<u8>(), 1..1000)) {
    let mut tree = TernaryTreeList::from(initial_data.clone());
    let mut vec = initial_data;

    // Loop one less time than the length, because the last drop makes the list empty.
    for _ in 0..vec.len() {
      tree = tree.drop_left();
      vec.remove(0);
      prop_assert_eq!(tree.to_vec(), vec.clone(), "Tree should match vec after drop_left");
    }

    prop_assert!(tree.is_empty(), "Tree should be empty after all elements are dropped");
  }

  #[test]
  fn test_drop_right(initial_data in prop::collection::vec(any::<u8>(), 1..1000)) {
    let mut tree = TernaryTreeList::from(initial_data.clone());
    let mut vec = initial_data;

    // Loop one less time than the length, because the last drop makes the list empty.
    for _ in 0..vec.len() {
      tree = tree.drop_right();
      vec.pop();
      prop_assert_eq!(tree.to_vec(), vec.clone(), "Tree should match vec after drop_right");
    }

    prop_assert!(tree.is_empty(), "Tree should be empty after all elements are dropped");
  }
}
