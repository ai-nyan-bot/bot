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

export type Value = ValueBoolean | ValueMoneyQuote | ValueMoneyUSD | ValueString;

export type ValueBoolean = {
    type: 'Boolean';
    value: boolean;
}

export type ValueMoneyQuote = {
    type: 'MoneyQuote';
    value: Decimal;
}

export type ValueMoneyUSD = {
    type: 'MoneyUSD';
    value: Decimal;
}

export type ValuePercent = {
    type: 'Percent';
    value: number;
}

export type ValueString = {
    type: 'String';
    value: string;
}