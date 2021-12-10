pub fn divide_ternary_sizes(size: usize) -> (usize, usize, usize) {
  let group_size = (size / 3) as usize;
  let extra = size - group_size * 3;

  (group_size, group_size + extra, group_size)
}

pub fn rough_int_pow(x: usize, times: u16) -> usize {
  if times < 1 {
    return x;
  }

  x.pow(times as u32)
}
