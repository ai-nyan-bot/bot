import React, {FC, useState} from "react";
import {Decimal} from "decimal.js";
import {ValuePercent} from "@types";

export type ValuePercentInputProps = {
    defaultValue?: Decimal;
    value?: Decimal;
    onChange?: (value: ValuePercent) => void;
}

export const ValuePercentInput: FC<ValuePercentInputProps> = ({value, defaultValue, onChange}) => {
    const [stateValue, setStateValue] = useState(value || defaultValue || new Decimal(0));
    return (
        <input
            type="number"
            value={(stateValue).toString()}
            onChange={(e) => {
                if (onChange) {
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