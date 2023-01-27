use crate::components::component_context::ComponentContext;

use super::{action::StrategyActionKind, trade::StrategyTrade};

pub struct StrategyContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
}

pub struct StrategyContext {
    pub config: StrategyContextConfig,
    pub trades: Vec<StrategyTrade>,
    ctx: ComponentContext,
    prev_action: Option<StrategyActionKind>,
}

impl StrategyContext {
    pub fn new(ctx: ComponentContext, config: StrategyContextConfig) -> Self {
        return StrategyContext {
            config,
            ctx,
            trades: Vec::new(),
            prev_action: None,
        };
    }

    fn create_trade(&mut self, action: StrategyActionKind) {
        assert!(action != StrategyActionKind::None);
        self.trades.push(StrategyTrade {
            direction: action.to_direction(),
            is_filled: false,
            is_closed: false,
            fill_tick: None,
            fill_price: None,
            close_tick: None,
            close_price: None,
        });
    }

    pub fn next(&mut self, action: StrategyActionKind) {
        let prev_action = self.prev_action;
        self.prev_action = Some(action);

        if !self.trades.is_empty() {
            let mut current_trade = self.trades.last_mut().unwrap();

            if (action != StrategyActionKind::None) {
                return;
            }

            if !current_trade.is_filled {
                let ctx = self.ctx.get();

                current_trade.fill_price = ctx.open();
                current_trade.is_filled = true;
                current_trade.fill_tick = Some(ctx.tick());

                return;
            }

            if let Some(prev_action) = prev_action {
                if prev_action != StrategyActionKind::None
                    && !current_trade.is_closed
                    && current_trade.direction != prev_action.to_direction()
                {
                    let ctx = self.ctx.get();

                    current_trade.close_price = ctx.open();
                    current_trade.close_tick = Some(ctx.tick());
                    current_trade.is_closed = true;

                    return;
                }
            }
        }

        if (action != StrategyActionKind::None
            && (self.trades.is_empty() || self.trades.last().unwrap().is_closed))
        {
            self.create_trade(action);
        }
    }
}
