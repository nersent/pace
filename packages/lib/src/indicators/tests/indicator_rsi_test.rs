#[cfg(test)]
mod tests {
    use crate::{
        base::{
            component_context::ComponentContext,
            implicit::{
                recursive::{recursive_rsi::RecursiveRSI, recursive_sma::RecursiveSMA},
                source::{Source, SourceKind},
            },
            utils::testing::{load_test_artifact_with_target, ComponentTestSnapshot},
        },
        indicators::indicator_rsi::{IndicatorRSI, IndicatorRSIConfig},
    };

    fn _test(cctx: &mut ComponentContext, target: &mut IndicatorRSI, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output.rsi);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_rsi_btc_1d_length14_open() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("indicators/rsi/btc_1d_length_14_open.csv");
        _test(
            &mut ctx.clone(),
            &mut IndicatorRSI::new(
                ctx.clone(),
                IndicatorRSIConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Open),
                },
            ),
            &expected,
        );
    }
}
