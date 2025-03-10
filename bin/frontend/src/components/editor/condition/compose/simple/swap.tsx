import React, {FC, useEffect, useState} from "react";
import {Label} from "@components/ui/label.tsx";
import {SelectTimeframe, ValueNumberInput} from "@components/editor/condition/component";
import {
    ALL_TIMEFRAMES,
    ComposedSimpleSwapBuy,
    ComposedSimpleSwapSell,
    ComposedSimpleSwapTotal,
    Timeframe,
    ValueCount,
    ValueNumber,
    ValueType
} from "@types";


export type SimpleSwapComposeProps = {
    condition: ComposedSimpleSwapTotal | ComposedSimpleSwapBuy | ComposedSimpleSwapSell;
    onChange: (condition: ComposedSimpleSwapTotal | ComposedSimpleSwapBuy | ComposedSimpleSwapSell) => void;
};

export const SimpleSwapCompose: FC<SimpleSwapComposeProps> = ({condition, onChange}) => {
    const min = condition.condition.conditions[0];
    const max = condition.condition.conditions[1];

    const [minValue, setMinValue] = useState<ValueCount | undefined>(min.value);
    const [minTimeframe, setMinTimeframe] = useState<Timeframe>(min.timeframe);
    const [maxValue, setMaxValue] = useState<ValueCount | undefined>(max.value);
    const [maxTimeframe, setMaxTimeframe] = useState<Timeframe>(max.timeframe);

    useEffect(() => {
        if (
            min.value !== minValue ||
            min.timeframe !== minTimeframe ||
            max.value !== maxValue ||
            max.timeframe !== maxTimeframe
        ) {
            onChange({
                ...condition,
                condition: {
                    ...condition.condition,
                    conditions: [
                        { ...condition.condition.conditions[0], value: minValue, timeframe: minTimeframe },
                        { ...condition.condition.conditions[1], value: maxValue, timeframe: maxTimeframe }
                    ]
                }
            });
        }
    }, [minValue, maxValue]);

    return (
        <div className={"flex flex-row"}>
            <div id="condition" className={"flex flex-col"}>
                <div className={"flex flex-row"}>
                    <Label htmlFor="swap-total-min">Min</Label>
                    <ValueNumberInput
                        id="swap-total-min"
                        value={minValue}
                        onChange={(value) => setMinValue(_ => {
                            if (!value || isNaN(value.value)) {
                                return undefined;
                            }
                            return value as ValueCount
                        })}
                        supported={[ValueType.COUNT]}
                        placeholder={"min total txs"}
                        hideValueSelect
                    />
                    <SelectTimeframe
                        defaultTimeframe={Timeframe.H1}
                        supported={ALL_TIMEFRAMES}
                        onChange={setMinTimeframe}
                    />
                </div>

                <div className={"flex flex-row space"}>
                    <Label htmlFor="swap-total-max">Max</Label>
                    <ValueNumberInput
                        id="swap-total-max"
                        value={maxValue}
                        onChange={(value) => setMaxValue(_ => {
                            if (!value || isNaN(value.value)) {
                                return undefined;
                            }
                            return value as ValueCount
                        })}
                        supported={[ValueType.COUNT]}
                        placeholder={"max total txs"}
                        hideValueSelect
                    />
                    <SelectTimeframe
                        defaultTimeframe={Timeframe.H1}
                        supported={ALL_TIMEFRAMES}
                        onChange={setMaxTimeframe}
                    />
                </div>

                <RenderText
                    minValue={minValue}
                    minTimeframe={minTimeframe}
                    maxValue={maxValue}
                    maxTimeframe={maxTimeframe}
                />
            </div>
        </div>
    )
}

type RenderTextProps = {
    minValue?: ValueNumber;
    minTimeframe: Timeframe;
    maxValue?: ValueNumber;
    maxTimeframe: Timeframe;
}

const RenderText: FC<RenderTextProps> = ({minValue, minTimeframe, maxValue, maxTimeframe}) => {
    // FIXME indicate error if minValue > maxValue for the same timeframe
    return (
        <div className="mt-4 text-sm text-center text-gray-500">
            {/*<p>At least 10 txn in the last 1 hour</p>*/}
            {/*<p>but not more than 20 in the last hour</p>*/}
            <p> {JSON.stringify(minValue?.value)}</p>
            <p> {JSON.stringify(minTimeframe)}</p>
            <p> {JSON.stringify(maxValue?.value)}</p>
            <p> {JSON.stringify(maxTimeframe)}</p>
        </div>
    )
}