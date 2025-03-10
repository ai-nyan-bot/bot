import {Compose, ComposedCurveProgress, ComposeType, Condition} from "@types";
import React, {FC, useState} from "react";
import {SelectComposeType} from "@components/editor/condition/compose/type";
import {CurveProgressWidget} from "@components/editor/condition/compose/curve-progress.tsx";

type ComposeWidgetProps = {
    condition: Compose;
    onRemove: (id: string) => void;
    onConditionChange: (value: Condition) => void;
}

export const ComposeWidget: FC<ComposeWidgetProps> = ({
                                                          condition,
                                                          onRemove,
                                                          onConditionChange
                                                      }) => {
    const [composeType, setComposeType] = useState<ComposeType>(condition.composition)

    // const supportedOperators = Object.keys(config[condition.field!!]?.operators || {}) as Operator[];
    // const supportedValueTypes = config[condition.field!!]?.operators[condition.operator!!]?.valueTypes || [];
    // const supportedTimeframes = config[condition.field!!]?.operators[condition.operator!!]?.timeframes || [];

    return (
        <div key={condition.id}>

            <div className={"flex flex-row justify-between"}>
                <SelectComposeType
                    defaultType={ComposeType.CURVE_PROGRESS}
                    supported={[ComposeType.CURVE_PROGRESS]}
                    onChange={(value) => {
                        setComposeType(value)
                        // TODO update condition
                    }}
                />
                <button
                    onClick={() => onRemove(condition.id)}
                    className="text-gray-500 hover:text-red-500"
                >
                    ✖
                </button>
            </div>

            {composeType === ComposeType.CURVE_PROGRESS && (
                <CurveProgressWidget
                    key={condition.id}
                    condition={condition as unknown as ComposedCurveProgress}
                    onChange={(condition) => {
                        onConditionChange(condition as unknown as Condition);
                    }}
                />

            )}

            {/*{composeType === ComposeType.PUMP_FUN_QUICK && (*/}
            {/*    <PumpfunComposeQuick*/}
            {/*        key={condition.id}*/}
            {/*        condition={condition as unknown as ComposeBondingCurve}*/}
            {/*        onChange={(condition) => {*/}
            {/*            onConditionChange(condition as unknown as Condition);*/}
            {/*        }}*/}
            {/*    />*/}

            {/*)}*/}
        </div>
    )
}
