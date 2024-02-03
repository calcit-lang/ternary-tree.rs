extern crate im_ternary_tree;

use im_ternary_tree::TernaryTreeList;

pub fn main() -> Result<(), String> {
  let mut tree: TernaryTreeList<usize> = TernaryTreeList::Empty;

  let n = 20000000;

  for idx in 0..n {
    // println!();
    tree = tree.push_left(idx);
    // println!("{}", tree.format_inline());
  }

  println!("{}", tree.len());

  for _ in 0..n {
    // println!();
    tree = tree.drop_right();
    // println!("{}", tree.format_inline());
  }

  println!("{}", tree.len());

  Ok(())
}
