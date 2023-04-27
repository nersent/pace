use crate::strategy::{strategy::Strategy, trade::TradeDirection};

pub struct PineScriptExportStrategyConfig {
    pub title: String,
    pub currency: String,
    pub risk_free_rate: f64,
    pub include_cobra_metrics: bool,
}

impl Default for PineScriptExportStrategyConfig {
    fn default() -> Self {
        return Self {
            title: "Pace Strategy".to_string(),
            currency: "USD".to_string(),
            risk_free_rate: 0.0,
            include_cobra_metrics: false,
        };
    }
}

pub struct PineScriptExporter {}

impl PineScriptExporter {
    pub fn new() -> Self {
        return Self {};
    }

    fn ps_array_from(&self, values: Vec<String>) -> String {
        let joined_items = values.join(", ");
        return format!("array.from({})", joined_items);
    }

    pub fn strategy(&self, strategy: &Strategy, config: PineScriptExportStrategyConfig) -> String {
        let mut ps_options: Vec<String> = vec![];

        ps_options.push(format!("title = \"{}\"", config.title));
        ps_options.push(format!(
            "initial_capital = {}",
            strategy.config.initial_capital
        ));
        ps_options.push(format!("currency = \"{}\"", config.currency));
        ps_options.push(format!("overlay = false"));
        ps_options.push(format!("pyramiding = 0"));
        ps_options.push(format!("risk_free_rate = {}", config.risk_free_rate));
        ps_options.push(format!(
            "process_orders_on_close = {}",
            strategy.config.on_bar_close
        ));

        if strategy.config.buy_with_equity {
            ps_options.push(format!("default_qty_type = strategy.percent_of_equity"));
            ps_options.push(format!("default_qty_value = 100"));
        }

        let ps_options = ps_options.join(", ");

        let mut ps = format!(
            r#"
//@version=5
strategy({})
"#,
            ps_options
        );

        if config.include_cobra_metrics {
            ps.push_str(
                r#"
import EliCobra/CobraMetrics/2 as table
disp_ind = input.string("Equity", title="Display",options=["Strategy", "Equity", "Open Profit", "Gross Profit", "Net Profit"])
table.cobraTable()
plot(table.curve(disp_ind))
"#,
            );
        }

        let mut offset: i32 = -1;
        // let mut start_tick: i32 = 0;
        // let mut end_tick: i32 = 0;

        let mut long_entries: Vec<i32> = vec![-1];
        let mut long_exits: Vec<i32> = vec![-1];
        let mut short_entries: Vec<i32> = vec![-1];
        let mut short_exits: Vec<i32> = vec![-1];

        for trade in &strategy.trades {
            if trade.entry_tick.is_none() {
                break;
            }

            let entry_tick = trade.entry_tick.unwrap() as i32 + offset;

            if trade.direction == TradeDirection::Long {
                long_entries.push(entry_tick);
                if trade.exit_tick.is_some() {
                    long_exits.push(trade.exit_tick.unwrap() as i32 + offset);
                }
            } else if trade.direction == TradeDirection::Short {
                short_entries.push(entry_tick);
                if trade.exit_tick.is_some() {
                    short_exits.push(trade.exit_tick.unwrap() as i32 + offset);
                }
            }
        }

        let _ps_trades_names = vec!["long_entries", "long_exits", "short_entries", "short_exits"];
        let trades = vec![long_entries, long_exits, short_entries, short_exits]
            .iter()
            .map(|x| self.ps_array_from(x.iter().map(|x| x.to_string()).collect()))
            .collect::<Vec<String>>();
        let trades = _ps_trades_names
            .iter()
            .zip(trades.iter())
            .map(|(name, values)| format!("int[] {} = {}", name, values))
            .collect::<Vec<String>>()
            .join("\n");

        ps.push_str(&format!(
            r#"
// --------- CONSTANTS ----------
{}
"#,
            trades,
        ));

        ps.push_str(&format!(
            r#"
// --------- EXECUTE ----------
if array.indexof(long_entries, bar_index) != -1
    strategy.entry("long_entry", strategy.long)

if array.indexof(long_exits, bar_index) != -1
    strategy.close("long_entry", "long_exit")

if array.indexof(short_entries, bar_index) != -1
    strategy.entry("short_entry", strategy.short)

if array.indexof(short_exits, bar_index) != -1
    strategy.close("short_entry", "short_exit")
"#,
        ));

        return ps.trim().to_string();
    }
}
