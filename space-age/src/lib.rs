// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub const EARTH_YEAR_SECONDS: f64 = 31_557_600_f64;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / EARTH_YEAR_SECONDS)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ($name: ident, $period: literal) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
