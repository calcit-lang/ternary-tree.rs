use im_ternary_tree::TernaryTreeList;

pub fn main() -> Result<(), String> {
  println!("{}", TernaryTreeList::<usize>::from(&[]));

  // assoc
  let origin5 = vec![1, 2, 3, 4, 5];
  let data5 = TernaryTreeList::from(&origin5);
  let updated = data5.assoc(3, 10)?;

  println!("{}", data5.format_inline());
  println!("{}", updated.format_inline());

  assert_eq!(updated.loop_get(3).unwrap(), 10);

  Ok(())
}
