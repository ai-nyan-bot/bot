import {Decimal} from "decimal.js";
import {Timeframe} from "@app/types/index.ts";

export type FieldType =
    | 'Price'
    | 'Trades'
    | 'Volume'

export type Field = {
    type: FieldType;
    operator_values: Map<Operator, ValueType>;
    operator_timeframes: Map<Operator, Array<Timeframe>>
}

export type ConditionType = "Compare" | "And" | "Or"

export type Condition = {
    id: string;
    type: ConditionType;
    field?: FieldType;
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

export type ValueType = 'Boolean' | 'Money' | 'Percent' | 'String';


export type ValueBoolean = {
    type: 'Boolean';
    value: boolean;
}

export type ValueMoney = {
    type: 'Money';
    value: Decimal;
}

export type ValuePercent = {
    type: 'Percent';
    value?: Decimal;
}

export type ValueString = {
    type: 'String';
    value: string;
}