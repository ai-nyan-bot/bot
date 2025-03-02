export * from './app.ts'
export * from './rules'
export * from './sequence.ts'
export * from './telegram.ts'



export enum Timeframe {
    M1 = "M1",
    M5 = "M5",
    M15 = "M15"
}

export const ALL_TIMEFRAMES = [Timeframe.M1, Timeframe.M5, Timeframe.M15];