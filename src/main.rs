use std::time::UNIX_EPOCH;
use vsop87::vsop87a;

fn main() {
    let t = UNIX_EPOCH.elapsed().unwrap().as_nanos() as i128;
    let jd = t as f64 / (86400.0 * 1000000000.0) + 2440587.5;
    println!("{:?}", vsop87a::mercury(jd));
    println!("{:?}", vsop87a::venus(jd));
    println!("{:?}", vsop87a::earth(jd));
    println!("{:?}", vsop87a::mars(jd));
    println!("{:?}", vsop87a::jupiter(jd));
    println!("{:?}", vsop87a::saturn(jd));
    println!("{:?}", vsop87a::uranus(jd));
    println!("{:?}", vsop87a::neptune(jd));
}
