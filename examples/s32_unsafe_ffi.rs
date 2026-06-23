// Demonstrates raw pointers, `unsafe`, `repr(C)`, and a local `extern "C"` function.

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

extern "C" fn add_one(value: i32) -> i32 {
    value + 1
}

fn increment_via_raw_ptr(value: &mut i32) {
    let ptr: *mut i32 = value;

    // Safety: `ptr` comes from a live `&mut i32`, so it is valid and properly aligned.
    unsafe {
        *ptr += 1;
    }
}

fn round_trip_boxed_point(point: Point) -> Point {
    let raw = Box::into_raw(Box::new(point));

    // Safety: `raw` was created by `Box::into_raw` just above and is converted back exactly once.
    let boxed_again = unsafe { Box::from_raw(raw) };
    *boxed_again
}

fn main() {
    let mut value = 5;
    increment_via_raw_ptr(&mut value);

    let boxed_again = round_trip_boxed_point(Point { x: 1, y: 2 });

    let moved = add_one(41);
    println!("value = {value}, boxed = {boxed_again:?}, moved = {moved}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_via_raw_ptr_updates_value() {
        let mut value = 10;
        increment_via_raw_ptr(&mut value);
        assert_eq!(value, 11);
    }

    #[test]
    fn round_trip_boxed_point_preserves_data() {
        let point = Point { x: 7, y: 9 };
        let result = round_trip_boxed_point(point);
        assert_eq!(result.x, 7);
        assert_eq!(result.y, 9);
    }

    #[test]
    fn add_one_returns_expected_result() {
        assert_eq!(add_one(41), 42);
    }
}