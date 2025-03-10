export enum Timeframe {
    M1 = "M1",
    M5 = "M5",
    M15 = "M15",
    H1 = "H1",
    H6 = "H6",
    D1 = "D1"
}

export enum TimeUnit {
    SECOND = "SECOND",
    MINUTE = "MINUTE",
    HOUR = "HOUR",
    DAY = "DAY"
}

export const ALL_TIMEFRAMES = [
    Timeframe.M1,
    Timeframe.M5,
    Timeframe.M15,
    Timeframe.H1,
    Timeframe.H6,
    Timeframe.D1,
];