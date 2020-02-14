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
    exp_ifelse(sum);
    /*8# 在Rust中，loop既是一种语句也是一个表达式*/
    exp_loop();
}
fn operat_1(a:i32, b:i32)->i32{
    if a>0 && b>0{
        a+b
    }else{
        (b + a)//.try_into().unwrap()
    }
    
}
/*该接口展示Rust循环语句的语法基础*/
fn exp_loop(){
    //---loop语句
    let mut counter = 0;
    let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
    };
    println!("The result is {}", result);
    //---while条件循环语句
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
    }
    //---for 范围循环
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
/*该函数展示条件语句的Rust语法基础*/
fn exp_ifelse(sum:i32){
    let mut _sum = if sum>0 {
        0
    }else{
        sum
    };
}
