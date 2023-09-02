pub(crate) mod orbit;

pub(crate) use orbit::HasOrbit;

pub(crate) struct Planet {
    pub orbit: Box<dyn HasOrbit>,
}
