use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    /*#1 生成随机数*/
    let secret_num = rand::thread_rng().gen_range(1,101);
    println!("secret_num is:{}",secret_num);
    /*#2 获取命令行字符串*/
    println!("please enter a guess");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("get a guess from stdin faild");
    println!("guess:{}",guess);
    /*#3 字符串->数字 类型强转*/
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    /*#4 对象的cmp接口返回排序器库中的排序结果枚举类型*/
    println!("comparing...");
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
        }

}
