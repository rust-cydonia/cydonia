use vsop87::{vsop87a, RectangularCoordinates, VSOP87Elements};

pub(crate) struct Orbit {
    pub position: fn(f64) -> RectangularCoordinates,
    pub elements: fn(f64) -> VSOP87Elements,
}

pub(crate) const VSOP87_MERCURY: Orbit = Orbit {
    position: vsop87a::mercury,
    elements: vsop87::mercury,
};

pub(crate) const VSOP87_VENUS: Orbit = Orbit {
    position: vsop87a::venus,
    elements: vsop87::venus,
};

pub(crate) const VSOP87_EARTH: Orbit = Orbit {
    position: vsop87a::earth_moon,
    elements: vsop87::earth_moon,
};

pub(crate) const VSOP87_MARS: Orbit = Orbit {
    position: vsop87a::mars,
    elements: vsop87::mars,
};

pub(crate) const VSOP87_JUPITER: Orbit = Orbit {
    position: vsop87a::jupiter,
    elements: vsop87::jupiter,
};

pub(crate) const VSOP87_SATURN: Orbit = Orbit {
    position: vsop87a::saturn,
    elements: vsop87::saturn,
};

pub(crate) const VSOP87_URANUS: Orbit = Orbit {
    position: vsop87a::uranus,
    elements: vsop87::uranus,
};

pub(crate) const VSOP87_NEPTUNE: Orbit = Orbit {
    position: vsop87a::neptune,
    elements: vsop87::neptune,
};
