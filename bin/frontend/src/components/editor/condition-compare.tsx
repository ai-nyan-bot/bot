import {Condition, Field, Operator, Timeframe, Value} from "@types";
import React, {FC} from "react";
import {SelectOperator} from "@components/editor/operator.tsx";
import {ValuePercentInput} from "@components/editor/value.tsx";
import {SelectTimeframe} from "@components/editor/timeframe.tsx";
import {SelectField} from "@components/editor/field.tsx";

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
                supported={[
                    Field.PRICE,
                    Field.TRADES,
                    Field.VOLUME,
                ]}
                onChange={(value) => onFieldChange(condition.id, value)}
            />

            <SelectOperator
                supported={[
                    Operator.INCREASED_BY,
                    Operator.GREATER_THAN
                ]}
                onChange={(value) => onOperatorChange(condition.id, value)}
            />

            <ValuePercentInput
                onChange={(value) => onValueChange(condition.id, value)}
            />

            <SelectTimeframe
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
