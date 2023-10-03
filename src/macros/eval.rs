#[macro_export]
macro_rules! arithmatic {
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

#[macro_export]
macro_rules! trig {
    (sin $number:tt) => {
        println!("Sin({}) = {}", $number, f64::sin($number as f64))
    };
    (cos $number:tt) => {
        println!("Cos({}) = {}", $number, f64::cos($number as f64))
    };
    (tan $number:tt) => {
        println!("Tan({}) = {}", $number, f64::tan($number as f64))
    };
    (asin $number:tt) => {
        println!("Asin({}) = {}", $number, f64::asin($number as f64))
    };
    (acos $number:tt) => {
        println!("Acos({}) = {}", $number, f64::acos($number as f64))
    };
    (atan $number:tt) => {
        println!("Atan({}_ = {}", $number, f64::atan($number as f64))
    };
}