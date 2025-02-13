import React, {FC, useState} from "react";
import {Decimal} from "decimal.js";
import {ValuePercent} from "@types";

export type ValuePercentInputProps = {
    defaultValue?: Decimal;
    value?: Decimal;
    onChange?: (value: ValuePercent | undefined) => void;
}

export const ValuePercentInput: FC<ValuePercentInputProps> = ({value, defaultValue, onChange}) => {
    const [stateValue, setStateValue] = useState(value || defaultValue);
    return (
        <input
            type="number"
            value={stateValue?.toString()}
            onChange={(e) => {
                if (onChange) {
                    const input = e.target.value;
                    if (input.trim() === "") {
                        setStateValue(undefined);
                        onChange(
                            {
                                type: "Percent",
                                value: new Decimal(0)
                            }
                        )
                        return;
                    }
                    const newValue = new Decimal(e.target.value);
                    setStateValue(newValue);
                    onChange(
                        {
                            type: "Percent",
                            value: newValue
                        }
                    )
                }
            }}
            className="border p-2 w-full rounded"
        />
    )
}