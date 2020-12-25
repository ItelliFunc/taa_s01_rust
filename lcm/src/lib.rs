use gcd_recursive::gcd;

/**
 * 最小公倍数
 */
pub fn lcm(a: i32, b: i32) -> i32 {
    (a * b / gcd(a, b)).abs()
}