import {
    CompareCurveProgressAge,
    CompareCurveProgressPercent,
    CompareSimpleAgeBase,
    CompareSimpleMarketCap,
    CompareSimpleSwapBuyAggregate,
    CompareSimpleSwapSell,
    CompareSimpleSwapTotalAggregate,
    CompareVenuePumpfun,
    DEFAULT_COMPARE_VENUE_PUMPFUN
} from "./compare";

import {Field} from "./field";
import {Operator} from "./operator";
import {ValueType} from "./value";
import {Condition, ConditionType} from "./condition";
import {Timeframe, TimeUnit} from "./time";

import {uuidv4} from "@utils";

export enum ComposeType {
    CURVE_PROGRESS = 'CURVE_PROGRESS',
    GROUP = 'GROUP',
    PUMP_FUN_QUICK = 'PUMP_FUN_QUICK',

    SIMPLE_AGE = 'SIMPLE_AGE',
    SIMPLE_MARKET_CAP = 'SIMPLE_MARKET_CAP',

    SIMPLE_SWAP_TOTAL_AGGREGATE = 'SIMPLE_SWAP_TOTAL_AGGREGATE',
    SIMPLE_SWAP_BUY_AGGREGATE = 'SIMPLE_SWAP_BUY_AGGREGATE',
    SIMPLE_SWAP_SELL_AGGREGATE = 'SIMPLE_SWAP_SELL_AGGREGATE',
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

export type ComposedSimpleAge = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_AGE,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleAgeBase,
            CompareSimpleAgeBase
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_AGE: ComposedSimpleAge = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_AGE,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.AGE_BASE,
                operator: Operator.MORE_THAN_EQUAL,
                value: {
                    type: ValueType.DURATION,
                    value: 1,
                    unit: TimeUnit.HOUR
                },
            },
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.AGE_BASE,
                operator: Operator.LESS_THAN_EQUAL,
                value: {
                    type: ValueType.DURATION,
                    value: 1,
                    unit: TimeUnit.DAY
                },
            }
        ]
    }
};

export type ComposedSimpleMarketCap = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_MARKET_CAP,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleMarketCap,
            CompareSimpleMarketCap
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_MARKET_CAP: ComposedSimpleMarketCap = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_MARKET_CAP,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.MARKET_CAP,
                operator: Operator.MORE_THAN_EQUAL,
                value: undefined,
            },
            {
                id: uuidv4(),
                type: ConditionType.COMPARE,
                field: Field.MARKET_CAP,
                operator: Operator.LESS_THAN_EQUAL,
                value: undefined
            }
        ]
    }
};

export type ComposedSimpleSwapTotalAggregate = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_TOTAL_AGGREGATE,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapTotalAggregate,
            CompareSimpleSwapTotalAggregate
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL_AGGREGATE: ComposedSimpleSwapTotalAggregate = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_SWAP_TOTAL_AGGREGATE,
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


export const isComposedSimpleSwapTotalAggregate = (condition: any): condition is ComposedSimpleSwapTotalAggregate => {
    return condition && condition.composition === ComposeType.SIMPLE_SWAP_TOTAL_AGGREGATE;
}


export type ComposedSimpleSwapBuyAggregate = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_BUY_AGGREGATE,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapBuyAggregate,
            CompareSimpleSwapBuyAggregate
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_SWAP_BUY_AGGREGATE: ComposedSimpleSwapBuyAggregate = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_SWAP_BUY_AGGREGATE,
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

export const isComposedSimpleSwapBuyAggregate = (condition: any): condition is ComposedSimpleSwapBuyAggregate => {
    return condition && condition.composition === ComposeType.SIMPLE_SWAP_BUY_AGGREGATE;
}


export type ComposedSimpleSwapSellAggregate = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_SELL_AGGREGATE,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapSell,
            CompareSimpleSwapSell
        ]
    }
}

export const DEFAULT_COMPOSED_SIMPLE_SWAP_SELL_AGGREGATE: ComposedSimpleSwapSellAggregate = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.SIMPLE_SWAP_SELL_AGGREGATE,
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

export const isComposedSimpleSwapSellAggregate = (condition: any): condition is ComposedSimpleSwapSellAggregate => {
    return condition && condition.composition === ComposeType.SIMPLE_SWAP_SELL_AGGREGATE;
}

export type ComposedPumpFunQuick = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.PUMP_FUN_QUICK,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareVenuePumpfun?,
            ComposedSimpleAge?,
            ComposedCurveProgress?,
            ComposedSimpleSwapTotalAggregate?,
            ComposedSimpleSwapBuyAggregate?,
            ComposedSimpleSwapSellAggregate?,
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
            {...DEFAULT_COMPARE_VENUE_PUMPFUN},
            {...DEFAULT_COMPOSED_SIMPLE_AGE},
            {...DEFAULT_COMPOSED_CURVE_PROGRESS},
            {...DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL_AGGREGATE},
            {...DEFAULT_COMPOSED_SIMPLE_SWAP_BUY_AGGREGATE},
            {...DEFAULT_COMPOSED_SIMPLE_SWAP_SELL_AGGREGATE},
        ]
    }
};