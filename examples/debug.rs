extern crate im_ternary_tree;

use im_ternary_tree::TernaryTreeList;

pub fn main() -> Result<(), String> {
  println!("{}", TernaryTreeList::<usize>::from(&vec![]));

  // assoc
  // let origin5 = [1, 2, 3, 4, 5];
  // let data5 = TernaryTreeList::from(&origin5);
  // let updated = data5.assoc(3, 10)?;

  // println!("{}", data5.format_inline());
  // println!("{}", updated.format_inline());

  // assert_eq!(updated.loop_get(3).unwrap(), &10);

  // let mut data: Vec<usize> = vec![];
  // for idx in 0..100000 {
  //   data.push(idx)
  // }
  // let tree = TernaryTreeList::from(&data);

  // for _ in 0..1000 {
  //   for idx in 0..1000 {
  //     let _ = tree.loop_get(idx);
  //   }
  // }

  // for _ in 0..1000 {
  //   for idx in 0..1000 {
  //     let _ = tree.ref_get(idx);
  //   }
  // }

  // for _ in 0..1000 {
  //   let mut data = tree.to_owned();
  //   for _ in 0..1000 {
  //     data = data.rest().unwrap();
  //   }
  // }

  // for _ in 0..10 {
  //   let mut data: TernaryTreeList<usize> = TernaryTreeList::Empty;
  //   for idx in 0..10000 {
  //     data = data.push(idx);
  //   }
  // }

  // let mut data: Vec<usize> = vec![];
  // for idx in 0..110 {
  //   data.push(idx);
  //   let tree = TernaryTreeList::from(&data);
  //   println!("{}", tree.format_inline());
  // }

  let mut data: TernaryTreeList<usize> = TernaryTreeList::Empty;
  data = data.push_right(0);
  let _e = data.dissoc(0)?;

  Ok(())
}
