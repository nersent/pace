#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Timeframe {
    Months(usize),
    Weeks(usize),
    Days(usize),
    Hours(usize),
    Seconds(usize),
    Unknown,
}

impl Into<String> for Timeframe {
    fn into(self) -> String {
        return match self {
            Timeframe::Months(value) => format!("{}M", value),
            Timeframe::Weeks(value) => format!("{}W", value),
            Timeframe::Days(value) => format!("{}D", value),
            Timeframe::Hours(value) => format!("{}h", value),
            Timeframe::Seconds(value) => format!("{}s", value),
            Timeframe::Unknown => String::from("UNKNOWN"),
        };
    }
}

// impl TryFrom<usize> for Timeframe {
//     type Error = String;

//     fn try_from(value: usize) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Timeframe::OneDay),
//             1 => Ok(Timeframe::TwelveHours),
//             2 => Ok(Timeframe::FourHours),
//             3 => Ok(Timeframe::OneHour),
//             _ => Err(format!("Invalid timeframe: {}", value)),
//         }
//     }
// }

// impl TryInto<usize> for Timeframe {
//     type Error = String;

//     fn try_into(self) -> Result<usize, Self::Error> {
//         match self {
//             Timeframe::OneDay => Ok(0),
//             Timeframe::TwelveHours => Ok(1),
//             Timeframe::FourHours => Ok(2),
//             Timeframe::OneHour => Ok(3),
//         }
//     }
// }

// impl TryInto<String> for Timeframe {
//     type Error = String;

//     fn try_into(self) -> Result<String, Self::Error> {
//         match self {
//             Timeframe::OneDay => Ok("1D".to_string()),
//             Timeframe::TwelveHours => Ok("12h".to_string()),
//             Timeframe::FourHours => Ok("4h".to_string()),
//             Timeframe::OneHour => Ok("1h".to_string()),
//         }
//     }
// }
