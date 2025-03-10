import {
    Action,
    CompareCurveProgressAge,
    CompareCurveProgressPercent,
    CompareSimpleSwapTotal,
    ComposedCurveProgress,
    ComposedPumpFunQuick,
    ComposedSimpleSwapTotal,
    // ComposeType,
    Condition,
    // Field,
    // Operator,
    // Timeframe,
    // TimeUnit,
    // ValueType
} from "@app/types";

import {ConditionType} from "./condition";
import {ComposeType} from "./compose";
import {Field} from "./field";
import {Operator} from "./operator";
import {Timeframe, TimeUnit} from "./time";
import {ValueType} from "./value";

import {uuidv4} from "@utils";

export * from './action';
export * from './compare';
export * from './compose';
export * from './condition';
export * from './field';
export * from './operator';
export * from './time';
export * from './value';

export enum RuleStatus {
    ACTIVE = 'ACTIVE',
    INACTIVE = 'INACTIVE',
    ACTIVE_EXHAUSTED = 'ACTIVE_EXHAUSTED',
    INACTIVE_EXHAUSTED = 'INACTIVE_EXHAUSTED'
}


export type Sequence = {
    condition: Condition,
    action: Action
}


export const DEFAULT_CONDITION: ComposedPumpFunQuick = {
    id: uuidv4(),
    type: ConditionType.COMPOSE,
    composition: ComposeType.PUMP_FUN_QUICK,
    condition: {
        type: ConditionType.AND,
        conditions: [
            {
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
            } satisfies ComposedCurveProgress,
            {
                id: uuidv4(),
                type: ConditionType.COMPOSE,
                composition: ComposeType.SIMPLE_SWAP_TOTAL,
                condition: {
                    type: ConditionType.AND,
                    conditions: [
                        {
                            id: uuidv4(),
                            type: ConditionType.COMPARE,
                            field: Field.SWAP_TOTAL,
                            operator: Operator.MORE_THAN_EQUAL,
                            value: undefined,
                            timeframe: Timeframe.H1
                        } satisfies CompareSimpleSwapTotal,
                        {
                            id: uuidv4(),
                            type: ConditionType.COMPARE,
                            field: Field.SWAP_TOTAL,
                            operator: Operator.MORE_THAN_EQUAL,
                            value: undefined,
                            timeframe: Timeframe.H1
                        } satisfies CompareSimpleSwapTotal
                    ]
                }
            } satisfies ComposedSimpleSwapTotal
        ]
    }
};