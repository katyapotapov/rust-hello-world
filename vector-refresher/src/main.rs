fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // compile time error
    // let mut oktrythis = vec![2, "hi", 2, "ayo", 0x03];
    let mut hi = vec![2, 3, 4];
    hi.push(6);

    let tree = hi[1];
    println!("third element is {tree}");

    // runtime error
    // let thousandth = hi[1000];

    let thousandth = hi.get(1000);
    match thousandth {
        Some(thousandth) => println!("Hey there is one and it's {thousandth}"),
        None => println!("Ok there is nothing there"),
    }

    // causes compile time error if the push line below is uncommented
    let hey = &hi[0];
    // hi.push(8);
    println!("{hey}");

    println!();
    for i in &hi {
        print_type_of(&i);
        println!("{i}");
        // *i += 50;
    }
    println!();
    for i in &mut hi {
        *i += 50;
        println!("{i}");
    }
    println!();

    // this copies ???
    for i in hi {
        print_type_of(&i);
        println!("{i}");
        // *i += 50;
    }

    // revisiting our first non-compiling example with... ENUMS
    #[derive(Debug)]
    enum AnythingGoes {
        Numbers(i32),
        // gets real complicated if we use &str instead, we're not there yet
        Letters(String),
        BiggerNumbers(f64),
    }

    let whatever = vec![
        AnythingGoes::Numbers(5),
        AnythingGoes::Letters("hey".to_string()),
        AnythingGoes::Numbers(6),
        AnythingGoes::BiggerNumbers(3.14),
    ];

    println!("{:?}", whatever);
}
