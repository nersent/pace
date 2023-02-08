#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Timeframe {
    OneDay = 0,
    FourHours = 1,
    OneHour = 2,
}

impl TryFrom<usize> for Timeframe {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Timeframe::OneDay),
            1 => Ok(Timeframe::FourHours),
            2 => Ok(Timeframe::OneHour),
            _ => Err(format!("Invalid timeframe: {}", value)),
        }
    }
}

impl TryInto<usize> for Timeframe {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Timeframe::OneDay => Ok(0),
            Timeframe::FourHours => Ok(1),
            Timeframe::OneHour => Ok(2),
        }
    }
}
