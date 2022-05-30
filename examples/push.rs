extern crate im_ternary_tree;

use im_ternary_tree::TernaryTreeList;

pub fn main() -> Result<(), String> {
  let mut tree: TernaryTreeList<usize> = TernaryTreeList::Empty;

  for idx in 0..80 {
    // println!();
    tree = tree.push_left(idx);
    println!("{}", tree.format_inline());
  }

  Ok(())
}
