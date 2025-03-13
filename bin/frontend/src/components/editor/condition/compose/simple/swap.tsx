import React, {FC, useEffect, useState} from "react";
import {SelectTimeframe, TimeframeText} from "@components/editor/condition/component";
import {
    ALL_TIME_FRAMES,
    ComposedSimpleSwapBuy,
    ComposedSimpleSwapSell,
    ComposedSimpleSwapTotal,
    isComposedSimpleSwapBuy,
    isComposedSimpleSwapSell,
    isComposedSimpleSwapTotal,
    Timeframe,
    ValueCount,
    ValueType
} from "@types";
import {ValueNumberInput} from "@components/editor/value";


export type SimpleSwapComposeProps = {
    condition: ComposedSimpleSwapTotal | ComposedSimpleSwapBuy | ComposedSimpleSwapSell;
    onChange: (condition: ComposedSimpleSwapTotal | ComposedSimpleSwapBuy | ComposedSimpleSwapSell) => void;
};

export const SimpleSwapCompose: FC<SimpleSwapComposeProps> = ({condition, onChange}) => {
    const min = condition.condition.conditions[0];
    const max = condition.condition.conditions[1];

    const type: SwapType =
        isComposedSimpleSwapTotal(condition) ? SwapType.Total :
            isComposedSimpleSwapBuy(condition) ? SwapType.Buy :
                isComposedSimpleSwapSell(condition) ? SwapType.Sell :
                    (() => {
                        throw new Error("Unsupported condition");
                    })();

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
                    // @ts-ignore
                    conditions: [
                        {...min, value: minValue, timeframe: minTimeframe},
                        {...max, value: maxValue, timeframe: maxTimeframe}
                    ]
                }
            });
        }
    }, [minValue, minTimeframe, maxValue, maxTimeframe]);

    let placeholderText = null;
    switch (type) {
        case SwapType.Total:
            placeholderText = "total txn"
            break;
        case SwapType.Buy:
            placeholderText = "buy txn"
            break;
        case SwapType.Sell:
            placeholderText = "sell txn"
            break;
    }

    return (
        <div className={"flex flex-row"}>
            <div className={"flex flex-col"}>
                <div className={"flex flex-row space-x-4"}>
                    <span className={"flex items-center text-zinc-500"}>Min</span>
                    <ValueNumberInput
                        value={minValue}
                        onChange={(value) => setMinValue(_ => {
                            if (!value || isNaN(value.value)) {
                                return undefined;
                            }
                            return value as ValueCount
                        })}
                        supported={[ValueType.COUNT]}
                        minValue={1}
                        placeholder={`min ${placeholderText}`}
                        hideValueSelect
                    />
                    <SelectTimeframe
                        value={minTimeframe}
                        defaultValue={Timeframe.H1}
                        supported={ALL_TIME_FRAMES}
                        onChange={setMinTimeframe}
                    />
                </div>

                <div className={"pt-4 flex flex-row space-x-4"}>
                    <span className={"flex items-center text-zinc-500"}>Max</span>
                    <ValueNumberInput
                        value={maxValue}
                        onChange={(value) => setMaxValue(_ => {
                            if (!value || isNaN(value.value)) {
                                return undefined;
                            }
                            return value as ValueCount
                        })}
                        supported={[ValueType.COUNT]}
                        minValue={1}
                        placeholder={`max ${placeholderText}`}
                        hideValueSelect
                    />
                    <SelectTimeframe
                        value={maxTimeframe}
                        defaultValue={Timeframe.H1}
                        supported={ALL_TIME_FRAMES}
                        onChange={setMaxTimeframe}
                    />
                </div>

                <RenderText
                    minValue={minValue}
                    minTimeframe={minTimeframe}
                    maxValue={maxValue}
                    maxTimeframe={maxTimeframe}
                    type={type}
                />
            </div>
        </div>
    )
}

export enum SwapType {
    Total,
    Buy,
    Sell
}
export type RenderTextProps = {
    minValue?: ValueCount;
    minTimeframe: Timeframe;
    maxValue?: ValueCount;
    maxTimeframe: Timeframe;
    type: SwapType;
}

export const RenderText: FC<RenderTextProps> = ({minValue, minTimeframe, maxValue, maxTimeframe, type}) => {
    if (!minValue && !maxValue) {
        return null;
    }

    if (minValue && maxValue) {
        if (minTimeframe === maxTimeframe && minValue.value > maxValue.value) {
            return (
                <div className="mt-4 text-sm text-center text-yellow-700 font-bold">
                    <p>⚠️ The rule will never execute ⚠️</p>
                    <p>Minimum txn count is greater than the maximum txn count for the same timeframe.</p>
                </div>
            );
        }
    }


    const text = (value: number) => {
        switch (type) {
            case SwapType.Total:
                return `${value} txn`
            case SwapType.Buy:
                return `${value} buy txn`
            case SwapType.Sell:
                return `${value} sell txn`
        }
    }

    const className = "mt-4 text-sm text-center text-gray-500"

    if (minTimeframe === maxTimeframe) {
        if (minValue && maxValue) {
            if (minValue.value === maxValue.value) {
                return (
                    <div className={className}>
                        <p>Exactly {text(minValue.value)} occurred in the last <TimeframeText value={minTimeframe}/>.
                        </p>
                    </div>
                );
            }

            return (
                <div className={className}>
                    <p>At least {text(minValue.value)} occurred in the last <TimeframeText value={minTimeframe}/>.</p>
                    <p>However, the count should not exceed {maxValue.value} in the same timeframe.</p>
                </div>
            );
        }
    }

    if (minValue && maxValue) {
        return (
            <div className={className}>
                <p>At least {text(minValue.value)} occurred in the last <TimeframeText value={minTimeframe}/>.</p>
                <p>However, no more than {text(maxValue.value)} should occur in the last <TimeframeText
                    value={maxTimeframe}/>.</p>
            </div>
        );
    } else if (minValue) {
        return (
            <div className={className}>
                <p>At least {text(minValue.value)} occurred in the last <TimeframeText value={minTimeframe}/>.</p>
            </div>
        );
    }

    return (
        <div className={className}>
            <p>No more than {text(maxValue!!.value)} should occur in the last <TimeframeText value={maxTimeframe}/>.</p>
        </div>
    );
};