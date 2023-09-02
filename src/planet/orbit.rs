use vsop87::{vsop87a, RectangularCoordinates, VSOP87Elements};

pub(crate) type Jd = f64;

pub(crate) trait HasOrbit {
    fn position(&self, jd: Jd) -> RectangularCoordinates;
    fn elements(&self, jd: Jd) -> VSOP87Elements;
}

pub(crate) struct VSOP87Orbit {
    position: fn(Jd) -> RectangularCoordinates,
    elements: fn(Jd) -> VSOP87Elements,
}

impl HasOrbit for VSOP87Orbit {
    fn position(&self, jd: Jd) -> RectangularCoordinates {
        (self.position)(jd)
    }

    fn elements(&self, jd: Jd) -> VSOP87Elements {
        (self.elements)(jd)
    }
}

pub(crate) const VSOP87_MERCURY: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::mercury,
    elements: vsop87::mercury,
};

pub(crate) const VSOP87_VENUS: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::venus,
    elements: vsop87::venus,
};

pub(crate) const VSOP87_EARTH: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::earth_moon,
    elements: vsop87::earth_moon,
};

pub(crate) const VSOP87_MARS: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::mars,
    elements: vsop87::mars,
};

pub(crate) const VSOP87_JUPITER: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::jupiter,
    elements: vsop87::jupiter,
};

pub(crate) const VSOP87_SATURN: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::saturn,
    elements: vsop87::saturn,
};

pub(crate) const VSOP87_URANUS: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::uranus,
    elements: vsop87::uranus,
};

pub(crate) const VSOP87_NEPTUNE: VSOP87Orbit = VSOP87Orbit {
    position: vsop87a::neptune,
    elements: vsop87::neptune,
};
