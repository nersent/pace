#[derive(Clone, Copy)]
pub enum SrcKind {
    Open,
    High,
    Low,
    Close,
    Volume,
    OHLC4,
    HLC3,
    HL2,
}
