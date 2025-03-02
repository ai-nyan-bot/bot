import React, {FC, useState} from "react";
import {Compare, ComposeBondingCurve, ComposeType, ConditionType} from "@types";
import {CompareWidget} from "@components/editor/condition";


export type CurveProgressWidgetProps = {
    condition: ComposeBondingCurve;
    onChange: (condition: ComposeBondingCurve) => void;

}

export const CurveProgressWidget: FC<CurveProgressWidgetProps> = ({condition, onChange}) => {
    const [curveProgress, setCurveProgress] = useState<Compare>(condition.condition.conditions[0]);

    const propagateChange = (updatedCurveProgress: Compare) => {
        onChange({
            id: condition.id,
            type: ConditionType.COMPOSE,
            ty: ComposeType.CURVE_PROGRESS,
            condition: {
                type: ConditionType.AND,
                conditions: [{
                    id: updatedCurveProgress.id,
                    type: ConditionType.COMPARE,
                    field: updatedCurveProgress.field!!,
                    operator: updatedCurveProgress.operator!!,
                    value: updatedCurveProgress.value
                }],
            },
        });
    };

    return (
        <>
            <CompareWidget
                condition={curveProgress}
                onOperatorChange={(id, operator) => {
                    setCurveProgress(prevState => {
                        const updated = {...prevState, operator};
                        propagateChange(updated);
                        return updated;
                    });
                }}
                onTimeframeChange={(id, timeframe) => {
                    setCurveProgress(prevState => {
                        const updated = {...prevState, timeframe};
                        propagateChange(updated);
                        return updated;
                    });
                }}
                onValueChange={(id, value) => {
                    setCurveProgress(prevState => {
                        const updated = {...prevState, value};
                        propagateChange(updated);
                        return updated;
                    });
                }}
            />
        </>
    );
};