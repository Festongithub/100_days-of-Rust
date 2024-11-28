use std::slice;

fn find_middle_element(values: &mut[i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 6, 8];

    let r = &mut v[..];

    let(a, b) = r.find_middle_element(3);

    assert_eq!(a, &mut[1, 2,3]);
    assert_eq!(b, &mut[4, 6, 8]);
}
