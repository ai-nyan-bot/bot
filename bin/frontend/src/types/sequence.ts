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
    VOLUME = 'VOLUME'
}

// export type Field = {
//     type: FieldType;
//     operator_values: Map<Operator, ValueType>;
//     operator_timeframes: Map<Operator, Array<Timeframe>>
// }

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
    GREATER_THAN = 'GREATER_THAN',
}

export type Value = ValueBoolean | ValueCount | ValuePercent | ValueQuote | ValueString | ValueUsd;

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