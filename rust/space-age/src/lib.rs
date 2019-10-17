// in seconds
const EARTH_ORBITAL_PERIOD_S: f64 = 31557600.0;

// in Earth years
const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / MERCURY_ORBITAL_PERIOD
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / VENUS_ORBITAL_PERIOD
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years()
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / MARS_ORBITAL_PERIOD
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / JUPITER_ORBITAL_PERIOD
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / SATURN_ORBITAL_PERIOD
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / URANUS_ORBITAL_PERIOD
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / NEPTUNE_ORBITAL_PERIOD
    }
}
