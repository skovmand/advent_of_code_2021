// Consider copying this into a general module, or to implement it for Vec<Vec<T>>
pub fn transpose_vec<T: Copy>(vec: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let length = vec[0].len();

    // Scan the 2d vec columns one by one, emit a row for every column scanned vertically
    (0..length)
        .map(|index| vec.iter().map(|row| row[index]).collect())
        .collect()
}
