import {ConditionType} from "@app/types/rules/condition";
import {Condition} from "@types";
import {CompareCurveProgressAge, CompareCurveProgressPercent} from "./compare";

export enum ComposeType {
    CURVE_PROGRESS = 'CURVE_PROGRESS',
    GROUP = 'GROUP',
    PUMP_FUN_QUICK = 'PUMP_FUN_QUICK',
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