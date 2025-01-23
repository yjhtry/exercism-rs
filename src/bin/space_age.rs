#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / 31557600_f64)
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::period()
    }
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn period() -> f64 {
                $p
            }
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!(
                "Your result of {actual} should be within {delta} of the expected result {expected}"
            )
        }
    }
    #[test]
    fn age_on_earth() {
        let seconds = 1_000_000_000;
        let duration = Duration::from(seconds);
        let output = Earth::years_during(&duration);
        let expected = 31.69;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_mercury() {
        let seconds = 2_134_835_688;
        let duration = Duration::from(seconds);
        let output = Mercury::years_during(&duration);
        let expected = 280.88;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_venus() {
        let seconds = 189_839_836;
        let duration = Duration::from(seconds);
        let output = Venus::years_during(&duration);
        let expected = 9.78;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_mars() {
        let seconds = 2_129_871_239;
        let duration = Duration::from(seconds);
        let output = Mars::years_during(&duration);
        let expected = 35.88;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_jupiter() {
        let seconds = 901_876_382;
        let duration = Duration::from(seconds);
        let output = Jupiter::years_during(&duration);
        let expected = 2.41;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_saturn() {
        let seconds = 2_000_000_000;
        let duration = Duration::from(seconds);
        let output = Saturn::years_during(&duration);
        let expected = 2.15;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_uranus() {
        let seconds = 1_210_123_456;
        let duration = Duration::from(seconds);
        let output = Uranus::years_during(&duration);
        let expected = 0.46;
        assert_in_delta(expected, output);
    }
    #[test]
    fn age_on_neptune() {
        let seconds = 1_821_023_456;
        let duration = Duration::from(seconds);
        let output = Neptune::years_during(&duration);
        let expected = 0.35;
        assert_in_delta(expected, output);
    }
}
