import React, {FC} from "react";
import {ValuePercent} from "@types";

export type ValuePercentInputProps = {
    defaultValue?: number;
    value?: number;
    onChange?: (value: ValuePercent) => void;
}

export const ValuePercentInput: FC<ValuePercentInputProps> = ({value, defaultValue, onChange}) => {
    return (
        <input
            type="number"
            value={value?.toString() || defaultValue?.toString()}
            onChange={(e) => {
                if (onChange) {
                    const input = e.target.value;
                    if (input.trim() === "") {
                        // setStateValue(null);
                        onChange(
                            {
                                type: 'PERCENT',
                                value: 0
                            }
                        )
                        return;
                    }
                    onChange(
                        {
                            type: 'PERCENT',
                            value: parseFloat(e.target.value)
                        }
                    )
                }
            }}
            className="border p-2 w-full rounded"
        />
    )
}