import {ConditionType, Field, Operator, Timeframe, ValueCount, ValueDuration, ValuePercent} from "@types";

export type CompareCurveProgressPercent = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.CURVE_PROGRESS,
    operator: Operator.MORE_THAN_EQUAL | Operator.LESS_THAN_EQUAL,
    value: ValuePercent
}

export type CompareCurveProgressAge = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.CURVE_PROGRESS_AGE,
    operator: Operator.MORE_THAN_EQUAL | Operator.LESS_THAN_EQUAL,
    value: ValueDuration
}

export type CompareSimpleSwapTotal = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.SWAP_ALL,
    operator: Operator.MORE_THAN_EQUAL | Operator.LESS_THAN_EQUAL,
    value?: ValueCount,
    timeframe: Timeframe
}


export type CompareSimpleSwapBuy = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.SWAP_BUY,
    operator: Operator.MORE_THAN_EQUAL | Operator.LESS_THAN_EQUAL,
    value?: ValueCount ,
    timeframe: Timeframe
}

export type CompareSimpleSwapSell = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.SWAP_SELL,
    operator: Operator.MORE_THAN_EQUAL | Operator.LESS_THAN_EQUAL,
    value?: ValueCount,
    timeframe: Timeframe
}