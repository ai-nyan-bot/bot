import {Timeframe} from "@app/types/index.ts";

export type Sequence = {
    condition: Condition,
    action: Action
}

export type Action = ActionNotify;

export type ActionNotify = {
    type: 'NOTIFY'
}

export enum Field {
    TRADES = 'TRADES',
    TRADES_BUY = 'TRADES_BUY',
    TRADES_SELL = 'TRADES_SELL',
}

export type ConditionType = 'COMPARE' | 'AND' | 'OR'

export type Condition = {
    id: string;
    type: ConditionType;
    field?: Field;
    operator?: Operator;
    value?: Value;
    timeframe?: Timeframe;
    conditions?: Condition[];
};

export enum Operator {
    EQUAL = "EQUAL",
    NOT_EQUAL = "NOT_EQUAL",

    INCREASED_BY_MORE_THAN = 'INCREASED_BY_MORE_THAN',
    INCREASED_BY_MORE_THAN_EQUAL = 'INCREASED_BY_MORE_THAN_EQUAL',
    INCREASED_BY_LESS_THAN = 'INCREASED_BY_LESS_THAN',
    INCREASED_BY_LESS_THAN_EQUAL = 'INCREASED_BY_LESS_THAN_EQUAL',

    DECREASED_BY_MORE_THAN = 'DECREASED_BY_MORE_THAN',
    DECREASED_BY_MORE_THAN_EQUAL = 'DECREASED_BY_MORE_THAN_EQUAL',
    DECREASED_BY_LESS_THAN = 'DECREASED_BY_LESS_THAN',
    DECREASED_BY_LESS_THAN_EQUAL = 'DECREASED_BY_LESS_THAN_EQUAL',

    MORE_THAN = 'MORE_THAN',
    MORE_THAN_EQUAL = 'MORE_THAN_EQUAL',
    LESS_THAN = 'LESS_THAN',
    LESS_THAN_EQUAL = 'LESS_THAN_EQUAL',

}

export enum ValueType {
    BOOLEAN = 'BOOLEAN',
    COUNT = 'COUNT',
    PERCENT = 'PERCENT',
    QUOTE = 'QUOTE',
    STRING = 'STRING',
    USD = 'USD'
}

export const COUNT_AND_PERCENT: Array<ValueNumberType> = [ValueType.COUNT, ValueType.PERCENT];

export type ValueNumberType = ValueType.COUNT | ValueType.PERCENT | ValueType.QUOTE | ValueType.USD;

export type Value = ValueBoolean | ValueCount | ValuePercent | ValueQuote | ValueString | ValueUsd;
export type ValueNumber = ValueCount | ValuePercent | ValueQuote | ValueUsd;

export type ValueBoolean = {
    type: ValueType.BOOLEAN;
    value: boolean;
}

export type ValueCount = {
    type: ValueType.COUNT;
    value: number;
}

export type ValuePercent = {
    type: ValueType.PERCENT;
    value: number;
}

export type ValueQuote = {
    type: ValueType.QUOTE;
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