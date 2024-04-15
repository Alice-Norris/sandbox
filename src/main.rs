use cursive::views::TextView;

fn main() {
    let mut data = TwoNumbers{ a: 0, b: 12};
    
    
}

struct TwoNumbers {
    a: i32,
    b: i32
}

struct Calculator {
    first_op: i32,
    secnd_op: i32,
    func: fn(i32, i32),
    output: i32
}

fn multiply(first_op: i32, secnd_op: i32) -> i32 {
    first_op * secnd_op
}

// fn splash() {
//     let mut sis = cursive::default();

//     sis.add_global_callback('q', |sis| sis.quit());

//     sis.add_layer(TextView::new("Heckin', yeah, cool."));
// }