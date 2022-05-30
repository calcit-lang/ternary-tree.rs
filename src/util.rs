pub(crate) fn divide_ternary_sizes(size: usize) -> (usize, usize, usize) {
  let group_size = (size / 3) as usize;
  let extra = size - group_size * 3;

  (group_size, group_size + extra, group_size)
}

pub(crate) fn triple_size(t: u8) -> usize {
  let n: usize = 3;
  n.pow(t as u32)
}
