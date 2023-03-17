// use crate::{
//     components::component_context::ComponentContext,
//     ta::{
//         cross_over_threshold_component::CrossOverThresholdComponent,
//         cross_under_threshold_component::CrossUnderThresholdComponent,
//     },
// };

// pub struct CksStrategyConfig {
//     pub threshold_oversold: f64,
//     pub threshold_overbought: f64,
// }

// impl Default for CksStrategyConfig {
//     fn default() -> Self {
//         return CksStrategyConfig {
//             threshold_oversold: CHANDE_KROLL_STOP_STRATEGY_THRESHOLD_OVERSOLD,
//             threshold_overbought: CHANDE_KROLL_STOP_STRATEGY_THRESHOLD_OVERBOUGHT,
//         };
//     }
// }

// pub struct CksStrategy {
//     pub config: CksStrategyConfig,
//     pub ctx: ComponentContext,
//     cross_over: CrossOverThresholdComponent,
//     cross_under: CrossUnderThresholdComponent,
// }

// pub static CHANDE_KROLL_STOP_STRATEGY_THRESHOLD_OVERSOLD: f64 = 0.0;
// pub static CHANDE_KROLL_STOP_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 0.0;

// impl ChandeKrollStopStrategy {
//     pub fn new(ctx: ComponentContext, config: ChandeKrollStopStrategyConfig) -> Self {
//         todo!("Not implemented yet");
//         return ChandeKrollStopStrategy {
//             ctx: ctx.clone(),
//             cross_over: CrossOverThresholdComponent::new(ctx.clone(), config.threshold_oversold),
//             cross_under: CrossUnderThresholdComponent::new(
//                 ctx.clone(),
//                 config.threshold_overbought,
//             ),
//             config,
//         };
//     }
// }

// impl Component<Option<f64>, Option<TradeDirection>> for ChandeKrollStopStrategy {
//     fn next(&mut self, cmf: Option<f64>) -> Option<TradeDirection> {
//         self.ctx.on_next();

//         let is_cross_over = self.cross_over.next(cmf);
//         let is_cross_under = self.cross_under.next(cmf);

//         let result = if is_cross_over {
//             Some(TradeDirection::Long)
//         } else if is_cross_under {
//             Some(TradeDirection::Short)
//         } else {
//             None
//         };

//         return result;
//     }
// }
