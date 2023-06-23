use crate::core::{context::Context, incremental::Incremental};

use super::{change::Change, cum::Cum};

pub struct Pvt {
    pub ctx: Context,
    change: Change,
    cum: Cum,
}

impl Pvt {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx: ctx.clone(),
            change: Change::new(ctx.clone(), 1),
            cum: Cum::new(ctx.clone()),
        }
    }
}

/*pine
f_pvt() =>
    return = cum((change(close) / close[1]) * volume)
*/
impl Incremental<(), f64> for Pvt {
    fn next(&mut self, _: ()) -> f64 {
        let close = self.ctx.bar.close();
        let volume = self.ctx.bar.volume();
        let prev_close = self.ctx.close(1);

        let change = self.change.next(close);

        return self.cum.next((change / prev_close) * volume);
    }
}
