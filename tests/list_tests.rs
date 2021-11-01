use std::sync::Arc;
use ternary_tree::TernaryTreeList;

#[test]
fn init_list() -> Result<(), String> {
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3, 4]).to_string(),
    String::from("TernaryTreeList[4, ...]")
  );

  let origin11 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let data11 = TernaryTreeList::init_from(&origin11);

  data11.check_structure()?;

  assert_eq!(
    data11.format_inline(),
    String::from("((1 (2 3 _) 4) (5 6 7) (8 (9 10 _) 11))")
  );
  // assert_eq!(
  //   origin11, [...listToItems(data11)],
  // );

  // assert_eq!(arrayEqual<number>([...listToItems(data11)], [...indexToItems(data11)]));

  let empty_xs: Vec<usize> = vec![];
  assert_eq!(
    TernaryTreeList::init_empty(),
    TernaryTreeList::init_from(&empty_xs)
  );

  Ok(())
}

#[test]
fn list_operations() -> Result<(), String> {
  let origin11 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let data11 = TernaryTreeList::init_from(&origin11);

  // get
  for (idx, v) in origin11.iter().enumerate() {
    assert_eq!(v, &data11.unsafe_get(idx));
  }

  assert_eq!(data11.first(), 1);
  assert_eq!(data11.last(), 11);

  // assoc
  let origin5 = vec![1, 2, 3, 4, 5];
  let data5 = TernaryTreeList::init_from(&origin5);
  let updated = data5.assoc(3, 10);
  assert_eq!(updated.unsafe_get(3), 10);
  assert_eq!(data5.unsafe_get(3), 4);
  assert_eq!(updated.len(), data5.len());

  for idx in 0..data5.len() {
    // echo data5.dissoc(idx).formatInline
    assert_eq!(data5.dissoc(idx).len(), data5.len() - 1);
  }

  assert_eq!(data5.format_inline(), "((1 2 _) 3 (4 5 _))");
  assert_eq!(data5.dissoc(0).format_inline(), "(2 3 (4 5 _))");
  assert_eq!(data5.dissoc(1).format_inline(), "(1 3 (4 5 _))");
  assert_eq!(data5.dissoc(2).format_inline(), "((1 2 _) (4 5 _) _)");
  assert_eq!(data5.dissoc(3).format_inline(), "((1 2 _) 3 5)");
  assert_eq!(data5.dissoc(4).format_inline(), "((1 2 _) 3 4)");

  assert_eq!(TernaryTreeList::init_from(&[1]).rest().format_inline(), "_");
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2]).rest().format_inline(),
    "2"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3])
      .rest()
      .format_inline(),
    "(2 3 _)"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3, 4])
      .rest()
      .format_inline(),
    "((2 3 _) 4 _)"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3, 4, 5])
      .rest()
      .format_inline(),
    "(2 3 (4 5 _))"
  );

  assert_eq!(
    TernaryTreeList::init_from(&[1]).butlast().format_inline(),
    "_"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2])
      .butlast()
      .format_inline(),
    "1"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3])
      .butlast()
      .format_inline(),
    "(1 2 _)"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3, 4])
      .butlast()
      .format_inline(),
    "(1 (2 3 _) _)"
  );
  assert_eq!(
    TernaryTreeList::init_from(&[1, 2, 3, 4, 5])
      .butlast()
      .format_inline(),
    "((1 2 _) 3 4)"
  );

  Ok(())
}

#[test]
fn list_insertions() -> Result<(), String> {
  let origin5 = vec![1, 2, 3, 4, 5];
  let data5 = TernaryTreeList::init_from(&origin5);

  assert_eq!(data5.format_inline(), "((1 2 _) 3 (4 5 _))");

  assert_eq!(
    data5.insert(0, 10, false).format_inline(),
    "(10 ((1 2 _) 3 (4 5 _)) _)"
  );
  assert_eq!(
    data5.insert(0, 10, true).format_inline(),
    "((1 10 2) 3 (4 5 _))"
  );
  assert_eq!(
    data5.insert(1, 10, false).format_inline(),
    "((1 10 2) 3 (4 5 _))"
  );
  assert_eq!(
    data5.insert(1, 10, true).format_inline(),
    "((1 2 10) 3 (4 5 _))"
  );
  assert_eq!(
    data5.insert(2, 10, false).format_inline(),
    "((1 2 _) (10 3 _) (4 5 _))"
  );
  assert_eq!(
    data5.insert(2, 10, true).format_inline(),
    "((1 2 _) (3 10 _) (4 5 _))"
  );
  assert_eq!(
    data5.insert(3, 10, false).format_inline(),
    "((1 2 _) 3 (10 4 5))"
  );
  assert_eq!(
    data5.insert(3, 10, true).format_inline(),
    "((1 2 _) 3 (4 10 5))"
  );
  assert_eq!(
    data5.insert(4, 10, false).format_inline(),
    "((1 2 _) 3 (4 10 5))"
  );
  assert_eq!(
    data5.insert(4, 10, true).format_inline(),
    "(((1 2 _) 3 (4 5 _)) 10 _)"
  );

  let origin4 = [1, 2, 3, 4];
  let data4 = TernaryTreeList::init_from(&origin4);

  assert_eq!(
    data4.assoc_before(3, 10).format_inline(),
    "(1 (2 3 _) (10 4 _))"
  );
  assert_eq!(
    data4.assoc_after(3, 10).format_inline(),
    "(1 (2 3 _) (4 10 _))"
  );

  assert_eq!(
    data4.prepend(10, false).format_inline(),
    "((10 1 _) (2 3 _) 4)"
  );
  assert_eq!(
    data4.append(10, false).format_inline(),
    "(1 (2 3 _) (4 10 _))"
  );

  Ok(())
}

#[test]
fn test_concat() -> Result<(), String> {
  let data1 = TernaryTreeList::init_from(&[1, 2]);
  let data2 = TernaryTreeList::init_from(&[3, 4]);

  let data3 = TernaryTreeList::init_from(&[5, 6]);
  let data4 = TernaryTreeList::init_from(&[7, 8]);

  assert_eq!(
    TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned()]).format_inline(),
    "((1 2 _) (3 4 _) _)"
  );
  assert_eq!(
    TernaryTreeList::concat(&[TernaryTreeList::init_from(&[]), data1.to_owned()]).format_inline(),
    "(1 2 _)"
  );
  assert_eq!(
    TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned(), data3.to_owned()])
      .format_inline(),
    "((1 2 _) (3 4 _) (5 6 _))"
  );
  assert_eq!(
    TernaryTreeList::concat(&[
      data1.to_owned(),
      data2.to_owned(),
      data3.to_owned(),
      data4.to_owned()
    ])
    .format_inline(),
    "((1 2 _) ((3 4 _) (5 6 _) _) (7 8 _))"
  );

  TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned()]).check_structure()?;
  TernaryTreeList::concat(&[data1.to_owned(), data2.to_owned(), data3.to_owned()])
    .check_structure()?;
  TernaryTreeList::concat(&[
    data1.to_owned(),
    data2.to_owned(),
    data3.to_owned(),
    data4.to_owned(),
  ])
  .check_structure()?;

  assert_eq!(
    TernaryTreeList::concat(&[data1, data2, data3, data4]).len(),
    8
  );

  Ok(())
}

#[test]
fn check_equality() -> Result<(), String> {
  let origin4 = [1, 2, 3, 4];
  let data4 = TernaryTreeList::init_from(&origin4);
  let data4n = TernaryTreeList::init_from(&origin4);
  let data4_made = TernaryTreeList::init_from(&[2, 3, 4]).prepend(1, false);

  assert!(data4.same_shape(&data4));
  assert!(data4.same_shape(&data4n));
  assert!(!data4.same_shape(&data4_made));

  assert!(data4 == data4n);
  assert!(data4 == data4_made);
  assert!(data4n == data4_made);
  // assert!(data4 != data4_made); // identical false

  Ok(())
}

#[test]
fn force_balancing() -> Result<(), String> {
  let mut data = TernaryTreeList::<usize>::init_from(&[]);
  for idx in 0..20 {
    data = data.append(idx, true);
  }
  // echo data.formatInline
  assert_eq!(
    data.format_inline(),
    String::from("(((0 1 2) (3 4 5) (6 7 8)) ((9 10 11) (12 13 14) (15 16 17)) (18 19 _))")
  );
  data.force_inplace_balancing();
  assert_eq!(
    data.format_inline(),
    "(((0 1 _) (2 3 4) (5 6 _)) ((7 8 _) (9 10 _) (11 12 _)) ((13 14 _) (15 16 17) (18 19 _)))"
  );
  // echo data.formatInline

  Ok(())
}

#[test]
fn iterator() -> Result<(), String> {
  let origin4 = vec![1, 2, 3, 4];
  let data4 = TernaryTreeList::init_from(&origin4);

  let mut i = 0;
  for _ in data4.to_owned() {
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
  let mut data = TernaryTreeList::init_from(&[]);
  for idx in 0..20 {
    data = data.append(idx, true);
  }

  data.check_structure()?;

  let origin11 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let data11 = TernaryTreeList::init_from(&origin11);

  data11.check_structure()?;

  Ok(())
}

#[test]
fn slices() -> Result<(), String> {
  let mut data = TernaryTreeList::init_from(&[]);
  for idx in 0..40 {
    data = data.append(idx, true);
  }

  let mut list40: Vec<usize> = vec![];
  for idx in 0..40 {
    list40.push(idx);
  }

  for i in 0..40 {
    for j in i..40 {
      assert_eq!(data.slice(i, j).to_vec(), list40[i..j]);
    }
  }
  Ok(())
}

#[test]
fn reverse() -> Result<(), String> {
  let data = TernaryTreeList::init_from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
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
  let data = TernaryTreeList::init_from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  for _ in data {
    i += 1;
  }

  assert_eq!(i, 10);

  Ok(())
}

#[test]
fn index_of() -> Result<(), String> {
  let data = TernaryTreeList::init_from(&[1, 2, 3, 4, 5, 6, 7, 8]);
  assert_eq!(data.index_of(&2), 1);
  assert_eq!(data.find_index(Arc::new(|x| -> bool { x == &2 })), 1);
  assert_eq!(data.index_of(&9), -1);
  assert_eq!(data.find_index(Arc::new(|x| -> bool { x == &9 })), -1);

  Ok(())
}

#[test]
fn map_values() -> Result<(), String> {
  let data = TernaryTreeList::init_from(&[1, 2, 3, 4]);
  let data2 = TernaryTreeList::init_from(&[1, 4, 9, 16]);
  let data3 = data.map(Arc::new(|x| x * x));

  data3.check_structure()?;
  assert_eq!(data2, data3);
  assert_eq!(data2.format_inline(), data3.format_inline());

  Ok(())
}
