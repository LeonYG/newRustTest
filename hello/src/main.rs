
use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    /*#1-5*/
    //rand_num_guess();
    //let a:i32=-12;
    //let b:i32 = 12;
    //println!("{}", operat_1(a,b));
    /*#6 使用有参数和返回值的函数*/
    //let mut sum = operat_1(a, b);
    //sum = 2*sum;
    //println!("2*sum:{}",sum);
    /*#7 Rust中if既是一种语句又是一种表达式。*/
    //exp_ifelse(sum);
    /*8# 在Rust中，loop既是一种语句也是一个表达式*/
    //exp_loop();
    /*Pranctice*/
    //temp_converse();
    let  mut a = 10;
    let  mut b = a;
    b+=1;
    println!("{}",a);
    println!("{}",b);
    let s = String::from("hello");
    println!("s:{}",s);
    let (s,mid) = owner_test(s);
    println!("s in main:{} mid in main{}",s,mid);
    let mid2 = owner_test2(&s);
    println!("mid2 in main:{}",mid2);
    assert!(false);
}
/*chap 2~3---------------------------------------------------------------------
    2：rust工程创建，文件依赖，生成随机数，编写简单的命令行程序
    3：数据类型、函数、分支语句（条件表达式）和循环语句（循环表达式）

    用例参照Rust language手册第2、3章内容
    LeonYG；2020.02
*/
fn owner_test(s:String)->(String,usize){
    let mid = (s.len()/2) as usize;
    //let len = s.len();
    let mid_str = &s[..mid];
    println!("mid:{} str_mid{}",mid,mid_str);
    (s,mid)
}
fn owner_test2(s:&String)->usize{
    let mid = (s.len()/2) as usize;
    //let len = s.len();
    let mid_str = &s[..mid];
    println!("mid:{} str_mid{}",mid,mid_str);
    mid
}
/*返回第一个空格之前的字串，字符串切片类型str，切片会获得一个新的ownership*/
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn rand_num_guess(){
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
}
/*第三章练习题：摄氏度与华氏度转换
    对于string类型的trim使用有疑惑，不能将数字与字母分离，所以要分两行输入30C或30F
*/
fn temp_converse(){
    println!("input temperature as 0C or 0F");
    let mut temp = String::new();
    let mut flag = String::new();
    
    stdin().read_line(&mut temp).expect("stdin error to get temperature");
    stdin().read_line(&mut flag).expect("stdin error to get temperature");

    let output:i32 = temp.trim().parse().expect("parse failed");
    match flag.find('F'){
        Some(_) => {
            let output:f32 = ((output as f32) - 32.0)/1.8;
            println!("result_temp:{}C",output);
        }
        None => {
            let output:f32 = (output as f32)*1.8 + 32.0;
            println!("result_temp:{}F",output);
        }
    }
    
}

/*该接口演示条件表达式*/
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
    for element in a.iter() {//a.iter().rev() 
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
        println!("LIFTOFF!!!");
}
/*该函数展示条件语句的Rust语法基础*/
fn exp_ifelse(sum:i32){
    let mut _sum = if sum>0 {
        0
    }else{
        sum
    };
}

/*chap 4-----------------------------------------------------------------------
*/