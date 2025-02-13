import {Condition, Operator, Timeframe, Value} from "@types";
import React, {FC} from "react";
import {SelectOperator} from "@components/editor/operator.tsx";
import {ValuePercentInput} from "@components/editor/value.tsx";
import {SelectTimeframe} from "@components/editor/timeframe.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";

type CompareProps = {
    condition: Condition,
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}

const fieldOptions = [
    {value: "TOKEN_PRICE", label: "Price"},
    {value: "TOKEN_TRADES", label: "Trades"},
]

export const Compare: FC<CompareProps> = ({condition, onOperatorChange, onTimeframeChange, onValueChange}) => {
    return (
        <div key={condition.id}>
            <Select>
                <SelectTrigger className="w-full">
                    <SelectValue placeholder={condition.field || "What?"}/>
                </SelectTrigger>
                <SelectContent>
                    {fieldOptions.map(({value, label}) => (
                        <SelectItem key={value} value={value}>
                            {label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>

            <SelectOperator
                supported={[
                    Operator.INCREASED_BY,
                    Operator.GREATER_THAN
                ]}
                onChange={(value) => onOperatorChange(condition.id, value)}
            />

            <ValuePercentInput
                onChange={(value) => onValueChange(condition.id, value)}
            />

            <SelectTimeframe
                supported={[
                    Timeframe.M1,
                    Timeframe.M5,
                    Timeframe.M15
                ]}
                onChange={(value) => onTimeframeChange(condition.id, value)}
            >
            </SelectTimeframe>
        </div>
    )
}
