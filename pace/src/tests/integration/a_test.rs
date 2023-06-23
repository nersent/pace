#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{rc::Rc, sync::Arc};

    use crate::common::float_series::FloatSeries;
    use crate::common::src::{AnySrc, Src, SrcKind};
    use crate::core::data_provider::DataProvider;
    use crate::core::incremental::Incremental;
    use crate::pinescript::common::PineScriptFloat64;
    use crate::pinescript::float::Float;
    use crate::ta::highest::Highest;
    use crate::ta::lowest::Lowest;
    use crate::testing::fixture::{DataFrameFixtureUtils, Fixture};
    use crate::testing::pace::format_pace_fixture_path;
    use crate::{
        common::fixnan::FixNan,
        core::{context::Context, in_memory_data_provider::InMemoryDataProvider},
        testing::array_snapshot::ArraySnapshot,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/integration/a/{}", path))
    }

    // Src: FDI-Adaptive Supertrend w/ Floating Levels [Loxx] TradingView
    // plot(volume, title="volume")

    // fdip(float src, int per, int speedin)=>
    //     float fmax = ta.highest(src, per)
    //     float fmin = ta.lowest(src,  per)
    //     float length = 0
    //     float diff = 0
    //     for i = 1 to per - 1
    //         diff := (nz(src[i]) - fmin) / (fmax - fmin)
    //         if i > 0
    //             length += nz(diff[i]) - nz(diff[i + 1])
    //     length

    // masterdom = fdip(close, 5, 20)

    // plot(bar_index, title='bar_index')
    // plot(masterdom, title='_target_', color=#ff0000)
    fn _test(ctx: Context, mut src: AnySrc, per: usize, speedin: usize, expected: &[f64]) {
        let mut highest = Highest::new(ctx.clone(), per);
        let mut lowest = Lowest::new(ctx.clone(), per);
        let mut src_series = FloatSeries::new(ctx.clone());

        let mut snapshot = ArraySnapshot::<f64>::new();

        // let mut diff_s = FloatSeries::new(ctx.clone());
        let mut diff = Float::new(ctx.clone());

        for _ in ctx.clone() {
            let _src = src.next(());

            src_series.next(_src);

            let fmax = highest.next(_src);
            let fmin = lowest.next(_src);
            let mut length = 0.0;

            let mut sum = 0.0;
            // let mut diff: Float = Float::new(ctx.clone(), 0.0);

            if ctx.bar.index() < 15 {
                println!("");
            }

            // let mut _diff = f64::NAN;

            for i in 1..=(per - 1) {
                // diff.next(_diff);
                diff.set(((src_series.get(i).ps_nz()) - fmin) / (fmax - fmin));

                // sum += diff[i] * (i as f64);
                // sum = diff[i];
                // sum = src_series.get(i);
                // sum = diff_s.get(50 - 1);

                // if ctx.bar.index() < 15 {
                //     println!(
                //         "[{}] ({}): DIFF[50]: {}",
                //         ctx.bar.index(),
                //         i,
                //         diff_s.get(50),
                //     );
                // }
                if i > 0 {
                    // let _length = diff_s.get(i - 1).ps_nz() - diff_s.get(i + 1 - 1).ps_nz();
                    let _length = diff.get(i) - diff.get(i + 1);
                    length += _length;

                    // if ctx.bar.index() < 15 {
                    //     println!(
                    //         "[{}] ({}): DIFF[0]: {} | DIFF[i]: {} | DIFF[i + 1]: {} | _LENGTH: {} | LENGTH: {}",
                    //         ctx.bar.index(),
                    //         i,
                    //         _diff,
                    //         diff[i],
                    //         diff[i + 1],
                    //         _length,
                    //         length,
                    //     );
                    // }
                }
            }

            diff.next(());

            let target = length;

            snapshot.push(target);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn per_5_speedin_20_close() {
        let (df, ctx) = Fixture::load(&format_path("xd.csv")); // 5_20_close
        _test(
            ctx.clone(),
            Src::new(ctx.clone(), SrcKind::Close).to_box(),
            5,
            20,
            &df.test_target(),
        );
    }
}
