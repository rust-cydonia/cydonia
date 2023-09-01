mod planet;

use planet::{orbit, Planet};
use std::time::UNIX_EPOCH;

fn current_jd() -> f64 {
    let t = UNIX_EPOCH.elapsed().unwrap().as_nanos() as i128;
    t as f64 / (86400.0 * 1000000000.0) + 2440587.5
}

fn main() {
    let jd = current_jd();

    let mercury = Planet {
        orbit: orbit::VSOP87_MERCURY,
    };

    let venus = Planet {
        orbit: orbit::VSOP87_VENUS,
    };

    let earth = Planet {
        orbit: orbit::VSOP87_EARTH,
    };

    #[rustfmt::skip] // rustfmt wants to make this one line; ignore for consistency
    let mars = Planet {
        orbit: orbit::VSOP87_MARS,
    };

    let jupiter = Planet {
        orbit: orbit::VSOP87_JUPITER,
    };

    let saturn = Planet {
        orbit: orbit::VSOP87_SATURN,
    };

    let uranus = Planet {
        orbit: orbit::VSOP87_URANUS,
    };

    let neptune = Planet {
        orbit: orbit::VSOP87_NEPTUNE,
    };

    println!("{:?}", (mercury.orbit.position)(jd));
    println!("{:?}", (venus.orbit.position)(jd));
    println!("{:?}", (earth.orbit.position)(jd));
    println!("{:?}", (mars.orbit.position)(jd));
    println!("{:?}", (jupiter.orbit.position)(jd));
    println!("{:?}", (saturn.orbit.position)(jd));
    println!("{:?}", (uranus.orbit.position)(jd));
    println!("{:?}", (neptune.orbit.position)(jd));
}
