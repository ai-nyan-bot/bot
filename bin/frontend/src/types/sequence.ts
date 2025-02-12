import {Decimal} from "decimal.js";

export type ConditionType = "Compare" | "And" | "Or"

export type Condition = {
    type: ConditionType;
    field?: string;
    operator?: string;
    value?: Value;
    timeframe?: string;
    conditions?: Condition[];
};

export enum Operator {
    EQUAL = "EQUAL",
    GREATER_THAN = 'GREATER_THAN',
    INCREASED_BY = 'INCREASED_BY',
}

export type Value = ValueBoolean | ValueMoney | ValueString;

export type ValueTyp = 'Boolean' | 'Money' | 'Percent' | 'String';


export type ValueBoolean = {
    type: 'Boolean';
    value: boolean;
}

export type ValueMoney = {
    type: 'MoneyQuote';
    value: Decimal;
}

export type ValuePercent = {
    type: 'Percent';
    value: Decimal;
}

export type ValueString = {
    type: 'String';
    value: string;
}