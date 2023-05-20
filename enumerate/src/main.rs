enum IpAddr {
    V4(String), 
    V6(String),
}

// struct는 같은 구조로 찍어내야하지만 enum의 advantage임
enum IpAddrWithVariants {
    V4(u8, u8, u8, u8), 
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Named fields
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home1 = IpAddrWithVariants::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 
    // Option type encodes the very common scenario in which
    // a value could be something or it could be nothing

    // Rust doesn't have nulls, but it does have an enum 
    // that can encode the concept of a value being present or absent.
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    // 왜 Option<T>가 Null보다 낫냐?
    // Option<T>와 T는 다른 타입이기 때문에, 
    // compiler는 Option<T>가 유효한 값처럼 사용하게 놔두지 않는다. 
    // ex) Option<i8> + i8 계산 안됨

    // 정말 사용해야할땐 T 값을 꺼내서 써야하기 때문에,
    // you can safely assume that the value isn't null.
}

fn route(ip_kind: IpAddr) {

}