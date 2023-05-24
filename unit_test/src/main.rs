fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
}

// 예상된 패닉을 캡쳐하고 통과한 테스트로 처리가능
#[test]
#[should_panic]
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}

// ignore test
#[test]
#[ignore = "not yet reviewed by the Q.A team"]
fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(1));
    }
}

fn main() {
    println!("Hello, world!");
}
