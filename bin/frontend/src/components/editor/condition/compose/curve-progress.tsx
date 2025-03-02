import React, {FC, useState} from "react";
import {
    Compare,
    CompareCurveProgressPercent,
    ComposeBondingCurve,
    ComposeType,
    ConditionType,
    Field,
    Operator,
    TimeUnit,
    ValueType
} from "@types";
import {CompareWidget} from "@components/editor/condition";


export type CurveProgressWidgetProps = {
    condition: ComposeBondingCurve;
    onChange: (condition: ComposeBondingCurve) => void;

}

export const CurveProgressWidget: FC<CurveProgressWidgetProps> = ({condition, onChange}) => {
    const [curveProgress, setCurveProgress] = useState<Compare>(condition.condition.conditions[0]);

    const propagateChange = (updatedCurveProgress: CompareCurveProgressPercent) => {
        onChange({
            id: condition.id,
            type: ConditionType.COMPOSE,
            ty: ComposeType.CURVE_PROGRESS,
            condition: {
                type: ConditionType.AND,
                conditions: [
                    {
                        id: updatedCurveProgress.id,
                        type: ConditionType.COMPARE,
                        field: Field.CURVE_PROGRESS,
                        operator: updatedCurveProgress.operator,
                        value: updatedCurveProgress.value
                    },
                    {
                        id: updatedCurveProgress.id,
                        type: ConditionType.COMPARE,
                        field: Field.CURVE_PROGRESS_AGE,
                        operator: Operator.LESS_THAN,
                        value: {
                            type: ValueType.DURATION,
                            value: 1,
                            unit: TimeUnit.MINUTE
                        }
                    }
                ],
            },
        });
    };

    return (
        <>
            <CompareWidget
                compare={curveProgress}
                onChange={(compare) => {
                    setCurveProgress(() => {
                        propagateChange(compare as CompareCurveProgressPercent);
                        return compare;
                    });

                }}
            />
        </>
    );
};