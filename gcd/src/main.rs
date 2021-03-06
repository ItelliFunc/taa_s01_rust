use std::env;

const MSG: &str = "请输入两个整数";

fn main() {
    // 从命令行读取参数并转化为整数
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        dbg!(MSG);
        return
    }
    let a = match args[1].parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            dbg!(MSG);
            return
        }
    };
    let b = match args[2].parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            dbg!(MSG);
            return
        }
    };
    println!("{} 和 {} 的最大公约数为 {}", a, b, gcd::gcd(a, b));
}
