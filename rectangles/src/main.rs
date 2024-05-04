#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(side_len: u32) -> Self {
        Self {
            width: side_len,
            height: side_len,
        }
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let bonie = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let sq = Rectangle::square(50);

    let boonie = Rectangle {
        width: 10000,
        height: 100000,
    };

    println!("{}", boonie.can_hold(&bonie));

    println!("Area of rect is {} square pixels.", rect1.area());
    println!("Area of square is {} square pixels.", sq.area());

    println!("Rect is {:#?}", rect1);
}
