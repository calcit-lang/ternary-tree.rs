extern crate im_ternary_tree;

use im_ternary_tree::TernaryTreeList;

pub fn main() -> Result<(), String> {
  let mut tree: TernaryTreeList<usize> = TernaryTreeList::Empty;

  for idx in 0..4 {
    // println!();
    tree = tree.push_left(idx);
  }

  println!("tree: {}", tree.format_inline());

  for x in tree {
    println!("{}", x);
  }

  // println!("{}", tree.format_inline());

  Ok(())
}
