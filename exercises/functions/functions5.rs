// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}
//为什么删除这个分号就对了
fn square(num: i32) -> i32 {
    num * num
}
