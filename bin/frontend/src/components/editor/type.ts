import {ALL_TIMEFRAMES, Field, Operator, Timeframe, ValueType} from "@types";

export type UiField = {
    id: Field;
    operators: Array<UiOperator>;
}

export type UiOperator = {
    id: string;
    operator: Operator;
    values: Array<ValueType>;
    timeframes?: Array<Timeframe>;
};

// trades increased by 23% in 15 minutes
// trades increased by 23 in 15 minutes
// buy trades more than 100 in 15 minutes

export const UI_FIELDS: Array<UiField> = [
    {
        id: Field.TRADES,
        operators: [
            {
                id: "INCREASED_BY",
                operator: Operator.INCREASED_BY,
                values: ['PERCENT', 'COUNT'],
                timeframes: ALL_TIMEFRAMES
            },
            {
                id: "INCREASED_BY_EQUAL",
                operator: Operator.INCREASED_BY_EQUAL,
                values: ['PERCENT', 'COUNT'],
                timeframes: ALL_TIMEFRAMES
            },
            {
                id: "MORE_THAN",
                operator: Operator.MORE_THAN,
                values: ['COUNT'],
                timeframes: ALL_TIMEFRAMES
            },
            {
                id: "MORE_THAN_EQUAL",
                operator: Operator.MORE_THAN_EQUAL,
                values: ['COUNT'],
                timeframes: ALL_TIMEFRAMES
            }
        ]
    }
]