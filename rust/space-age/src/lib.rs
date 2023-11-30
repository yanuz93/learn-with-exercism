// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub struct Duration {
    pub earth_years: f64,
}

impl From<u64> for Duration {
    fn from(value: u64) -> Self {
        Duration {
            earth_years: value as f64 / 31557600 as f64,
        }
    }
}

pub trait Planet {
    fn earth_factor() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::earth_factor()
    }
}

macro_rules! planet_macro {
    ($name:ident, $factor:expr) => {
        pub struct $name; impl Planet for $name {
            fn earth_factor() -> f64 { $factor }
        }
    };
}

// Mercury: orbital period 0.2408467 Earth years
planet_macro!(Mercury, 0.2408467);

// Venus: orbital period 0.61519726 Earth years
planet_macro!(Venus, 0.61519726);

// Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
planet_macro!(Earth, 1.0);

// Mars: orbital period 1.8808158 Earth years
planet_macro!(Mars, 1.8808158);

// Jupiter: orbital period 11.862615 Earth years
planet_macro!(Jupiter, 11.862615);

// Saturn: orbital period 29.447498 Earth years
planet_macro!(Saturn, 29.447498);

// Uranus: orbital period 84.016846 Earth years
planet_macro!(Uranus, 84.016846);

// Neptune: orbital period 164.79132 Earth years
planet_macro!(Neptune, 164.79132);
