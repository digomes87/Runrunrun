// ==================== Vectors ====================

pub fn vector_of_integer() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 66];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 66]);
}

pub fn vector_sort() {
    let mut vec = vec![1.1_f64, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.total_cmp(b));
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
