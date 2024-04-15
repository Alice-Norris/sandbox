use cursive::views::TextView;

fn main() {
    let data = TwoNumbers{ a: 0, b: 12};
    let calc = Calculator{ 
        first_op: None,
        secnd_op: None,
        func: multiply,
        output: None
    };
    data.exec(&calc);

    println!("{}", calc.write_string());
}

struct TwoNumbers {
    a: i32,
    b: i32,
}
impl TwoNumbers {
    pub fn exec(&self, calc: &Calculator) {
        (calc.func)(Some(8), Some(8));
    }
}

struct Calculator {
    first_op: Option<i32>,
    secnd_op: Option<i32>,
    func: fn(Option<i32>, Option<i32>) -> Option<i32>,
    output: Option<i32>
}
impl Calculator {

    fn set_op_1(&mut self, num: i32) {
        self.first_op = Some(num)
    }

    fn set_op_2(&mut self, num: i32) {
        self.secnd_op = Some(num)
    }

    fn calculate(&mut self) {
        self.output = (self.func)(self.first_op, self.secnd_op)
    }

    fn write_string(&self) -> String{
        format!("{} * {} = {}", self.first_op.unwrap(), self.secnd_op.unwrap(), self.output.unwrap())
    }
}

fn multiply(first_op: Option<i32>, secnd_op: Option<i32>) -> Option<i32> {
    if first_op.is_some() & secnd_op.is_some() {
        Some(first_op.unwrap() * secnd_op.unwrap())
    } else {
        return None
    }
}

// fn splash() {
//     let mut sis = cursive::default();

//     sis.add_global_callback('q', |sis| sis.quit());

//     sis.add_layer(TextView::new("Heckin', yeah, cool."));
// }