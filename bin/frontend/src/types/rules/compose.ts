import {ConditionType} from "@app/types/rules/condition.ts";
import {Condition, Field, Operator, Value, ValuePercent} from "@types";

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
            {
                id: string;
                type: ConditionType.COMPARE,
                field: Field.CURVE_PROGRESS,
                operator: Operator,
                value: Value
            }
        ]

    }
}

