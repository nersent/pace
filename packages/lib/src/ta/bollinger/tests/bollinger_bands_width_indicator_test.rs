#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        data::polars::SeriesCastUtils,
        ta::{
            aroon::aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
            awesome_oscillator::awesome_oscillator_indicator::{
                AwesomeOscillatorIndicator, AwesomeOscillatorIndicatorConfig,
            },
            balance_of_power::balance_of_power_indicator::BalanceOfPowerIndicator,
            bollinger::{
                bollinger_bands_pb_indicator::{
                    BollingerBandsPercentBIndicator, BollingerBandsPercentBIndicatorConfig,
                },
                bollinger_bands_width_indicator::{
                    BollingerBandsWidthIndicator, BollingerBandsWidthIndicatorConfig,
                },
            },
            moving_average::ma::MovingAverageKind,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut BollingerBandsWidthIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output.value);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_bollinger_bands_width_indicator_btc_1d() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/bollinger/tests/fixtures/bbw/indicator/btc_1d_length_20_sma_mult_2_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut BollingerBandsWidthIndicator::new(
                ctx.clone(),
                BollingerBandsWidthIndicatorConfig {
                    length: 20,
                    mult: 2.0,
                    source: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}