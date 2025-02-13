import {Decimal} from "decimal.js";
import {Timeframe} from "@app/types/index.ts";

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
    INCREASED_BY = 'INCREASED_BY',
}

export type Value = ValueBoolean | ValueMoney | ValuePercent | ValueString;


export type ValueBoolean = {
    type: 'BOOLEAN';
    value: boolean;
}

export type ValueMoney = {
    type: 'MONEY';
    value: Decimal;
}

export type ValuePercent = {
    type: 'PERCENT';
    value: number;
}

export type ValueString = {
    type: 'STRING';
    value: string;
}