use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    /*#1 生成随机数*/
    let secret_num = rand::thread_rng().gen_range(1,101);
    loop{
        println!("secret_num is:{}",secret_num);
        /*#2 获取命令行字符串*/
        println!("please enter a guess");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("get a guess from stdin faild");
        println!("guess:{}",guess);
        /*#3 字符串->数字 类型强转*/
        /*#5 异常处理：注意对返回值的抓取*/
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("input not a num");
                continue;
            }
        };
        /*#4 对象的cmp接口返回排序器库中的排序结果枚举类型*/
        println!("comparing...");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    let a:i32=-12;
    let b:i32 = 12;
    println!("{}", operat_1(a,b));
    /*#6 使用有参数和返回值的函数*/
    let mut sum = operat_1(a, b);
    sum = 2*sum;
    println!("2*sum:{}",sum);
    /*#7 Rust中if既是一种语句又是一种表达式。*/
    let mut _sum = if sum>0 {
        0
    }else{
        sum
    };
}
fn operat_1(a:i32, b:i32)->i32{
    if a>0 && b>0{
        a+b
    }else{
        (b + a)//.try_into().unwrap()
    }
    
}
