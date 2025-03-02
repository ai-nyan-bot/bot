import {Compose, ComposeBondingCurve, ComposeType, Condition} from "@types";
import React, {FC, useState} from "react";
import {SelectComposeType} from "@components/editor/condition/compose/type";
import {CurveProgressWidget} from "@components/editor/condition/compose/curve-progress.tsx";

type ComposeWidgetProps = {
    condition: Compose,
    onComposeTypeChange: (id: string, value: ComposeType) => void;
    // onOperatorChange: (id: string, value: Operator) => void;
    // onTimeframeChange: (id: string, value: Timeframe) => void;
    // onValueChange: (id: string, value: Value) => void;
    onConditionChange: (value: Condition) => void;
}

export const ComposeWidget: FC<ComposeWidgetProps> = ({
                                                          condition,
                                                          onComposeTypeChange,
                                                          onConditionChange
                                                          // onOperatorChange,
                                                          // onTimeframeChange,
                                                          // onValueChange
                                                      }) => {
    const [composeType, setComposeType] = useState<ComposeType>(condition.ty)

    // const supportedOperators = Object.keys(config[condition.field!!]?.operators || {}) as Operator[];
    // const supportedValueTypes = config[condition.field!!]?.operators[condition.operator!!]?.valueTypes || [];
    // const supportedTimeframes = config[condition.field!!]?.operators[condition.operator!!]?.timeframes || [];

    return (
        <div key={condition.id}>
            <SelectComposeType
                defaultType={ComposeType.CURVE_PROGRESS}
                supported={[ComposeType.CURVE_PROGRESS]}
                onChange={(value) => {
                    setComposeType(value)
                    onComposeTypeChange(condition.id, value)
                }}
            />

            {composeType === ComposeType.CURVE_PROGRESS && (
                <CurveProgressWidget
                    key={condition.id}
                    condition={condition as unknown as ComposeBondingCurve}
                    onChange={(condition) => {
                        onConditionChange(condition as unknown as Condition);
                    }}
                />

            )}
        </div>
    )
}
