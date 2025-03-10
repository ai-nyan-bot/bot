import React, {FC, useEffect, useState} from "react";
import {Label} from "@components/ui/label.tsx";
import {SelectTimeframe, TimeframeText} from "@components/editor/condition/component";
import {
    ALL_TIMEFRAMES,
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
import {ValueNumberInput} from "@components/editor/value.tsx";


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
                    conditions: [
                        {...condition.condition.conditions[0], value: minValue, timeframe: minTimeframe},
                        {...condition.condition.conditions[1], value: maxValue, timeframe: maxTimeframe}
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
                    <p>⚠️ The rule will never match ⚠️</p>
                    <p>Minimum tx count is greater than the maximum tx count for the same timeframe.</p>
                </div>
            );
        }
    }

    const text = (value: number) => {
        switch (type) {
            case SwapType.Total:
                if (value === 1) {
                    return "1 tx"
                } else {
                    return `${value} txs`
                }
            case SwapType.Buy:
                if (value === 1) {
                    return "1 buy tx"
                } else {
                    return `${value} buy txs`
                }
            case SwapType.Sell:
                if (value === 1) {
                    return "1 sell tx"
                } else {
                    return `${value} sell txs`
                }
        }
    }

    const className = "mt-4 text-sm text-center text-gray-500"

    if (minTimeframe === maxTimeframe) {
        if (minValue && maxValue) {
            if(minValue.value === maxValue.value) {
                return (
                    <div className={className}>
                        <p>Exactly {text(minValue.value)} occurred in the last <TimeframeText value={minTimeframe}/>.</p>
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