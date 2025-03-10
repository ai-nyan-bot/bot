import React, {FC, useState} from "react";
import {ComposedCurveProgress, ComposeType, ConditionType, Field, Operator, TimeUnit, ValueType} from "@types";
import {DualRangeSlider} from "@components/ui/slider.tsx";

export type CurveProgressComposeProps = {
    condition: ComposedCurveProgress;
    onChange: (condition: ComposedCurveProgress) => void;

}

export const CurveProgressCompose: FC<CurveProgressComposeProps> = ({condition, onChange}) => {
    const curveProgress = condition.condition.conditions[0];

    const [range, setRange] = useState<number[]>(() => {
        let conditions = condition.condition.conditions;
        return [conditions[0].value.value, conditions[1].value.value]
    });

    const handleChange = (values: number[]) => {
        setRange(_ => {
            let operator = values[0] == 0.0 ? Operator.MORE_THAN_EQUAL : Operator.LESS_THAN_EQUAL;

            onChange({
                id: condition.id,
                type: ConditionType.COMPOSE,
                composition: ComposeType.CURVE_PROGRESS,
                condition: {
                    type: ConditionType.AND,
                    conditions: [
                        {
                            id: curveProgress.id,
                            type: ConditionType.COMPARE,
                            field: Field.CURVE_PROGRESS,
                            operator: Operator.MORE_THAN_EQUAL,
                            value: {
                                type: ValueType.PERCENT,
                                value: values[0]
                            }
                        },
                        {
                            id: curveProgress.id,
                            type: ConditionType.COMPARE,
                            field: Field.CURVE_PROGRESS,
                            operator,
                            value: {
                                type: ValueType.PERCENT,
                                value: values[1]
                            }
                        },
                        {
                            id: curveProgress.id,
                            type: ConditionType.COMPARE,
                            field: Field.CURVE_PROGRESS_AGE,
                            operator: Operator.LESS_THAN_EQUAL,
                            value: {
                                type: ValueType.DURATION,
                                value: 1,
                                unit: TimeUnit.MINUTE
                            }
                        }
                    ],
                },
            });

            return values
        });
    };

    return (
        <div className="w-full max-w-md mx-auto p-4 ">
            <DualRangeSlider
                value={range}
                onValueChange={handleChange}
                min={0}
                max={99}
                step={1}
                className="mt-2"
            />
            <div className="mt-4 text-sm text-center text-gray-500">
                {range[0] == 0 && (<p>Curve progressed more than {range[1]}%</p>)}
                {range[0] > 0 && (<>
                    <p>Curve progressed at least {range[0]}%</p>
                    <p>but not more than {range[1]}%</p>
                </>)}
            </div>
        </div>
    );
};