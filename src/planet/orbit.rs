use vsop87::{vsop87a, KeplerianElements, RectangularCoordinates};

pub(crate) type Jd = f64;

pub(crate) trait HasOrbit {
    fn position(&self, jd: Jd) -> RectangularCoordinates;
    fn elements(&self, jd: Jd) -> KeplerianElements;
}

pub(crate) struct VSOP87Orbit {
    position: fn(Jd) -> RectangularCoordinates,
    elements: fn(Jd) -> KeplerianElements,
}

impl HasOrbit for VSOP87Orbit {
    fn position(&self, jd: Jd) -> RectangularCoordinates {
        (self.position)(jd)
    }

    fn elements(&self, jd: Jd) -> KeplerianElements {
        (self.elements)(jd)
    }
}

pub(crate) const VSOP87_MERCURY: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::mercury,
    elements: |jd| KeplerianElements::from(vsop87::mercury(jd)),
};

pub(crate) const VSOP87_VENUS: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::venus,
    elements: |jd| KeplerianElements::from(vsop87::venus(jd)),
};

pub(crate) const VSOP87_EARTH: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::earth_moon,
    elements: |jd| KeplerianElements::from(vsop87::earth_moon(jd)),
};

pub(crate) const VSOP87_MARS: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::mars,
    elements: |jd| KeplerianElements::from(vsop87::mars(jd)),
};

pub(crate) const VSOP87_JUPITER: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::jupiter,
    elements: |jd| KeplerianElements::from(vsop87::jupiter(jd)),
};

pub(crate) const VSOP87_SATURN: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::saturn,
    elements: |jd| KeplerianElements::from(vsop87::saturn(jd)),
};

pub(crate) const VSOP87_URANUS: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::uranus,
    elements: |jd| KeplerianElements::from(vsop87::uranus(jd)),
};

pub(crate) const VSOP87_NEPTUNE: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::neptune,
    elements: |jd| KeplerianElements::from(vsop87::neptune(jd)),
};
