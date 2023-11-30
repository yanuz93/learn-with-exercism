use duration::Duration;

pub trait Planet {
    fn get_type_name() -> String;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years
    }
}
