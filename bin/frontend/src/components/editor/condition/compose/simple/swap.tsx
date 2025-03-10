import React, {FC} from "react";
import {Label} from "@components/ui/label.tsx";
import {SelectTimeframe, ValueNumberInput} from "@components/editor/condition/component";
import {ALL_TIMEFRAMES, Timeframe, ValueType} from "@types";

export enum SimpleSwapType {
    TOTAL,
    BUY,
    SELL
}

export type SimpleSwapComposeProps = {
    type: SimpleSwapType;
};

export const SimpleSwapCompose: FC<SimpleSwapComposeProps> = () => {
    
    return (
        <div className={"flex flex-row"}>
            <div id="condition" className={"flex flex-col"}>
                <div className={"flex flex-row"}>
                    <Label htmlFor="trading-volume-min">Min</Label>
                    <ValueNumberInput
                        id="trading-volume-min"
                        supported={[ValueType.COUNT]}
                        hideValueSelect
                    />
                    <SelectTimeframe
                        defaultTimeframe={Timeframe.H1}
                        supported={ALL_TIMEFRAMES}
                    />
                </div>

                <div className={"flex flex-row space"}>
                    <Label htmlFor="trading-volume-max">Max</Label>
                    <ValueNumberInput
                        id="trading-volume-max"
                        supported={[ValueType.COUNT]}
                        hideValueSelect
                    />
                    <SelectTimeframe
                        defaultTimeframe={Timeframe.H1}
                        supported={ALL_TIMEFRAMES}
                    />
                </div>

                <RenderText/>
            </div>
        </div>
    )
}

type RenderTextProps = {}

const RenderText: FC<RenderTextProps> = ({}) => {
    return (
        <div className="mt-4 text-sm text-center text-gray-500">
            <p>At least 10 txn in the last 1 hour</p>
            <p>but not more than 20 in the last hour</p>
        </div>
    )
}