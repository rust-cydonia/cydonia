use glam::f64::DVec3;
use std::time::UNIX_EPOCH;

#[derive(Clone, Copy)]
struct Orbit {
    /// longitude of the ascending node
    asc_node: f64,
    /// inclination to the ecliptic (plane of the Earth's orbit)
    inc: f64,
    /// argument of perihelion
    arg_of_periapsis: f64,
    /// semi-major axis, or mean distance from Sun
    sma: f64,
    /// eccentricity (0=circle, 0-1=ellipse, 1=parabola)
    ecc: f64,
    /// mean anomaly (0 at perihelion; increases uniformly with time)
    mean_anomaly: f64,
    /// perturbation of longitude (used for Jupiter, Saturn and Uranus)
    lon_pert: Option<f64>,
    /// perturbation of latitude (used for Saturn)
    lat_pert: Option<f64>,
}

impl Orbit {
    fn eccentric_anomaly(&self) -> f64 {
        let Self {
            ecc, mean_anomaly, ..
        } = self;

        let mean_anomaly = mean_anomaly.to_radians();

        let mut prev = mean_anomaly + ecc * mean_anomaly.sin() * (1.0 + ecc * mean_anomaly.cos());
        let mut ecc_anomaly =
            prev - (prev - ecc * prev.sin() - mean_anomaly) / (1.0 - ecc * prev.sin());

        while f64::abs(ecc_anomaly - prev) > f64::to_radians(0.001) {
            prev = ecc_anomaly;
            ecc_anomaly =
                prev - (prev - ecc * prev.sin() - mean_anomaly) / (1.0 - ecc * prev.sin());
        }
        ecc_anomaly
    }

    fn true_anomaly_and_distance(&self) -> (f64, f64) {
        let Self { sma, ecc, .. } = self;

        let ecc_anomaly = self.eccentric_anomaly();

        let x_true_anomaly = sma * (ecc_anomaly.cos() - ecc);
        let y_true_anomaly = sma * (f64::sqrt(1.0 - ecc * ecc) * ecc_anomaly.sin());

        let true_anomaly = f64::atan2(y_true_anomaly, x_true_anomaly);
        let dist = f64::sqrt(x_true_anomaly * x_true_anomaly + y_true_anomaly * y_true_anomaly);
        (true_anomaly, dist)
    }

    fn position(&self) -> DVec3 {
        let Self {
            asc_node,
            inc,
            arg_of_periapsis,
            lon_pert,
            lat_pert,
            ..
        } = self;

        let (true_anomaly, dist) = self.true_anomaly_and_distance();

        let asc_node = asc_node.to_radians();
        let inc = inc.to_radians();
        let arg_of_periapsis = arg_of_periapsis.to_radians();

        let x = dist
            * (asc_node.cos() * f64::cos(true_anomaly + arg_of_periapsis)
                - asc_node.sin() * f64::sin(true_anomaly + arg_of_periapsis) * inc.cos());
        let y = dist
            * (asc_node.sin() * f64::cos(true_anomaly + arg_of_periapsis)
                + asc_node.cos() * f64::sin(true_anomaly + arg_of_periapsis) * inc.cos());
        let z = dist * f64::sin(true_anomaly + arg_of_periapsis) * inc.sin();
        let pos = DVec3::new(x, y, z);

        apply_perturbation(
            pos,
            lon_pert.map(f64::to_radians),
            lat_pert.map(f64::to_radians),
            dist,
        )
    }
}

#[must_use]
fn apply_perturbation(
    mut pos: DVec3,
    lon_pert: Option<f64>,
    lat_pert: Option<f64>,
    dist: f64,
) -> DVec3 {
    if lon_pert.is_none() || lat_pert.is_none() {
        let lon_ecl = f64::atan2(pos.y, pos.x) + lon_pert.unwrap_or_default();
        let lat_ecl = f64::atan2(pos.z, f64::sqrt(pos.x * pos.x + pos.y * pos.y))
            + lat_pert.unwrap_or_default();
        pos.x = dist * lon_ecl.cos() * lat_ecl.cos();
        pos.y = dist * lon_ecl.sin() * lat_ecl.cos();
        pos.z = dist * lat_ecl.sin();
    }
    pos
}

fn mercury_orbit(d: f64) -> Orbit {
    Orbit {
        asc_node: 48.3313 + 3.24587e-5 * d,
        inc: 7.0047 + 5.00e-8 * d,
        arg_of_periapsis: 29.1241 + 1.01444e-5 * d,
        sma: 0.387098,
        ecc: 0.205635 + 5.59e-10 * d,
        mean_anomaly: (168.6562 + 4.0923344368 * d) % 360.0,
        lon_pert: None,
        lat_pert: None,
    }
}

fn venus_orbit(d: f64) -> Orbit {
    Orbit {
        asc_node: 76.6799 + 2.46590e-5 * d,
        inc: 3.3946 + 2.75e-8 * d,
        arg_of_periapsis: 54.8910 + 1.38374e-5 * d,
        sma: 0.723330,
        ecc: 0.006773 - 1.302e-9 * d,
        mean_anomaly: (48.0052 + 1.6021302244 * d) % 360.0,
        lon_pert: None,
        lat_pert: None,
    }
}

fn earth_orbit(d: f64) -> Orbit {
    Orbit {
        asc_node: 0.0,
        inc: 0.0,
        arg_of_periapsis: 282.9404 + 180.0 + 4.70935e-5 * d,
        sma: 1.000000,
        ecc: 0.016709 - 1.151e-9 * d,
        mean_anomaly: (356.0470 + 0.9856002585 * d) % 360.0,
        lon_pert: None,
        lat_pert: None,
    }
}

fn mars_orbit(d: f64) -> Orbit {
    Orbit {
        asc_node: 49.5574 + 2.11081e-5 * d,
        inc: 1.8497 - 1.78e-8 * d,
        arg_of_periapsis: 286.5016 + 2.92961e-5 * d,
        sma: 1.523688,
        ecc: 0.093405 + 2.516e-9 * d,
        mean_anomaly: (18.6021 + 0.5240207766 * d) % 360.0,
        lon_pert: None,
        lat_pert: None,
    }
}

fn jupiter_orbit(d: f64) -> Orbit {
    let jupiter_mean_anomaly = (19.8950 + 0.0830853001 * d) % 360.0;
    let saturn_mean_anomaly = (316.9670 + 0.0334442282 * d) % 360.0;
    Orbit {
        asc_node: 100.4542 + 2.76854e-5 * d,
        inc: 1.3030 - 1.557e-7 * d,
        arg_of_periapsis: 273.8777 + 1.64505e-5 * d,
        sma: 5.20256,
        ecc: 0.048498 + 4.469e-9 * d,
        mean_anomaly: jupiter_mean_anomaly,
        lon_pert: Some(
            -0.332
                * f64::sin(f64::to_radians(
                    2.0 * jupiter_mean_anomaly - 5.0 * saturn_mean_anomaly - 67.6,
                ))
                - 0.056
                    * f64::sin(f64::to_radians(
                        2.0 * jupiter_mean_anomaly - 2.0 * saturn_mean_anomaly + 21.0,
                    ))
                + 0.042
                    * f64::sin(f64::to_radians(
                        3.0 * jupiter_mean_anomaly - 5.0 * saturn_mean_anomaly + 21.0,
                    ))
                - 0.036
                    * f64::sin(f64::to_radians(
                        jupiter_mean_anomaly - 2.0 * saturn_mean_anomaly,
                    ))
                + 0.022 * f64::cos(f64::to_radians(jupiter_mean_anomaly - saturn_mean_anomaly))
                + 0.023
                    * f64::sin(f64::to_radians(
                        2.0 * jupiter_mean_anomaly - 3.0 * saturn_mean_anomaly + 52.0,
                    ))
                - 0.016
                    * f64::sin(f64::to_radians(
                        jupiter_mean_anomaly - 5.0 * saturn_mean_anomaly - 69.0,
                    )),
        ),
        lat_pert: None,
    }
}

fn saturn_orbit(d: f64) -> Orbit {
    let saturn_mean_anomaly = (316.9670 + 0.0334442282 * d) % 360.0;
    let jupiter_mean_anomaly = (19.8950 + 0.0830853001 * d) % 360.0;
    Orbit {
        asc_node: 113.6634 + 2.38980e-5 * d,
        inc: 2.4886 - 1.081e-7 * d,
        arg_of_periapsis: 339.3939 + 2.97661e-5 * d,
        sma: 9.55475,
        ecc: 0.055546 - 9.499e-9 * d,
        mean_anomaly: saturn_mean_anomaly,
        lon_pert: Some(
            0.812
                * f64::sin(f64::to_radians(
                    2.0 * jupiter_mean_anomaly - 5.0 * saturn_mean_anomaly - 67.6,
                ))
                - 0.229
                    * f64::cos(f64::to_radians(
                        2.0 * jupiter_mean_anomaly - 4.0 * saturn_mean_anomaly - 2.0,
                    ))
                + 0.119
                    * f64::sin(f64::to_radians(
                        jupiter_mean_anomaly - 2.0 * saturn_mean_anomaly - 3.0,
                    ))
                + 0.046
                    * f64::sin(f64::to_radians(
                        2.0 * jupiter_mean_anomaly - 6.0 * saturn_mean_anomaly - 69.0,
                    ))
                + 0.014
                    * f64::sin(f64::to_radians(
                        jupiter_mean_anomaly - 3.0 * saturn_mean_anomaly + 32.0,
                    )),
        ),
        lat_pert: Some(
            -0.020
                * f64::cos(f64::to_radians(
                    2.0 * jupiter_mean_anomaly - 4.0 * saturn_mean_anomaly - 2.0,
                ))
                + 0.018
                    * f64::sin(f64::to_radians(
                        2.0 * jupiter_mean_anomaly - 6.0 * saturn_mean_anomaly - 49.0,
                    )),
        ),
    }
}

fn uranus_orbit(d: f64) -> Orbit {
    let uranus_mean_anomaly = (142.5905 + 0.011725806 * d) % 360.0;
    let jupiter_mean_anomaly = (19.8950 + 0.0830853001 * d) % 360.0;
    let saturn_mean_anomaly = (316.9670 + 0.0334442282 * d) % 360.0;
    Orbit {
        asc_node: 74.0005 + 1.3978e-5 * d,
        inc: 0.7733 + 1.9e-8 * d,
        arg_of_periapsis: 96.6612 + 3.0565e-5 * d,
        sma: 19.18171 - 1.55e-8 * d,
        ecc: 0.047318 + 7.45e-9 * d,
        mean_anomaly: uranus_mean_anomaly,
        lon_pert: Some(
            0.040
                * f64::sin(f64::to_radians(
                    saturn_mean_anomaly - 2.0 * uranus_mean_anomaly + 6.0,
                ))
                + 0.035
                    * f64::sin(f64::to_radians(
                        saturn_mean_anomaly - 3.0 * uranus_mean_anomaly + 33.0,
                    ))
                - 0.015
                    * f64::sin(f64::to_radians(
                        jupiter_mean_anomaly - uranus_mean_anomaly + 20.0,
                    )),
        ),
        lat_pert: None,
    }
}

fn neptune_orbit(d: f64) -> Orbit {
    Orbit {
        asc_node: 131.7806 + 3.0173e-5 * d,
        inc: 1.7700 - 2.55e-7 * d,
        arg_of_periapsis: 272.8461 - 6.027e-6 * d,
        sma: 30.05826 + 3.313e-8 * d,
        ecc: 0.008606 + 2.15e-9 * d,
        mean_anomaly: (260.2471 + 0.005995147 * d) % 360.0,
        lon_pert: None,
        lat_pert: None,
    }
}

fn main() {
    let t = UNIX_EPOCH.elapsed().unwrap().as_nanos() as i128 - 946598400 * 1000000000;
    let d = t as f64 / (86400.0 * 1000000000.0);
    println!("{:?}", mercury_orbit(d).position());
    println!("{:?}", venus_orbit(d).position());
    println!("{:?}", earth_orbit(d).position());
    println!("{:?}", mars_orbit(d).position());
    println!("{:?}", jupiter_orbit(d).position());
    println!("{:?}", saturn_orbit(d).position());
    println!("{:?}", uranus_orbit(d).position());
    println!("{:?}", neptune_orbit(d).position());
}
