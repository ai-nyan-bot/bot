// export type Operator =
//     | 'Equal'
//     | 'GreaterThan'
//     | 'GreaterThanEqual'
//     | 'LessThan'
//     | 'LessThanEqual'
//     | 'NotEqual'
//
//
// export type Sequence = {
//     condition: Condition;
//     action: Action;
// }
//
// export type ConditionType = 'Compare' | 'And' | 'Or'
//
// export type Condition = ConditionCompare | ConditionAnd | ConditionOr;
//
// export type BaseCondition = {
//     type: ConditionType;
// }
//
// export type ConditionCompare = BaseCondition & {
//     type: 'Compare';
//     operator?: Operator;
//     value?: Value;
//     timeframe?: Timeframe
// }
//
// export type ConditionAnd = BaseCondition & {
//     type: 'And';
//     left: Condition;
//     right: Condition;
// }
//
// export type ConditionOr = BaseCondition & {
//     type: 'Or';
//     left: Condition;
//     right: Condition;
// }
//
// export type Action = 'Notify'
//
// export type ValueType = 'Boolean' | 'Number' | 'String'
//
// export type Value = ValueBoolean | ValueNumber | ValueString;
//
// export type BaseValue = {
//     type: ValueType;
// }
//
// export type ValueBoolean = BaseValue & {
//     value: boolean;
// }
//
// export type ValueNumber = BaseValue & {
//     value: ValueBoolean
// }
//
// export type ValueString = BaseValue & {
//     value: string;
// }
//
// export enum Timeframe {
//     S1,
//     M1,
//     M5,
//     M15,
//     H1,
//     H4,
//     D1,
//     W1,
// }