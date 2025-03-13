import {ConditionType} from "./condition";
import {Field} from "./field";
import {Operator} from "./operator";
import {ValueBoolean, ValueCount, ValueDuration, ValuePercent, ValueType,} from "./value";
import {Timeframe} from "./time";
import {uuidv4} from "@utils";

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

export type CompareSimpleAgeBase = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.AGE_BASE,
    operator: Operator.MORE_THAN_EQUAL | Operator.LESS_THAN_EQUAL,
    value?: ValueDuration
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
    value?: ValueCount,
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

export type CompareVenuePumpfun = {
    id: string;
    type: ConditionType.COMPARE,
    field: Field.VENUE_PUMPFUN,
    operator: Operator.EQUAL | Operator.NOT_EQUAL,
    value?: ValueBoolean,
}

export const DEFAULT_COMPARE_VENUE_PUMPFUN: CompareVenuePumpfun = {
    id: uuidv4(),
    type: ConditionType.COMPARE,
    field: Field.VENUE_PUMPFUN,
    operator: Operator.EQUAL,
    value: {
        type: ValueType.BOOLEAN,
        value: true
    }
};