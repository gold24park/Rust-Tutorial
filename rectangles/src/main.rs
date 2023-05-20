// Add attribute to derive the Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation Block
// All functions defined within impl block are "associated functions"
impl Rectangle {
    // Self is alias for the type that the impl block is for.
    // area(self: &Self) == area(&self)
    fn area(&self) -> u32 {
        self.width * self.height
    }    

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! returns ownership of the expression's value
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    // stdout
    println!("rect1 is {:#?}", rect1); // 소유권 반환해줌
    
    // stderr
    dbg!(&rect1); // reference를 사용하지 않으면 소유권 가져가버림

    // field명과 같은 method도 가능하다.
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Rust의 automatic referencing & dereferencing
    // 알잘딱으로 &, &mut, * 같은걸 붙여주기에 아래 두 구문은 결국 같음
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // :: syntax is used for both associated functions, 
    // namespaces created by modules
    let sq = Rectangle::square(3);
}

