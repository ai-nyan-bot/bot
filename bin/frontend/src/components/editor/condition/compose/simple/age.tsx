import React, {FC, useEffect, useState} from "react";
import {ALL_TIME_UNITS, compareDurations, ComposedSimpleAge, ValueDuration} from "@types";
import {ValueDurationInput} from "@components/editor/value";
import {DurationText} from "@components/editor/condition/component/timeunit";

export type SimpleAgeComposeProps = {
    condition: ComposedSimpleAge;
    onChange: (condition: ComposedSimpleAge) => void;
};

export const SimpleAgeCompose: FC<SimpleAgeComposeProps> = ({condition, onChange}) => {
    const min = condition.condition.conditions[0];
    const max = condition.condition.conditions[1];

    const [minValue, setMinValue] = useState<ValueDuration | undefined>(min.value);
    const [maxValue, setMaxValue] = useState<ValueDuration | undefined>(max.value);
    useEffect(() => {
        if (min.value !== minValue || max.value !== maxValue) {
            onChange({
                ...condition,
                condition: {
                    ...condition.condition,
                    conditions: [
                        {...min, value: minValue},
                        {...max, value: maxValue}
                    ]
                }
            });
        }
    }, [minValue, maxValue]);

    return (
        <div className={"flex flex-row"}>
            <div className={"flex flex-col"}>
                <div className={"flex flex-row space-x-4"}>
                    <span className={"flex items-center text-zinc-500"}>Min</span>
                    <ValueDurationInput
                        supported={ALL_TIME_UNITS}
                        value={minValue}
                        onChange={setMinValue}
                        placeholder={`min age`}
                    />
                </div>

                <div className={"pt-4 flex flex-row space-x-4"}>
                    <span className={"flex items-center text-zinc-500"}>Max</span>
                    <ValueDurationInput
                        supported={ALL_TIME_UNITS}
                        value={maxValue}
                        onChange={setMaxValue}
                        placeholder={`max age`}
                    />
                </div>

                <RenderText
                    minValue={minValue}
                    maxValue={maxValue}
                />
            </div>
        </div>
    )
}

export type RenderTextProps = {
    minValue?: ValueDuration;
    maxValue?: ValueDuration;
}

export const RenderText: FC<RenderTextProps> = ({minValue, maxValue}) => {
    if (!minValue && !maxValue) {
        return null;
    }
    const className = "mt-4 text-sm text-center text-gray-500"
    if (minValue && maxValue) {
        if (compareDurations(minValue, maxValue) > 0) {
            return (
                <div className="mt-4 text-sm text-center text-yellow-700 font-bold">
                    <p>⚠️ The rule will never execute ⚠️</p>
                    <p>Minimum age is greater than the maximum age</p>
                </div>
            );
        }

        if (compareDurations(minValue, maxValue) == 0) {
            return (
                <div className="mt-4 text-sm text-center text-yellow-700 font-bold">
                    <p>⚠️ The rule will most likely not execute ⚠️</p>
                    <p>The token is exactly <DurationText {...maxValue}/> old</p>
                </div>
            );
        }

        return (
            <p className={className}>
                Token is at least <DurationText {...minValue}/> old
                but not older than <DurationText {...maxValue}/>
            </p>
        );
    }

    if (minValue) {
        return (
            <p className={className}>
                The token must be at least <DurationText {...minValue} /> old
            </p>
        );
    }

    if (maxValue) {
        return (
            <p className={className}>
                The token must be no older than <DurationText {...maxValue} />
            </p>
        );
    }

    return null;
}