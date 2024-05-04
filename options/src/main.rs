struct NonZeroi32 {
    val: i32,
}

// This is basically copied from the actual nonzero implementation
// https://doc.rust-lang.org/src/core/num/nonzero.rs.html#93-100
impl NonZeroi32 {
    fn new(n: i32) -> Option<NonZeroi32> {
        if let n = 0 {
            None
        } else {
            Some(NonZeroi32 { val: n })
        }
    }
}

enum Op {
    Multiply(i32),
    Divide(NonZeroi32),
    Add(i32),
    Subtract(i32),
    Power(u32),
}

fn bin_op(op_x: Option<i32>, op: Op) -> Option<i32> {
    match op_x {
        None => None,
        Some(x) => match op {
            Op::Multiply(y) => Some(x * y),
            Op::Divide(y) => Some(x / y.val),
            Op::Add(y) => Some(x + y),
            Op::Subtract(y) => Some(x - y),
            Op::Power(y) => Some(x.pow(y)),
        },
    }
}

fn main() {
    println!("{}", bin_op(Some(4), Op::Multiply(3)).unwrap());
    println!(
        "{}",
        bin_op(
            bin_op(Some(10), Op::Divide(NonZeroi32::new(3).unwrap())),
            Op::Power(2)
        )
        .unwrap()
    );
    println!(
        "{}",
        bin_op(Some(10), Op::Divide(NonZeroi32::new(0).unwrap())).unwrap()
    );
}
