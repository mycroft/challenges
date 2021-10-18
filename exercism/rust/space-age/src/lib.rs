#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self { Self(s as f64 / 31557600.) }
}

pub trait Planet {
    const COEFF : f64;
    fn years_during(d: &Duration) -> f64 { d.0 / Self::COEFF }
}

#[macro_export]
macro_rules! planet {
    ($x:ident, $y:expr) => { 
        pub struct $x;
        impl Planet for $x { const COEFF : f64 =  $y; }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
