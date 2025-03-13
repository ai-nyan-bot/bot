import {TimeUnit} from "./index.ts";

export enum ValueType {
    BOOLEAN = 'BOOLEAN',
    COUNT = 'COUNT',
    DURATION = 'DURATION',
    PERCENT = 'PERCENT',
    QUOTE = 'QUOTE',
    SOL = 'SOL',
    STRING = 'STRING',
    USD = 'USD'
}

export const COUNT_AND_PERCENT: Array<ValueNumberType> = [ValueType.COUNT, ValueType.PERCENT];

export type ValueNumberType = ValueType.COUNT | ValueType.PERCENT | ValueType.SOL | ValueType.QUOTE | ValueType.USD;

export type Value =
    ValueBoolean
    | ValueCount
    | ValueDuration
    | ValuePercent
    | ValueQuote
    | ValueSol
    | ValueString
    | ValueUsd;

export type ValueNumber = ValueCount | ValuePercent | ValueQuote | ValueSol | ValueUsd;

export type ValueBoolean = {
    type: ValueType.BOOLEAN;
    value: boolean;
}

export type ValueCount = {
    type: ValueType.COUNT;
    value: number;
}

export type ValueDuration = {
    type: ValueType.DURATION;
    value: number;
    unit: TimeUnit;
}

const durationToSeconds = (duration: ValueDuration): number => {
    const conversionFactors: Record<TimeUnit, number> = {
        [TimeUnit.SECOND]: 1,
        [TimeUnit.MINUTE]: 60,
        [TimeUnit.HOUR]: 3600,
        [TimeUnit.DAY]: 86400,
    };

    return duration.value * conversionFactors[duration.unit];
}

export const compareDurations = (a: ValueDuration, b: ValueDuration): number => {
    const aSeconds = durationToSeconds(a);
    const bSeconds = durationToSeconds(b);
    return aSeconds - bSeconds;
}


export type ValuePercent = {
    type: ValueType.PERCENT;
    value: number;
}

export type ValueQuote = {
    type: ValueType.QUOTE;
    value: number;
}

export type ValueSol = {
    type: ValueType.SOL;
    value: number;
}

export type ValueString = {
    type: ValueType.STRING;
    value: string;
}

export type ValueUsd = {
    type: ValueType.USD;
    value: number;
}