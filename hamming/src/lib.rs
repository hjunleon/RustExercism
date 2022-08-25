/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if  s1.len() != s2.len() {
        return None;
    }
    let mut h_d: usize = 0;
    for idx in 0..s1.len() {
        if s1.chars().nth(idx).unwrap() != s2.chars().nth(idx).unwrap() {
            h_d += 1
        }
    }
    Some(h_d)
    // unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
}
