/// Performs an arithmatic operation on two numbers
///
/// The syntax is as follows
///
///     arithmatic!(Number Operation Number)
#[macro_export]
macro_rules! arithmatic {
    ($first:tt + $second:tt) => {
        println!("{} + {} = {}", $first, $second, $first + $second)
    };
    ($first:tt - $second:tt) => {
        println!("{} - {} = {}", $first, $second, $first - $second)
    };
    ($first:tt * $second:tt) => {
        println!("{} * {} = {}", $first, $second, $first * $second)
    };
    ($first:tt / $second:tt) => {
        println!("{} / {} = {}", $first, $second, $first / $second)
    };
    ($base:tt ^ $power:tt) => {
        println!("{} ^ {} = {}", $base, $power, f64::powf($base, $power))
    };
    ($nth:tt root $number:tt) => {
        println!("The {}th root of {} is {}", $nth, $number, f64::powf($number, 1.0 / $nth))
    };
}


/// Performs a trig operation on a number.
///
/// The general syntax is as follows:
///
///     trig!(Operation Number)
#[macro_export]
macro_rules! trig {
    (sin $number:tt) => {
        println!("Sin({}) = {}", $number, f64::sin($number))
    };
    (cos $number:tt) => {
        println!("Cos({}) = {}", $number, f64::cos($number))
    };
    (tan $number:tt) => {
        println!("Tan({}) = {}", $number, f64::tan($number))
    };
    (asin $number:tt) => {
        println!("Asin({}) = {}", $number, f64::asin($number))
    };
    (acos $number:tt) => {
        println!("Acos({}) = {}", $number, f64::acos($number))
    };
    (atan $number:tt) => {
        println!("Atan({}_ = {}", $number, f64::atan($number))
    };
}


#[macro_export]
macro_rules! stats {
    (average $numbers:tt) => {
        println!("The average is {}",
        $numbers.iter().sum::<f64>() / $numbers.len() as f64)
    };
    (min $numbers:tt) => {
        println!("The minmum value is {}",
        $numbers.iter().cloned().fold(0./0., f64::min))
    };
    (max $numbers:tt) => {
        println!("The maximum value is {}",
        $numbers.iter().cloned().fold(0./0., f64::max))
    }
}