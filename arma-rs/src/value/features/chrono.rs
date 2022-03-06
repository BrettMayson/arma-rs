use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike};

use crate::{FromArma, IntoArma, Value};

impl IntoArma for NaiveDateTime {
    fn to_arma(&self) -> Value {
        vec![
            self.year() as u32,
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond() / 1_000_000,
        ]
        .to_arma()
    }
}

impl<T: TimeZone> IntoArma for DateTime<T> {
    fn to_arma(&self) -> Value {
        self.naive_utc().to_arma()
    }
}

impl FromArma for NaiveDateTime {
    fn from_arma(s: String) -> Result<Self, String> {
        let arma_date: [i64; 7] = FromArma::from_arma(s)?;
        Ok(NaiveDate::from_ymd(
            arma_date[0].try_into().unwrap(),
            arma_date[1].try_into().unwrap(),
            arma_date[2].try_into().unwrap(),
        )
        .and_hms_milli(
            arma_date[3].try_into().unwrap(),
            arma_date[4].try_into().unwrap(),
            arma_date[5].try_into().unwrap(),
            arma_date[6].try_into().unwrap(),
        ))
    }
}
