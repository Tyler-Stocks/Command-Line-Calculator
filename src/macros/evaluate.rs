#[macro_export]
macro_rules! evalulate {
    ($first:tt + $second:tt) => {
        println!("{} + {} = {}", $first, $second, $first as f64 + $second as f64)
    };
    ($first:tt - $second:tt) => {
        println!("{} - {} = {}", $first, $second, $first as f64 - $second as f64)
    };
    ($first:tt * $second:tt) => {
        println!("{} * {} = {}", $first, $second, $first as f64 * $second as f64)
    };
    ($first:tt / $second:tt) => {
        println!("{} / {} = {}", $first, $second, $first as f64 / $second as f64)
    };
    ($base:tt ** $power:tt) => {
        println!("{} ^ {} = {}", $base, $power, i128::pow($base as i128, $power as u32))
    };
    ($nth:tt root $number:tt) => {
        println!("The {}th root of {} is {}", $nth, $number, i128::pow($nth, (1 / $number) as u32))
    };
}