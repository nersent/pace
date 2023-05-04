# PACE Google Sheet Integration

https://pace.nersent.com/

`connect@nersent-pace.iam.gserviceaccount.com`

## Annotations

To use PACE with seamless Google Sheet integration, you need to add annotations to your Google Sheet to dedicated columns.

### Config

Takes value from cell below the one annotated.

- `<nersent_pace::config::on_bar_close>`

- `<nersent_pace::config::initial_capital>`

- `<nersent_pace::config::buy_with_equity>`

- `<nersent_pace::config::risk_free_rate>`

### Data

Takes all values from the column below the one annotated, then truncates to the length of the shortest column.

- `<nersent_pace::data::time>` - unix timestamp in seconds

- `<nersent_pace::data::open>`

- `<nersent_pace::data::high>`

- `<nersent_pace::data::low>`

- `<nersent_pace::data::close>`

- `<nersent_pace::data::volume>`

### Input

Same as with data annotations.

- `<nersent_pace::input::strategy_signal>`

  - `long`
  - `short`
  - `long_entry`
  - `short_entry`
  - `long_exit`
  - `short_exit`

### Output

Modifies cells below the one annotated with values.

- `<nersent_pace::output::time>`

- `<nersent_pace::output::tick>`

- `<nersent_pace::output::equity>` - daily equity

- `<nersent_pace::output::net_equity>`

- `<nersent_pace::output::open_profit>`

- `<nersent_pace::output::returns>` - daily returns

- `<nersent_pace::output::position_size>` - current position size

- `<nersent_pace::output::direction>` - current trade direction

  - `0`: no position
  - `1`: long
  - `-1`: short

- `<nersent_pace::output::logs>` - used for debugging. Includes entry/exit events.

- `<nersent_pace::output::pinescript>` - pinescript code for debugging to be copy-pasted in TradingView

## Setup

1. Add annotations to google sheet

2. Give edit permissions
   `connect@nersent-pace.iam.gserviceaccount.com`

3. Go to
   https://pace.nersent.com/

4. Enter url and worksheet name (from bottom bar)

5. Click on "Update now"
