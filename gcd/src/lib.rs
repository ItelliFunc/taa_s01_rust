/**
 * 欧几里得算法
 */
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let _t = b;
        b = a % b;
        a = _t;
    }
    a.abs()
}