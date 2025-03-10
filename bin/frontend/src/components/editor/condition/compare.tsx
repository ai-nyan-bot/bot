import {Compare, Operator, ValueNumber, ValueNumberType} from "@types";
import React, {FC, useState} from "react";
import {config} from "@components/editor/config.ts";
import {SelectOperator, SelectTimeframe, ValueNumberInput,} from "./component";


type CompareWidgetProps = {
    compare: Compare,
    onChange: (compare: Compare) => void;
}

export const CompareWidget: FC<CompareWidgetProps> = ({
                                                          compare,
                                                          onChange
                                                      }) => {
    const [value, setValue] = useState<Compare>(compare);

    const supportedOperators = Object.keys(config[compare.field!!]?.operators || {}) as Operator[];
    const supportedValueTypes = config[compare.field!!]?.operators[compare.operator!!]?.valueTypes || [];
    const supportedTimeframes = config[compare.field!!]?.operators[compare.operator!!]?.timeframes || [];

    return (
        <div key={value.id}>
            {/*<SelectField*/}
            {/*    defaultField={condition.field}*/}
            {/*    supported={Object.keys(config) as Field[]}*/}
            {/*    onChange={(value) => onFieldChange(condition.id, value)}*/}
            {/*/>*/}

            <SelectOperator
                defaultOperator={value.operator}
                supported={supportedOperators}
                onChange={(operator) => {
                    setValue(prev => {
                        let updated = {...prev, operator}
                        onChange(updated);
                        return updated;
                    });
                }}
            />

            <ValueNumberInput
                supported={supportedValueTypes as ValueNumberType[]}
                defaultValue={(value.value) ? value.value as ValueNumber : undefined}
                onChange={(value) => {
                    setValue(prev => {
                        let updated = {...prev, value}
                        onChange(updated);
                        return updated;
                    });
                }}
            />

            {supportedTimeframes.length > 0 && <SelectTimeframe
                defaultTimeframe={value.timeframe}
                supported={supportedTimeframes}
                onChange={(timeframe) => {
                    setValue(prev => {
                        let updated = {...prev, timeframe}
                        onChange(updated);
                        return updated;
                    });
                }}
            />}
        </div>
    )
}
