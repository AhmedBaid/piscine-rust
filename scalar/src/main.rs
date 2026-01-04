use scalar::*;

fn main() {
    // sum
    println!("sum: {}", sum(234, 2)); // 'sum: 236'

    // diff
    println!("diff: {}", diff(234, 2)); // 'diff: 232'

    // product
    println!("pro: {}", pro(23, 2)); // 'pro: 46'

    // quotient
    println!("quo: {}", quo(22.0, 2.0)); // 'quo: 11'
    println!("quo: {}", quo(-128.23, 2.0)); // 'quo: -64.115'

    // remainder
    println!("rem: {}", rem(22.0, 2.0)); // 'rem: 0'
    println!("rem: {}", rem(-128.23, 2.0)); // 'rem: -0.22999573'
}