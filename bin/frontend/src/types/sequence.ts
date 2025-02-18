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
    PRICE = 'PRICE',
    TRADES = 'TRADES',
    TRADES_BUY = 'TRADES_BUY',
    TRADES_SELL = 'TRADES_SELL',
    VOLUME = 'VOLUME'
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

export type ValueType = 'BOOLEAN' | 'COUNT' | 'PERCENT' | 'QUOTE' | 'STRING' | 'USD';
export type ValueNumberType = 'COUNT' | 'PERCENT' | 'QUOTE' | 'USD';

export type Value = ValueBoolean | ValueCount | ValuePercent | ValueQuote | ValueString | ValueUsd;
export type ValueNumber = ValueCount | ValuePercent | ValueQuote | ValueUsd;

export type ValueBoolean = {
    type: 'BOOLEAN';
    value: boolean;
}

export type ValueCount = {
    type: 'COUNT';
    value: number;
}

export type ValuePercent = {
    type: 'PERCENT';
    value: number;
}

export type ValueQuote = {
    type: 'QUOTE';
    value: number;
}

export type ValueString = {
    type: 'STRING';
    value: string;
}

export type ValueUsd = {
    type: 'USD';
    value: number;
}