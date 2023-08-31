use std::time::UNIX_EPOCH;
use vsop87::vsop87c;

fn main() {
    let t = UNIX_EPOCH.elapsed().unwrap().as_nanos() as i128;
    let jd = t as f64 / (86400.0 * 1000000000.0) + 2440587.5;
    println!("{:?}", vsop87c::mercury(jd));
    println!("{:?}", vsop87c::venus(jd));
    println!("{:?}", vsop87c::earth(jd));
    println!("{:?}", vsop87c::mars(jd));
    println!("{:?}", vsop87c::jupiter(jd));
    println!("{:?}", vsop87c::saturn(jd));
    println!("{:?}", vsop87c::uranus(jd));
    println!("{:?}", vsop87c::neptune(jd));
}
