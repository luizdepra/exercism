#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl Duration {
    fn seconds(&self) -> f64 {
        self.seconds
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{ seconds: s as f64 }
    }
}

pub trait Planet {
    const YEAR_IN_SECONDS: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds() / Self::YEAR_IN_SECONDS
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

impl Planet for Mercury {
    const YEAR_IN_SECONDS: f64 = 7_600_543.81992;
}

impl Planet for Venus {
    const YEAR_IN_SECONDS: f64 = 19_414_149.052176;
}

impl Planet for Earth {
    const YEAR_IN_SECONDS: f64 = 31_557_600.0;
}

impl Planet for Mars {
    const YEAR_IN_SECONDS: f64 = 59_354_032.690079994;
}

impl Planet for Jupiter {
    const YEAR_IN_SECONDS: f64 = 374_355_659.124;
}

impl Planet for Saturn {
    const YEAR_IN_SECONDS: f64 = 929_292_362.8848;
}

impl Planet for Uranus {
    const YEAR_IN_SECONDS: f64 = 2_651_370_019.3296;
}

impl Planet for Neptune {
    const YEAR_IN_SECONDS: f64 = 5_200_418_560.032001;
}
