import {ConditionType, Field, Operator, ValueDuration, ValuePercent} from "@types";

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