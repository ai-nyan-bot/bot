import {Field, Operator, Timeframe, Value} from "@types";
import {Compose} from "@app/types/rules/compose.ts";

export enum ConditionType {
    COMPARE = 'COMPARE',
    COMPOSE = 'COMPOSE',
    AND = 'AND',
    OR = 'OR'
}

// export type Condition = {
//     id: string;
//     type: ConditionType;
//     field?: Field;
//     operator?: Operator;
//     value?: Value;
//     timeframe?: Timeframe;
//     conditions?: Condition[];
//
//     // COMPOSE
//     ty?: ComposeType;
//     condition?: Condition;
// };

export type Condition = And | Compare | Compose;

export type And = {
    id: string;
    type: ConditionType.AND,
    conditions: Array<Condition>
}

export type Compare = {
    id: string;
    type: ConditionType.COMPARE;
    field?: Field;
    operator?: Operator;
    value?: Value;
    timeframe?: Timeframe;
}