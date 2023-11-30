#[derive(Debug)]
pub struct Duration {
    pub earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / 31557600 as f64,
        }
    }
}

