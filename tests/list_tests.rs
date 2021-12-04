use im_ternary_tree::TernaryTreeList;
use std::sync::Arc;

#[test]
fn init_list() -> Result<(), String> {
  assert_eq!(
    TernaryTreeList::from(&[1, 2, 3, 4]).to_string(),
    String::from("TernaryTree[4, ...]")
  );

  let origin11 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let data11 = TernaryTreeList::from(&origin11);

  data11.check_structure()?;

  assert_eq!(data11.format_inline(), String::from("((1 (2 3) 4) (5 6 7) (8 (9 10) 11))"));
  // assert_eq!(
  //   origin11, [...listToItems(data11)],
  // );

  // assert_eq!(arrayEqual<number>([...listToItems(data11)], [...indexToItems(data11)]));

  let empty_xs: Vec<usize> = vec![];
  assert_eq!(TernaryTreeList::Empty, TernaryTreeList::from(empty_xs));

  Ok(())
}

#[test]
fn init_list_push_right() -> Result<(), String> {
  let mut data: Vec<usize> = vec![];
  let mut tree: TernaryTreeList<usize> = TernaryTreeList::Empty;
  for idx in 1..200 {
    data.push(idx);
    tree = tree.push_right(idx);
    assert_eq!(tree, TernaryTreeList::from(data.to_owned()))
  }
  Ok(())
}

#[test]
fn list_operations() -> Result<(), String> {
  let origin11 = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let data11 = TernaryTreeList::from(origin11);

  // get
  for (idx, v) in origin11.iter().enumerate() {
    assert_eq!(*v, data11.loop_get(idx).unwrap());
  }

  assert_eq!(data11.first(), Some(&1));
  assert_eq!(data11.last(), Some(&11));

  // assoc
  let origin5 = &[1, 2, 3, 4, 5];
  let data5 = TernaryTreeList::from(origin5);
  let updated = data5.assoc(3, 10)?;
  assert_eq!(updated.loop_get(3).unwrap(), 10);
  assert_eq!(data5.loop_get(3).unwrap(), 4);
  assert_eq!(updated.len(), data5.len());

  for idx in 0..data5.len() {
    // echo data5.dissoc(idx).formatInline
    assert_eq!(data5.dissoc(idx)?.len(), data5.len() - 1);
  }

  assert_eq!(data5.format_inline(), "((1 2) 3 (4 5))");
  assert_eq!(data5.dissoc(0)?.format_inline(), "(2 3 (4 5))");
  assert_eq!(data5.dissoc(1)?.format_inline(), "(1 3 (4 5))");
  assert_eq!(data5.dissoc(2)?.format_inline(), "((1 2) (4 5))");
  assert_eq!(data5.dissoc(3)?.format_inline(), "((1 2) 3 5)");
  assert_eq!(data5.dissoc(4)?.format_inline(), "((1 2) 3 4)");

  assert_eq!(TernaryTreeList::from(&[1]).rest()?.format_inline(), "_");
  assert_eq!(TernaryTreeList::from(&[1, 2]).rest()?.format_inline(), "2");
  assert_eq!(TernaryTreeList::from(&[1, 2, 3]).rest()?.format_inline(), "(2 3)");
  assert_eq!(TernaryTreeList::from(&[1, 2, 3, 4]).rest()?.format_inline(), "((2 3) 4)");
  assert_eq!(TernaryTreeList::from(&[1, 2, 3, 4, 5]).rest()?.format_inline(), "(2 3 (4 5))");

  assert_eq!(TernaryTreeList::from(&[1]).butlast()?.format_inline(), "_");
  assert_eq!(TernaryTreeList::from(&[1, 2]).butlast()?.format_inline(), "1");
  assert_eq!(TernaryTreeList::from(&[1, 2, 3]).butlast()?.format_inline(), "(1 2)");
  assert_eq!(TernaryTreeList::from(&[1, 2, 3, 4]).butlast()?.format_inline(), "(1 (2 3))");
  assert_eq!(TernaryTreeList::from(&[1, 2, 3, 4, 5]).butlast()?.format_inline(), "((1 2) 3 4)");

  Ok(())
}

#[test]
fn drop_left_data() -> Result<(), String> {
  let mut data: Vec<usize> = vec![];
  for idx in 0..200 {
    data.push(idx);
  }
  let mut tree: TernaryTreeList<usize> = TernaryTreeList::from(data.to_owned());

  // do once less than the length
  for _ in 0..data.len() {
    tree = tree.drop_left();
    data.remove(0);
    assert_eq!(tree, TernaryTreeList::from(data.to_owned()));
  }

  Ok(())
}

#[test]
fn dissoc() -> Result<(), String> {
  let data = TernaryTreeList::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
  assert_eq!(data.format_inline(), "((0 1 2) (3 (4 5) 6) (7 8 9))");
  assert_eq!(data.dissoc(4)?.format_inline(), "((0 1 2) (3 5 6) (7 8 9))");

  Ok(())
}

#[test]
fn list_insertions() -> Result<(), String> {
  let origin5 = [1, 2, 3, 4, 5];
  let data5 = TernaryTreeList::from(&origin5);

  assert_eq!(data5.format_inline(), "((1 2) 3 (4 5))");

  assert_eq!(data5.insert(0, 10, false)?.format_inline(), "(10 ((1 2) 3 (4 5)))");
  assert_eq!(data5.insert(0, 10, true)?.format_inline(), "((1 10 2) 3 (4 5))");
  assert_eq!(data5.insert(1, 10, false)?.format_inline(), "((1 10 2) 3 (4 5))");
  assert_eq!(data5.insert(1, 10, true)?.format_inline(), "((1 2 10) 3 (4 5))");
  assert_eq!(data5.insert(2, 10, false)?.format_inline(), "((1 2) (10 3) (4 5))");
  assert_eq!(data5.insert(2, 10, true)?.format_inline(), "((1 2) (3 10) (4 5))");
  assert_eq!(data5.insert(3, 10, false)?.format_inline(), "((1 2) 3 (10 4 5))");
  assert_eq!(data5.insert(3, 10, true)?.format_inline(), "((1 2) 3 (4 10 5))");
  assert_eq!(data5.insert(4, 10, false)?.format_inline(), "((1 2) 3 (4 10 5))");
  assert_eq!(data5.insert(4, 10, true)?.format_inline(), "(((1 2) 3 (4 5)) 10)");

  let origin4 = [1, 2, 3, 4];
  let data4 = TernaryTreeList::from(&origin4);

  assert_eq!(data4.assoc_before(3, 10)?.format_inline(), "(1 (2 3) (10 4))");
  assert_eq!(data4.assoc_after(3, 10)?.format_inline(), "(1 (2 3) (4 10))");

  assert_eq!(data4.prepend(10, false).format_inline(), "((10 1) (2 3) 4)");
  assert_eq!(data4.append(10, false).format_inline(), "(1 (2 3) (4 10))");

  Ok(())
}

#[test]
fn test_concat() -> Result<(), String> {
  let data1 = TernaryTreeList::from(&[1, 2]);
  let data2 = TernaryTreeList::from(&[3, 4]);

  let data3 = TernaryTreeList::from(&[5, 6]);
  let data4 = TernaryTreeList::from(&[7, 8]);

  assert_eq!(
    TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned()]).format_inline(),
    "((1 2) (3 4))"
  );
  assert_eq!(
    TernaryTreeList::concat(&[TernaryTreeList::from(&[]), data1.to_owned()]).format_inline(),
    "(1 2)"
  );
  assert_eq!(
    TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned(), data3.to_owned()]).format_inline(),
    "((1 2) (3 4) (5 6))"
  );
  assert_eq!(
    TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned(), data3.to_owned(), data4.to_owned()]).format_inline(),
    "((1 2) ((3 4) (5 6)) (7 8))"
  );

  TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned()]).check_structure()?;
  TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned(), data3.to_owned()]).check_structure()?;
  TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned(), data3.to_owned(), data4.to_owned()]).check_structure()?;

  assert_eq!(TernaryTreeList::concat(&[data1, data2, data3, data4]).len(), 8);

  Ok(())
}

#[test]
fn check_equality() -> Result<(), String> {
  let origin4 = [1, 2, 3, 4];
  let data4 = TernaryTreeList::from(&origin4);
  let data4n = TernaryTreeList::from(&origin4);
  let data4_made = TernaryTreeList::from(&[2, 3, 4]).prepend(1, false);

  assert!(data4.is_shape_same(&data4));
  assert!(data4.is_shape_same(&data4n));
  assert!(!data4.is_shape_same(&data4_made));

  assert!(data4 == data4n);
  assert!(data4 == data4_made);
  assert!(data4n == data4_made);
  // assert!(data4 != data4_made); // identical false

  Ok(())
}

#[test]
fn force_balancing() -> Result<(), String> {
  let mut data = TernaryTreeList::<usize>::from(&[]);
  for idx in 0..20 {
    data = data.append(idx, true);
  }
  // echo data.formatInline
  assert_eq!(
    data.format_inline(),
    String::from("((((0 1 2) 3 (4 5 6)) 7 ((8 9 10) 11 (12 13 14))) 15 ((16 17 18) 19))")
  );
  if let Err(msg) = data.force_inplace_balancing() {
    println!("[warning] {}", msg)
  }
  assert_eq!(
    data.format_inline(),
    "(((0 1) (2 3 4) (5 6)) ((7 8) (9 10) (11 12)) ((13 14) (15 16 17) (18 19)))"
  );
  // echo data.formatInline

  Ok(())
}

#[test]
fn iterator() -> Result<(), String> {
  let origin4 = [1, 2, 3, 4];
  let data4 = TernaryTreeList::from(&origin4);

  let mut i = 0;
  for _ in &data4.to_owned() {
    i += 1;
  }

  assert_eq!(i, 4);

  i = 0;
  for (idx, _) in data4.into_iter().enumerate() {
    i += idx;
  }

  assert_eq!(i, 6);

  Ok(())
}

#[test]
fn check_structure() -> Result<(), String> {
  let mut data = TernaryTreeList::from(&[]);
  for idx in 0..20 {
    data = data.append(idx, true);
  }

  data.check_structure()?;

  let origin11 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let data11 = TernaryTreeList::from(&origin11);

  data11.check_structure()?;

  Ok(())
}

#[test]
fn slices() -> Result<(), String> {
  let mut data = TernaryTreeList::from(&[]);
  for idx in 0..40 {
    data = data.append(idx, true);
  }

  let mut list40: Vec<usize> = vec![];
  for idx in 0..40 {
    list40.push(idx);
  }

  for i in 0..40 {
    for j in i..40 {
      assert_eq!(data.slice(i, j)?.to_vec(), list40[i..j]);
    }
  }
  Ok(())
}

#[test]
fn reverse() -> Result<(), String> {
  let data = TernaryTreeList::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  let reversed_data = data.reverse();
  let mut d2 = data.to_vec();
  d2.reverse();
  assert_eq!(d2, reversed_data.to_vec());
  reversed_data.check_structure()?;

  Ok(())
}

#[test]
fn list_traverse() -> Result<(), String> {
  let mut i = 0;
  let data = TernaryTreeList::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  for _ in &data {
    i += 1;
  }

  assert_eq!(i, 10);

  // makre sure &[_] work
  let data2 = &TernaryTreeList::from(&[1]);
  for _ in data2 {
    i += 1;
  }

  for (idx, _) in data2.iter().enumerate() {
    i += idx;
  }

  Ok(())
}

#[test]
fn index_of() -> Result<(), String> {
  let data = TernaryTreeList::from(&[1, 2, 3, 4, 5, 6, 7, 8]);
  assert_eq!(data.index_of(&2), Some(1));
  assert_eq!(data.find_index(Arc::new(|x| -> bool { x == &2 })), Some(1));
  assert_eq!(data.index_of(&9), None);
  assert_eq!(data.find_index(Arc::new(|x| -> bool { x == &9 })), None);

  Ok(())
}

#[test]
fn map_values() -> Result<(), String> {
  let data = TernaryTreeList::from(&[1, 2, 3, 4]);
  let data2 = TernaryTreeList::from(&[1, 4, 9, 16]);
  let data3 = data.map(Arc::new(|x| x * x));

  data3.check_structure()?;
  assert_eq!(data2, data3);
  assert_eq!(data2.format_inline(), data3.format_inline());

  Ok(())
}

#[test]
fn index_elem() -> Result<(), String> {
  let data = TernaryTreeList::from(&[1, 2, 3, 4, 5, 6, 7, 8]);

  assert_eq!(data[0], 1);
  assert_eq!(&data[0], &1);

  Ok(())
}

#[test]
fn take_skip() -> Result<(), String> {
  let data = TernaryTreeList::from(&[1, 2, 3, 4, 5, 6, 7, 8]);

  assert_eq!(data.skip(2).unwrap(), TernaryTreeList::from(&[3, 4, 5, 6, 7, 8]));
  assert_eq!(data.take(2).unwrap(), TernaryTreeList::from(&[1, 2]));

  Ok(())
}
