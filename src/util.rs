pub fn divide_ternary_sizes(size: usize) -> (usize, usize, usize) {
  let extra = size % 3;
  let group_size = (size / 3) as usize;
  let mut left_size = group_size;
  let mut middle_size = group_size;
  let mut right_size = group_size;

  match extra {
    0 => {}
    1 => {
      middle_size += 1;
    }
    2 => {
      left_size += 1;
      right_size += 1;
    }
    _ => {
      unreachable!("Unexpected mod result ${extra}");
    }
  }

  (left_size, middle_size, right_size)
}

pub fn rough_int_pow(x: usize, times: u16) -> usize {
  if times < 1 {
    return x;
  }

  x.pow(times as u32)
}
