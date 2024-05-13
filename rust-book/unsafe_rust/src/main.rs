use core::slice;
use std::usize;

static mut COUNTER: u32 = 0;

fn add_to_count() {
    unsafe {
        COUNTER += 1;
    }
}

unsafe fn dangerous() {}

fn main() {
    let mut a = 5;

    let r1 = &a as *const i32;
    let r2 = &mut a as *mut i32;

    let address = 0x012345usize;
    let r3 = address as *const usize;

    unsafe {
        println!("{:?}", r1);
        println!("{:?}", r2);
        // println!("{}", *r3);
        dangerous()
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x012345usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };

    // Throw error
    // println!("{:?}", values);

    add_to_count();
    add_to_count();
    add_to_count();

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
