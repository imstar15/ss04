fn sum(nums: &[u32]) -> Option<u32> {
  let mut sum: u32 = 0;
  for num in nums {
    match sum.checked_add(*num) {
      Some(s) => sum = s,
      None => { return None; },
    }
  }
  return Some(sum);
}

fn main() {
  let nums: [u32;3] = [1,2,3];
  let t = sum(&nums);
  print!("t: {:#?}", t);
}