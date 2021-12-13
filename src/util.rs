pub fn divide_ternary_sizes(size: usize) -> (usize, usize, usize) {
  let group_size = (size / 3) as usize;
  let extra = size - group_size * 3;

  (group_size, group_size + extra, group_size)
}
