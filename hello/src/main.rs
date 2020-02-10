use std::io;
fn main() {
    //println!("Hello, world!");
    println!("please enter a guess");
    let mut guess = String::new();//.expect("new string creat fail");
    io::stdin().read_line(&mut guess).expect("get a guess from stdin faild");
    println!("guess:{}",guess);
    
}
