#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TimeUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeUnitValue {
    pub value: u64,
    pub unit: TimeUnit
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitValue {
    Time(TimeUnitValue)
}