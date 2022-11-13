pub fn pow2(x: usize) -> bool {
    x != 0 && (!(x & (x - 1))) != 0
}
