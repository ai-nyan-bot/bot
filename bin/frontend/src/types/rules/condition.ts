import {Field} from "./field";
import {Operator} from "./operator";
import {Value} from "./value";
import {Timeframe} from "./time";
import {Compose} from "./compose";

export enum ConditionType {
    AND = 'AND',
    COMPARE = 'COMPARE',
    COMPOSE = 'COMPOSE'
}

export type Condition = And | Compare | Compose;

export type And = {
    id: string;
    type: ConditionType.AND,
    conditions: Array<Condition>
}

export type Compare = {
    id: string;
    type: ConditionType.COMPARE;
    field: Field;
    operator: Operator;
    value: Value;
    timeframe?: Timeframe;
}
