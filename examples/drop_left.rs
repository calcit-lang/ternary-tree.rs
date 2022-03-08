extern crate im_ternary_tree;
use im_ternary_tree::TernaryTreeList;

pub fn main() -> Result<(), String> {
  let mut tree: TernaryTreeList<usize> = TernaryTreeList::Empty;

  for idx in 0..60 {
    tree = tree.push_right(idx);
  }

  for _ in 0..59 {
    tree = tree.drop_left();
    println!("{}", tree.format_inline());
  }

  let mut origin4: Vec<usize> = vec![];
  for idx in 0..60 {
    origin4.push(idx);
  }
  let mut data4 = TernaryTreeList::from(&origin4);

  for _ in 0..59 {
    data4 = data4.drop_left();
    println!("{}", data4.format_inline());
  }

  let mut data = TernaryTreeList::Empty;

  for idx in 0..1000 {
    data = data.push(idx)
  }

  let mut d = data;

  while d.len() > 1 {
    d = d.drop_left();
    println!("{}", d.format_inline());
  }

  Ok(())
}
