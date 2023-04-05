#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use polars::prelude::{DataFrame, DataFrameJoinOps, JoinType};

    use crate::{
        common::src::{AnySrc, Src, SrcKind},
        core::{
            context::Context,
            data_provider::DataProvider,
            in_memory_data_provider::InMemoryDataProvider,
            incremental::{Chained, Incremental},
        },
        polars::{io::read_df, series::SeriesCastUtils},
        ta::{
            change::Change, commodity_channel_index::Cci, dev::Dev,
            exponential_moving_average::Ema, highest::Highest, highest_bars::HighestBars,
            lowest::Lowest, lowest_bars::LowestBars, percent_rank::Prank, rate_of_change::Roc,
            relative_strength_index::Rsi, running_moving_average::Rma, simple_moving_average::Sma,
            stdev::Stdev, stoch::Stoch, sum::Sum, symmetrically_weighted_moving_average::Swma,
            weighted_moving_average::Wma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            incremental::test_incremental,
            pace::format_pace_fixture_path,
        },
        utils::float::Float64Utils,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/nan/{}", path))
    }

    struct BarsWrapper {
        parent: Box<dyn Incremental<f64, Option<i32>>>,
    }

    impl BarsWrapper {
        fn new(parent: Box<dyn Incremental<f64, Option<i32>>>) -> Self {
            return Self { parent };
        }
    }

    impl Incremental<f64, f64> for BarsWrapper {
        fn next(&mut self, input: f64) -> f64 {
            let bars = self.parent.next(input);
            return bars.map_or(f64::NAN, |x| x as f64);
        }
    }

    struct StochWrapper {
        parent: Box<Stoch>,
    }

    impl StochWrapper {
        fn new(parent: Box<Stoch>) -> Self {
            return Self { parent };
        }
    }

    impl Incremental<f64, f64> for StochWrapper {
        fn next(&mut self, input: f64) -> f64 {
            let stoch = self.parent.next((input, input, input));
            return stoch;
        }
    }

    fn _test(
        ctx: Context,
        mut target: Box<dyn Incremental<f64, f64>>,
        expected_column: &str,
        df: &DataFrame,
    ) {
        let src = &df.column("_target_src_").unwrap().to_f64();
        let expected = df.column(expected_column).unwrap().to_f64();
        let mut snapshot = ArraySnapshot::<f64>::new();
        snapshot.name = Some(expected_column.to_string());

        for i in ctx.clone() {
            let src = src[i];
            let output = target.next(src);
            snapshot.push(output);
        }
        snapshot.assert(&expected);
    }

    fn _test_all(df: &DataFrame) {
        let get_ctx = || Context::new(InMemoryDataProvider::from_df(&df).to_arc());

        {
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Cci::new(
            //             ctx.clone(),
            //             1,
            //         ).to_box(),
            //         "_target_cci_1_",
            //         &df,
            //     );
            // }
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Cci::new(
            //             ctx.clone(),
            //             2,
            //         ).to_box(),
            //         "_target_cci_2_",
            //         &df,
            //     );
            // }
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Cci::new(
            //             ctx.clone(),
            //             3,
            //         ).to_box(),
            //         "_target_cci_3_",
            //         &df,
            //     );
            // }
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Cci::new(
            //             ctx.clone(),
            //             4,
            //         ).to_box(),
            //         "_target_cci_4_",
            //         &df,
            //     );
            // }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Cci::new(ctx.clone(), 5).to_box(),
                    "_target_cci_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Cci::new(ctx.clone(), 10).to_box(),
                    "_target_cci_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Cci::new(ctx.clone(), 14).to_box(),
                    "_target_cci_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 1).to_box(),
                    "_target_wma_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 2).to_box(),
                    "_target_wma_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 3).to_box(),
                    "_target_wma_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 4).to_box(),
                    "_target_wma_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 5).to_box(),
                    "_target_wma_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 10).to_box(),
                    "_target_wma_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Wma::new(ctx.clone(), 14).to_box(),
                    "_target_wma_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 1).to_box(),
                    "_target_sma_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 2).to_box(),
                    "_target_sma_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 3).to_box(),
                    "_target_sma_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 4).to_box(),
                    "_target_sma_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 5).to_box(),
                    "_target_sma_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 10).to_box(),
                    "_target_sma_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sma::new(ctx.clone(), 14).to_box(),
                    "_target_sma_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 1).to_box(),
                    "_target_ema_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 2).to_box(),
                    "_target_ema_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 3).to_box(),
                    "_target_ema_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 4).to_box(),
                    "_target_ema_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 5).to_box(),
                    "_target_ema_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 10).to_box(),
                    "_target_ema_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Ema::new(ctx.clone(), 14).to_box(),
                    "_target_ema_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 1).to_box(),
                    "_target_rma_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 2).to_box(),
                    "_target_rma_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 3).to_box(),
                    "_target_rma_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 4).to_box(),
                    "_target_rma_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 5).to_box(),
                    "_target_rma_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 10).to_box(),
                    "_target_rma_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rma::new(ctx.clone(), 14).to_box(),
                    "_target_rma_14_",
                    &df,
                );
            }
        }
        {
            let ctx = get_ctx();
            _test(
                ctx.clone(),
                Swma::new(ctx.clone()).to_box(),
                "_target_swma_",
                &df,
            );
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 1).to_box(),
                    "_target_highest_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 2).to_box(),
                    "_target_highest_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 3).to_box(),
                    "_target_highest_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 4).to_box(),
                    "_target_highest_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 5).to_box(),
                    "_target_highest_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 10).to_box(),
                    "_target_highest_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Highest::new(ctx.clone(), 14).to_box(),
                    "_target_highest_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 1).to_box(),
                    "_target_lowest_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 2).to_box(),
                    "_target_lowest_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 3).to_box(),
                    "_target_lowest_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 4).to_box(),
                    "_target_lowest_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 5).to_box(),
                    "_target_lowest_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 10).to_box(),
                    "_target_lowest_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 14).to_box(),
                    "_target_lowest_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 1).to_box()).to_box(),
                    "_target_highestbars_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 2).to_box()).to_box(),
                    "_target_highestbars_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 3).to_box()).to_box(),
                    "_target_highestbars_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 4).to_box()).to_box(),
                    "_target_highestbars_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 5).to_box()).to_box(),
                    "_target_highestbars_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 10).to_box()).to_box(),
                    "_target_highestbars_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(HighestBars::new(ctx.clone(), 14).to_box()).to_box(),
                    "_target_highestbars_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 1).to_box(),
                    "_target_lowest_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 2).to_box(),
                    "_target_lowest_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 3).to_box(),
                    "_target_lowest_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 4).to_box(),
                    "_target_lowest_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 5).to_box(),
                    "_target_lowest_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 10).to_box(),
                    "_target_lowest_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Lowest::new(ctx.clone(), 14).to_box(),
                    "_target_lowest_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 1).to_box()).to_box(),
                    "_target_lowestbars_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 2).to_box()).to_box(),
                    "_target_lowestbars_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 3).to_box()).to_box(),
                    "_target_lowestbars_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 4).to_box()).to_box(),
                    "_target_lowestbars_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 5).to_box()).to_box(),
                    "_target_lowestbars_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 10).to_box()).to_box(),
                    "_target_lowestbars_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    BarsWrapper::new(LowestBars::new(ctx.clone(), 14).to_box()).to_box(),
                    "_target_lowestbars_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 1).to_box(),
                    "_target_change_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 2).to_box(),
                    "_target_change_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 3).to_box(),
                    "_target_change_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 4).to_box(),
                    "_target_change_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 5).to_box(),
                    "_target_change_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 10).to_box(),
                    "_target_change_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Change::new(ctx.clone(), 14).to_box(),
                    "_target_change_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 1).to_box(),
                    "_target_roc_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 2).to_box(),
                    "_target_roc_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 3).to_box(),
                    "_target_roc_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 4).to_box(),
                    "_target_roc_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 5).to_box(),
                    "_target_roc_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 10).to_box(),
                    "_target_roc_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Roc::new(ctx.clone(), 14).to_box(),
                    "_target_roc_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Prank::new(ctx.clone(), 1).to_box(),
                    "_target_prank_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Prank::new(ctx.clone(), 2).to_box(),
                    "_target_prank_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Prank::new(ctx.clone(), 3).to_box(),
                    "_target_prank_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Prank::new(ctx.clone(), 4).to_box(),
                    "_target_prank_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Prank::new(ctx.clone(), 10).to_box(),
                    "_target_prank_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Prank::new(ctx.clone(), 14).to_box(),
                    "_target_prank_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 1, true).to_box(),
                    "_target_biased_stdev_1_",
                    &df,
                );
            }
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Stdev::new(
            //             ctx.clone(),
            //             2,
            //             true,
            //         ).to_box(),
            //         "_target_biased_stdev_2_",
            //         &df,
            //     );
            // }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 3, true).to_box(),
                    "_target_biased_stdev_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 4, true).to_box(),
                    "_target_biased_stdev_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 5, true).to_box(),
                    "_target_biased_stdev_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 10, true).to_box(),
                    "_target_biased_stdev_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 14, true).to_box(),
                    "_target_biased_stdev_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 1, false).to_box(),
                    "_target_unbiased_stdev_1_",
                    &df,
                );
            }
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Stdev::new(
            //             ctx.clone(),
            //             2,
            //             false,
            //         ).to_box(),
            //         "_target_unbiased_stdev_2_",
            //         &df,
            //     );
            // }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 3, false).to_box(),
                    "_target_unbiased_stdev_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 4, false).to_box(),
                    "_target_unbiased_stdev_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 5, false).to_box(),
                    "_target_unbiased_stdev_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 10, false).to_box(),
                    "_target_unbiased_stdev_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Stdev::new(ctx.clone(), 14, false).to_box(),
                    "_target_unbiased_stdev_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 1).to_box(),
                    "_target_dev_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 2).to_box(),
                    "_target_dev_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 3).to_box(),
                    "_target_dev_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 4).to_box(),
                    "_target_dev_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 5).to_box(),
                    "_target_dev_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 10).to_box(),
                    "_target_dev_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Dev::new(ctx.clone(), 14).to_box(),
                    "_target_dev_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 1).to_box(),
                    "_target_sum_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 2).to_box(),
                    "_target_sum_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 3).to_box(),
                    "_target_sum_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 4).to_box(),
                    "_target_sum_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 5).to_box(),
                    "_target_sum_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 10).to_box(),
                    "_target_sum_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Sum::new(ctx.clone(), 14).to_box(),
                    "_target_sum_14_",
                    &df,
                );
            }
        }
        {
            // {
            //     let ctx = get_ctx();
            //     _test(
            //         ctx.clone(),
            //         Rsi::new(
            //             ctx.clone(),
            //             1,
            //         ).to_box(),
            //         "_target_rsi_1_",
            //         &df,
            //     );
            // }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rsi::new(ctx.clone(), 2).to_box(),
                    "_target_rsi_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rsi::new(ctx.clone(), 3).to_box(),
                    "_target_rsi_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rsi::new(ctx.clone(), 4).to_box(),
                    "_target_rsi_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rsi::new(ctx.clone(), 5).to_box(),
                    "_target_rsi_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rsi::new(ctx.clone(), 10).to_box(),
                    "_target_rsi_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    Rsi::new(ctx.clone(), 14).to_box(),
                    "_target_rsi_14_",
                    &df,
                );
            }
        }
        {
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 1).to_box()).to_box(),
                    "_target_stoch_1_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 2).to_box()).to_box(),
                    "_target_stoch_2_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 3).to_box()).to_box(),
                    "_target_stoch_3_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 4).to_box()).to_box(),
                    "_target_stoch_4_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 5).to_box()).to_box(),
                    "_target_stoch_5_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 10).to_box()).to_box(),
                    "_target_stoch_10_",
                    &df,
                );
            }
            {
                let ctx = get_ctx();
                _test(
                    ctx.clone(),
                    StochWrapper::new(Stoch::new(ctx.clone(), 14).to_box()).to_box(),
                    "_target_stoch_14_",
                    &df,
                );
            }
        }
    }

    fn load_df(prefix: &str) -> DataFrame {
        let df0 = read_df(&format_path(&format!("{}0.csv", prefix)));
        let df1 = read_df(&format_path(&format!("{}1.csv", prefix)));
        let df2 = read_df(&format_path(&format!("{}2.csv", prefix)));

        let df0 = df0.slice(0, 200);
        let df1 = df1.slice(0, 200);
        let df2 = df2.slice(0, 200);

        let drop_columns = vec![
            "_target_src_",
            "_bar_index_",
            "_target_condition_",
            "open",
            "high",
            "low",
            "close",
            "volume",
        ];
        let df1 = df1.drop_many(&drop_columns);
        let df2 = df2.drop_many(&drop_columns);

        let df = df0
            .join(&df1, ["time"], ["time"], JoinType::Inner, None)
            .unwrap();

        let df = df
            .join(&df2, ["time"], ["time"], JoinType::Inner, None)
            .unwrap();

        return df;
    }

    #[test]
    fn a() {
        let df = load_df("a");
        _test_all(&df);
    }

    #[test]
    fn b() {
        let df = load_df("b");
        _test_all(&df);
    }

    #[test]
    fn c() {
        let df = load_df("c");
        _test_all(&df);
    }

    #[test]
    fn d() {
        let df = load_df("d");
        _test_all(&df);
    }
}
