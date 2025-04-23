pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = array.len().checked_sub(1)?;

    while l != r {
        let m = (l + r).div_ceil(2);

        if array[m] > key {
            r = m - 1;
        } else {
            l = m;
        }
    }

    if array[l] == key {
        Some(l)
    } else {
        None
    }
}
