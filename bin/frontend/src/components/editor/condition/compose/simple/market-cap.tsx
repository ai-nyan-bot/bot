import React, {FC, useEffect, useState} from "react";
import {ComposedSimpleMarketCap, ValueNumber, ValueType} from "@types";
import {ValueNumberInput} from "@components/editor/value";
import {NumberText} from "@components/editor/condition/component/number";

export type SimpleMarketCapComposeProps = {
    condition: ComposedSimpleMarketCap;
    onChange: (condition: ComposedSimpleMarketCap) => void;
};

export const SimpleMarketCapCompose: FC<SimpleMarketCapComposeProps> = ({condition, onChange}) => {
    const min = condition.condition.conditions[0];
    const max = condition.condition.conditions[1];

    const [minValue, setMinValue] = useState<ValueNumber | undefined>(min.value);
    const [maxValue, setMaxValue] = useState<ValueNumber | undefined>(max.value);

    useEffect(() => {
        if (min.value !== minValue || max.value !== maxValue) {
            onChange({
                ...condition,
                condition: {
                    ...condition.condition,
                    conditions: [
                        // @ts-ignore
                        {...min, value: minValue},
                        // @ts-ignore
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
                    <ValueNumberInput
                        value={minValue}
                        onChange={setMinValue}
                        supported={[ValueType.SOL, ValueType.USD]}
                        minValue={1}
                        placeholder={`min market cap`}
                    />
                </div>

                <div className={"pt-4 flex flex-row space-x-4"}>
                    <span className={"flex items-center text-zinc-500"}>Max</span>
                    <ValueNumberInput
                        value={maxValue}
                        onChange={setMaxValue}
                        supported={[ValueType.SOL, ValueType.USD]}
                        minValue={1}
                        placeholder={`max market cap`}
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
    minValue?: ValueNumber;
    maxValue?: ValueNumber;
}

export const RenderText: FC<RenderTextProps> = ({minValue, maxValue}) => {
    if (!minValue && !maxValue) {
        return null;
    }

    const className = "mt-4 text-sm text-center text-gray-500";

    if (minValue && maxValue) {
        if (minValue.type == maxValue.type && minValue.value > maxValue.value) {
            return (
                <div className="mt-4 text-sm text-center text-yellow-700 font-bold">
                    <p>⚠️ The rule will never execute ⚠️</p>
                    <p>Minimum market cap is greater than the maximum market cap</p>
                </div>
            );
        }
        return (
            <div className={className}>
                <p>Token market cap is at least <NumberText {...minValue} /></p>
                <p>but does not exceed <NumberText {...maxValue} />.</p>
            </div>
        );
    }

    if (minValue) {
        return (
            <div className={className}>
                <p>The token must have a market cap of at least <NumberText {...minValue} /></p>
            </div>
        );
    }

    if (maxValue) {
        return (
            <div className={className}>
                <p>The token must have a market cap no higher than <NumberText {...maxValue} /></p>
            </div>
        );
    }

    return null;
}