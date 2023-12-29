#[allow(non_snake_case, clippy::missing_safety_doc)]
pub unsafe fn guessNumber(n: i32) -> i32 {
    bsearch(1, n)
}

unsafe fn bsearch(min: i32, max: i32) -> i32 {
    let current = min + (max - min) / 2;
    match guess(current) {
        -1 => bsearch(min, current - 1),
        0 => current,
        1 => bsearch(current + 1, max),
        _ => panic!(),
    }
}

static mut PICK: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&PICK) {
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Less => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        unsafe {
            PICK = 6;
            assert_eq!(guessNumber(10), 6);
        }
    }

    #[test]
    fn case2() {
        unsafe {
            PICK = 1;
            assert_eq!(guessNumber(1), 1);
        }
    }

    #[test]
    fn case3() {
        unsafe {
            PICK = 1;
            assert_eq!(guessNumber(2), 1);
        }
    }

    #[test]
    fn failed_case1() {
        unsafe {
            PICK = 2;
            assert_eq!(guessNumber(2), 2);
        }
    }
}
