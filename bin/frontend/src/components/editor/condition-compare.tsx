import {Condition, Field, Operator, Timeframe, Value, ValueNumber} from "@types";
import React, {FC} from "react";
import {SelectOperator} from "@components/editor/operator.tsx";
import {ValueNumberInput} from "@components/editor/value.tsx";
import {SelectTimeframe} from "@components/editor/timeframe.tsx";
import {SelectField} from "@components/editor/field.tsx";


// export type Comparison = {
//     field: FieldType;
//     operator_values: Map<Operator, ValueType>;
//     operator_timeframes: Map<Operator, Array<Timeframe>>
// }

type CompareProps = {
    condition: Condition,
    onFieldChange: (id: string, value: Field) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}

export const Compare: FC<CompareProps> = ({
                                              condition,
                                              onFieldChange,
                                              onOperatorChange,
                                              onTimeframeChange,
                                              onValueChange
                                          }) => {
    return (
        <div key={condition.id}>
            <SelectField
                defaultField={condition.field}
                supported={[
                    Field.PRICE,
                    Field.TRADES,
                    Field.VOLUME,
                ]}
                onChange={(value) => onFieldChange(condition.id, value)}
            />

            <SelectOperator
                defaultOperator={condition.operator}
                supported={[
                    Operator.INCREASED_BY_MORE_THAN,
                    Operator.INCREASED_BY_MORE_THAN_EQUAL,
                    Operator.INCREASED_BY_LESS_THAN,
                    Operator.INCREASED_BY_LESS_THAN_EQUAL,
                    Operator.DECREASED_BY_MORE_THAN,
                    Operator.DECREASED_BY_MORE_THAN_EQUAL,
                    Operator.DECREASED_BY_LESS_THAN,
                    Operator.DECREASED_BY_LESS_THAN_EQUAL,
                    Operator.MORE_THAN,
                    Operator.MORE_THAN_EQUAL,
                    Operator.LESS_THAN,
                    Operator.LESS_THAN_EQUAL,
                ]}
                onChange={(value) => onOperatorChange(condition.id, value)}
            />

            <ValueNumberInput
                supportedTypes={['COUNT', 'PERCENT']}
                defaultValue={condition.value ? condition.value as ValueNumber : {type: 'COUNT', value: 0}}
                onChange={(value) => onValueChange(condition.id, value)}
            />

            <SelectTimeframe
                defaultTimeframe={condition.timeframe}
                supported={[
                    Timeframe.M1,
                    Timeframe.M5,
                    Timeframe.M15
                ]}
                onChange={(value) => onTimeframeChange(condition.id, value)}
            >
            </SelectTimeframe>
        </div>
    )
}
