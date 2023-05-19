pub fn generate_vector<T, F>(count: usize, f: F) -> Vec<T> where F: FnMut(usize) -> T {
    (0..count).map(f).collect()
}
