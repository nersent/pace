use crate::core::{context::Context, data_provider::AnyDataProvider, incremental::Incremental};

use super::{
    strategy::Strategy,
    trade::{TradeDirection, TradeEntries},
};

pub struct FitTradesConfig {
    pub start_index: usize,
    pub end_index: usize,
}

pub fn fit_trades(data_provider: AnyDataProvider, config: FitTradesConfig) -> TradeEntries {
    let mut entries = TradeEntries {
        long_entries: vec![],
        long_exits: vec![],
        short_entries: vec![],
        short_exits: vec![],
    };

    let mut prev_value: Option<bool> = None;

    let offset: usize = 1;
    let continous = true;

    for i in config.start_index..(config.end_index - offset) {
        let current_close = data_provider.get_close(i + offset);
        let current_open = data_provider.get_open(i + offset);
        let next_close = data_provider.get_close(i + 1 + offset);
        let next_open = data_provider.get_open(i + 1 + offset);

        if current_close.is_none()
            || current_open.is_none()
            || next_close.is_none()
            || next_open.is_none()
        {
            break;
        }

        let current_close = current_close.unwrap();
        let current_open = current_open.unwrap();
        let next_close = next_close.unwrap();
        let next_open = next_open.unwrap();

        let should_buy = next_open >= current_open;

        if prev_value.is_none() || prev_value.unwrap() != should_buy {
            if should_buy {
                // if !continous && prev_value.is_some() {
                //     entries.short_exits.push(i);
                // }
                entries.long_entries.push(i);
            } else {
                // if !continous && prev_value.is_some() {
                //     entries.long_exits.push(i);
                // }
                entries.short_entries.push(i);
            }
            prev_value = Some(should_buy);
        }

        // if current_trend.is_none() {
        //     current_trend = match current_candle_up {
        //         true => Some(TradeDirection::Long),
        //         false => Some(TradeDirection::Short),
        //     };

        //     if next_candle_up {
        //         entries.long_entries.push(i);
        //     } else {
        //         entries.short_entries.push(i);
        //     }

        //     continue;
        // }

        // let _current_trend = current_trend.unwrap();

        // if current_trend == Some(TradeDirection::Long) && !next_candle_up {
        //     entries.short_entries.push(i);
        //     current_trend = Some(TradeDirection::Short);
        // } else if current_trend == Some(TradeDirection::Short) && next_candle_up {
        //     entries.long_entries.push(i);
        //     current_trend = Some(TradeDirection::Long);
        // }

        // let is_next_candle_up = next_close >= next_open;

        // if is_next_candle_up {
        //     if current_close < current_open {
        //         entries.long_entries.push(i);
        //     } else {
        //         entries.long_exits.push(i);
        //     }
        // } else {
        //     if current_close > current_open {
        //         entries.short_entries.push(i);
        //     } else {
        //         entries.short_exits.push(i);
        //     }
        // }
    }

    return entries;
}
