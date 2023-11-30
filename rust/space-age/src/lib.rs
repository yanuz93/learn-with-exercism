// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use duration::Duration;
use planet_macro::Planet;
use planet_macro_derive::Planet;

// Mercury: orbital period 0.2408467 Earth years
#[derive(Planet)]
pub struct Mercury;

// Venus: orbital period 0.61519726 Earth years
#[derive(Planet)]
pub struct Venus;

// Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
#[derive(Planet)]
pub struct Earth;

// Mars: orbital period 1.8808158 Earth years
#[derive(Planet)]
pub struct Mars;

// Jupiter: orbital period 11.862615 Earth years
#[derive(Planet)]
pub struct Jupiter;

// Saturn: orbital period 29.447498 Earth years
#[derive(Planet)]
pub struct Saturn;

// Uranus: orbital period 84.016846 Earth years
#[derive(Planet)]
pub struct Uranus;

// Neptune: orbital period 164.79132 Earth years
#[derive(Planet)]
pub struct Neptune;

// impl Planet for Mercury {}
// impl Planet for Venus {}
// impl Planet for Earth {}
// impl Planet for Mars {}
// impl Planet for Jupiter {}
// impl Planet for Saturn {}
// impl Planet for Uranus {}
// impl Planet for Neptune {}
