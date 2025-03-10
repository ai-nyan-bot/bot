import React, {FC, useEffect, useState} from "react";
import {Label} from "@components/ui/label.tsx";
import {SelectTimeframe, ValueNumberInput} from "@components/editor/condition/component";
import {
    ALL_TIMEFRAMES,
    ComposedSimpleSwapBuy,
    ComposedSimpleSwapSell,
    ComposedSimpleSwapTotal,
    Timeframe,
    ValueNumber,
    ValueType
} from "@types";


export type SimpleSwapComposeProps = {
    condition: ComposedSimpleSwapTotal | ComposedSimpleSwapBuy | ComposedSimpleSwapSell;
    onChange: (condition: ComposedSimpleSwapTotal | ComposedSimpleSwapBuy | ComposedSimpleSwapSell) => void;
};

export const SimpleSwapCompose: FC<SimpleSwapComposeProps> = ({condition}) => {
    // console.log(JSON.stringify(condition, null, 2));
    // min value
    // min time frame
    // max value
    // max time frame
    const min = condition.condition.conditions[0];
    const max = condition.condition.conditions[1];
    // console.log(JSON.stringify(min, null, 2));
    // console.log(JSON.stringify(max, null, 2));

    const [minValue, setMinValue] = useState<ValueNumber | null>(min.value);
    const [minTimeframe, setMinTimeframe] = useState<Timeframe>(min.timeframe);
    const [maxValue, setMaxValue] = useState<ValueNumber | null>(max.value);
    const [maxTimeframe, setMaxTimeframe] = useState<Timeframe>(max.timeframe);

    // console.log(maxValue)

    useEffect(() => {
        if (
            min.value !== minValue ||
            min.timeframe !== minTimeframe ||
            max.value !== maxValue ||
            max.timeframe !== maxTimeframe
        ) {
            console.log("changed");
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
                        onChange={(value) => {
                            setMinValue((_) => {
                                if (value.value === 0 || value.value == null) {
                                    return null;
                                }
                                return value;
                            });
                        }}
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
                        onChange={setMaxValue}
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
    minValue: ValueNumber | null;
    minTimeframe: Timeframe;
    maxValue: ValueNumber | null;
    maxTimeframe: Timeframe;
}

const RenderText: FC<RenderTextProps> = ({minValue, minTimeframe, maxValue, maxTimeframe}) => {
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