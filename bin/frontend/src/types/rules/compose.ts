import {Condition, ConditionType} from "./condition";
import {
    CompareCurveProgressAge,
    CompareCurveProgressPercent,
    CompareSimpleSwapBuy,
    CompareSimpleSwapSell,
    CompareSimpleSwapTotal
} from "./compare";
import {Field} from "./field";
import {Operator} from "./operator";
import {ValueType} from "./value";
import {Timeframe, TimeUnit} from "./time";

import {uuidv4} from "@utils";

export enum ComposeType {
    CURVE_PROGRESS = 'CURVE_PROGRESS',
    GROUP = 'GROUP',
    PUMP_FUN_QUICK = 'PUMP_FUN_QUICK',

    SIMPLE_SWAP_TOTAL = 'SIMPLE_SWAP_TOTAL',
    SIMPLE_SWAP_BUY = 'SIMPLE_SWAP_BUY',
    SIMPLE_SWAP_SELL = 'SIMPLE_SWAP_SELL',
}

export type Compose = {
    id: string;
    type: ConditionType.COMPOSE,
    composition: ComposeType,
    condition: Condition
}

export type ComposedCurveProgress = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.CURVE_PROGRESS,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareCurveProgressPercent,
            CompareCurveProgressPercent,
            CompareCurveProgressAge
        ]
    }
}

export const DEFAULT_COMPOSED_CURVE_PROGRESS: ComposedCurveProgress = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.CURVE_PROGRESS,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.CURVE_PROGRESS,
                operator: Operator.MORE_THAN_EQUAL,
                value: {
                    type: ValueType.PERCENT,
                    value: 0
                }
            } satisfies CompareCurveProgressPercent,
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.CURVE_PROGRESS,
                operator: Operator.MORE_THAN_EQUAL,
                value: {
                    type: ValueType.PERCENT,
                    value: 95
                }
            } satisfies CompareCurveProgressPercent,
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.CURVE_PROGRESS_AGE,
                operator: Operator.LESS_THAN_EQUAL,
                value: {
                    type: ValueType.DURATION,
                    value: 1,
                    unit: TimeUnit.MINUTE
                }
            } satisfies CompareCurveProgressAge
        ]
    }
}

export type ComposedSimpleSwapTotal = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_TOTAL,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapTotal,
            CompareSimpleSwapTotal
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL: ComposedSimpleSwapTotal = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_SWAP_TOTAL,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.SWAP_ALL,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
                timeframe: Timeframe.H1
            },
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.SWAP_ALL,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
                timeframe: Timeframe.H1
            }
        ]
    }
};


export const isComposedSimpleSwapTotal = (condition: any): condition is ComposedSimpleSwapTotal => {
    return condition && condition.composition === ComposeType.SIMPLE_SWAP_TOTAL;
}


export type ComposedSimpleSwapBuy = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_BUY,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapBuy,
            CompareSimpleSwapBuy
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_SWAP_BUY: ComposedSimpleSwapBuy = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_SWAP_BUY,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.SWAP_BUY,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
                timeframe: Timeframe.H1
            },
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.SWAP_BUY,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
                timeframe: Timeframe.H1
            }
        ]
    }
};

export const isComposedSimpleSwapBuy = (condition: any): condition is ComposedSimpleSwapBuy => {
    return condition && condition.composition === ComposeType.SIMPLE_SWAP_BUY;
}


export type ComposedSimpleSwapSell = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_SELL,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapSell,
            CompareSimpleSwapSell
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_SWAP_SELL: ComposedSimpleSwapSell = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_SWAP_SELL,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.SWAP_SELL,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
                timeframe: Timeframe.H1
            },
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.SWAP_SELL,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
                timeframe: Timeframe.H1
            }
        ]
    }
};

export const isComposedSimpleSwapSell = (condition: any): condition is ComposedSimpleSwapSell => {
    return condition && condition.composition === ComposeType.SIMPLE_SWAP_SELL;
}

export type ComposedPumpFunQuick = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.PUMP_FUN_QUICK,
    condition: {
        type: ConditionType.AND,
        conditions: [
            ComposedCurveProgress?,
            ComposedSimpleSwapTotal?,
            ComposedSimpleSwapBuy?,
            ComposedSimpleSwapSell?,
        ]
    }
}

export const DEFAULT_COMPOSED_PUMP_FUN_QUICK: ComposedPumpFunQuick = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.PUMP_FUN_QUICK,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {...DEFAULT_COMPOSED_CURVE_PROGRESS},
            {...DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL},
            {...DEFAULT_COMPOSED_SIMPLE_SWAP_BUY},
            {...DEFAULT_COMPOSED_SIMPLE_SWAP_SELL},
        ]
    }
};