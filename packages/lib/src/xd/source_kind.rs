#[derive(Clone, Copy)]
pub enum SourceKind {
    Open,
    High,
    Low,
    Close,
    Volume,
    OHLC4,
    HLC3,
    HL2,
}
