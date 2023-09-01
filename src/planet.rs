pub(crate) mod orbit;

pub(crate) use orbit::Orbit;

pub(crate) struct Planet {
    pub orbit: Orbit,
}
