use crate::core::{context::Context, data_provider::AnyDataProvider, incremental::Incremental};

use super::{
    strategy::Strategy,
    trade::{SignalFixture, TradeDirection},
};

pub struct ForceCurveFitConfig {
    pub start_index: usize,
    pub end_index: usize,
}

pub fn force_curve_fit(
    data_provider: AnyDataProvider,
    config: ForceCurveFitConfig,
) -> SignalFixture {
    let mut entries = SignalFixture {
        long_entries: vec![],
        long_exits: vec![],
        short_entries: vec![],
        short_exits: vec![],
    };

    let mut prev_value: Option<bool> = None;

    let offset: usize = 1;

    for i in config.start_index..(config.end_index - offset) {
        let current_close = data_provider.get_close(i + offset);
        let current_open = data_provider.get_open(i + offset);
        let next_close = data_provider.get_close(i + 1 + offset);
        let next_open = data_provider.get_open(i + 1 + offset);

        if current_close.is_nan()
            || current_open.is_nan()
            || next_close.is_nan()
            || next_open.is_nan()
        {
            break;
        }

        let should_buy = next_open >= current_open;

        if prev_value.is_none() || prev_value.unwrap() != should_buy {
            if should_buy {
                entries.long_entries.push(i);
            } else {
                entries.short_entries.push(i);
            }
            prev_value = Some(should_buy);
        }
    }

    return entries;
}
