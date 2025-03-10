import { Action } from './action';
import {ComposedPumpFunQuick, DEFAULT_COMPOSED_PUMP_FUN_QUICK} from './compose';
import { Condition } from './condition';

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


export const DEFAULT_CONDITION: ComposedPumpFunQuick = {...DEFAULT_COMPOSED_PUMP_FUN_QUICK};