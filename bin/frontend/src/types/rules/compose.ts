import {ConditionType} from "@app/types/rules/condition";
import {Condition} from "@types";
import {CompareCurveProgressAge, CompareCurveProgressPercent} from "./compare";

export enum ComposeType {
    CURVE_PROGRESS = 'CURVE_PROGRESS',
}

export type Compose = {
    id: string;
    type: ConditionType.COMPOSE,
    ty: ComposeType,
    condition: Condition
}

export type ComposeBondingCurve = {
    id: string;
    type: ConditionType.COMPOSE;
    ty: ComposeType.CURVE_PROGRESS,
    condition: {
        type: ConditionType.AND,
        conditions: [
            CompareCurveProgressPercent,
            CompareCurveProgressAge
        ]
    }
}

