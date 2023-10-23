use common::MyResult::{MyOk, self};

fn main() {
    let value:Result<String, String> = Ok("Hello".to_string());
    let result = value.expect("Hello");
    println!("{result}");

    let value2:MyResult<String, String> = MyOk("Hello".to_string());
    let result2 = value2.expect("Help");
    println!("{result2}")
}
