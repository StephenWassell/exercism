// in seconds
const EARTH_ORBITAL_PERIOD_S: f64 = 31_557_600.0;

// in Earth years
const MERCURY_ORBITAL_PERIOD: f64 = 0.240_846_7;
const VENUS_ORBITAL_PERIOD: f64 = 0.615_197_26;
const MARS_ORBITAL_PERIOD: f64 = 1.880_815_8;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862_615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447_498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016_846;
const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

impl Duration {
    fn earth_years(&self) -> f64 {
        self.s as f64 / EARTH_ORBITAL_PERIOD_S
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub trait OrbitalPeriod {
    fn orbital_period() -> f64;
}

impl<T: OrbitalPeriod> Planet for T {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / T::orbital_period()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl OrbitalPeriod for Mercury {
    fn orbital_period() -> f64 {
        MERCURY_ORBITAL_PERIOD
    }
}

impl OrbitalPeriod for Venus {
    fn orbital_period() -> f64 {
        VENUS_ORBITAL_PERIOD
    }
}

impl OrbitalPeriod for Earth {
    fn orbital_period() -> f64 {
        1.0
    }
}
impl OrbitalPeriod for Mars {
    fn orbital_period() -> f64 {
        MARS_ORBITAL_PERIOD
    }
}

impl OrbitalPeriod for Jupiter {
    fn orbital_period() -> f64 {
        JUPITER_ORBITAL_PERIOD
    }
}

impl OrbitalPeriod for Saturn {
    fn orbital_period() -> f64 {
        SATURN_ORBITAL_PERIOD
    }
}

impl OrbitalPeriod for Uranus {
    fn orbital_period() -> f64 {
        URANUS_ORBITAL_PERIOD
    }
}

impl OrbitalPeriod for Neptune {
    fn orbital_period() -> f64 {
        NEPTUNE_ORBITAL_PERIOD
    }
}
