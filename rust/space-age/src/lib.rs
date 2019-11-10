// in seconds
const EARTH_ORBITAL_PERIOD_S: f64 = 31_557_600.0;

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
    const ORBITAL_PERIOD: f64; // in Earth years
}

impl<T: OrbitalPeriod> Planet for T {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years() / T::ORBITAL_PERIOD
    }
}

macro_rules! define_planet {
    ($name:ident, $orbital_period:expr) => (
        pub struct $name;
        impl OrbitalPeriod for $name {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    );
}

define_planet!(Mercury, 0.240_846_7);
define_planet!(Venus, 0.615_197_26);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.880_815_8);
define_planet!(Jupiter, 11.862_615);
define_planet!(Saturn, 29.447_498);
define_planet!(Uranus, 84.016_846);
define_planet!(Neptune, 164.79132);
