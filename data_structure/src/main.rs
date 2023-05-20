struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// "Tuple Structs" Without Named Fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// "Unit-like Structs"
// 타입자체만 쓰려고할때 유용
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 특정 한 필드만 mutable하게 할 순 없음
    user1.email = String::from("anotheremail@example.com");

    // user1.username은 이제사용하지 못한다.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 항상 마지막에 위치해야한다.
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // &str 
    // - Not a Owned Type
    // - String Slice Type
    // - Consists of a pointer into memory and a length
    // - Static Size (size is known at compile time)
    // - 변경할 필요가 없는 문자열에 사용 (함수에 인자로 넘겨 함수 안 에서 읽기만 할때)
    // String 
    // - Owned Type
    // - Dynamic Size (size is unknown at compile time)
    // - 추가하거나 변경이 가능하다.
}

fn build_user(email: String, username: String) -> User {
    User  {
        active: true,
        username,
        email, // field init shorthand syntax
        sign_in_count: 1,
    }
}
