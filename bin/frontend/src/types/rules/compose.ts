import {ConditionType} from "@app/types/rules/condition";
import {CompareSimpleSwapsTotal, Condition} from "@types";
import {CompareCurveProgressAge, CompareCurveProgressPercent} from "./compare";

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

export type ComposedPumpFunQuick = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.PUMP_FUN_QUICK,
    condition: {
        type: ConditionType.AND,
        conditions: [
            ComposedCurveProgress
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
            CompareSimpleSwapsTotal,
            CompareSimpleSwapsTotal
        ]
    }
}


export type ComposedSimpleSwapBuy = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_BUY,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapsTotal,
            CompareSimpleSwapsTotal
        ]
    }
}


export type ComposedSimpleSwapSell = {
    id: string;
    type: ConditionType.COMPOSE;
    composition: ComposeType.SIMPLE_SWAP_SELL,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareSimpleSwapsTotal,
            CompareSimpleSwapsTotal
        ]
    }
}
