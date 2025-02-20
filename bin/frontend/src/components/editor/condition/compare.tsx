import {Condition, Field, Operator, Timeframe, Value, ValueNumber, ValueNumberType} from "@types";
import React, {FC} from "react";
import {SelectOperator} from "./operator.tsx";
import {ValueNumberInput} from "./value.tsx";
import {SelectTimeframe} from "./timeframe.tsx";
import {SelectField} from "./field.tsx";
import {config} from "@components/editor/config.ts";


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

    console.log("C", condition)
    const supportedOperators = Object.keys(config[condition.field!!]?.operators || {}) as Operator[];
    const supportedValueTypes = config[condition.field!!]?.operators[condition.operator!!]?.valueTypes || [];
    const supportedTimeframes = config[condition.field!!]?.operators[condition.operator!!]?.timeframes || [];

    return (
        <div key={condition.id}>
            <SelectField
                defaultField={condition.field}
                supported={Object.keys(config) as Field[]}
                onChange={(value) => onFieldChange(condition.id, value)}
            />

            <SelectOperator
                defaultOperator={condition.operator}
                supported={supportedOperators}
                onChange={(value) => onOperatorChange(condition.id, value)}
            />

            <ValueNumberInput
                supported={supportedValueTypes as ValueNumberType[]}
                defaultValue={(condition.value) ? condition.value as ValueNumber : undefined}
                onChange={(value) => onValueChange(condition.id, value)}
            />

            {supportedTimeframes.length > 0 && <SelectTimeframe
                defaultTimeframe={condition.timeframe}
                supported={supportedTimeframes}
                onChange={(value) => onTimeframeChange(condition.id, value)}
            />}
        </div>
    )
}
