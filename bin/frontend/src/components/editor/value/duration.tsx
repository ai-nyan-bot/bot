import React, {FC, useEffect, useState} from "react";
import {TimeUnit, ValueDuration, ValueType} from "@types";

export type ValueDurationInputProps = {
    id?: string;
    supported: Array<TimeUnit>;
    defaultValue?: ValueDuration;
    value?: ValueDuration;
    onChange?: (value: ValueDuration | undefined) => void;
    hideValueSelect?: boolean;
    placeholder?: string;
};

export const ValueDurationInput: FC<ValueDurationInputProps> = ({
                                                                    id,
                                                                    supported,
                                                                    value,
                                                                    defaultValue,
                                                                    onChange,
                                                                    hideValueSelect,
                                                                    placeholder,
                                                                }) => {

    const [selectedUnit, setSelectedUnit] = useState<TimeUnit>(
        value?.unit || defaultValue?.unit || supported[0]
    );

    const [selectedValue, setSelectedValue] = useState<number | undefined>(() => {
            if (value?.value) {
                if (isNaN(value.value)) {
                    return undefined;
                }
            }
            return value?.value ?? defaultValue?.value
        }
    );

    useEffect(() => {
        if (value && !isNaN(value.value)) {
            setSelectedUnit(value.unit);
            setSelectedValue(value.value);
        }
    }, [value]);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const input = e.target.value.trim();
        let parsedValue = parseInt(input, 10);

        if (input === '' || isNaN(parsedValue)) {
            setSelectedValue(undefined);
            onChange?.(undefined);
        } else {
            setSelectedValue(parsedValue);
            onChange?.({type: ValueType.DURATION, unit: selectedUnit, value: parsedValue});
        }
    };

    const handleUnitChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newUnit = e.target.value as TimeUnit;
        setSelectedUnit(newUnit);

        if (selectedValue) {
            onChange?.({
                type: ValueType.DURATION,
                unit: newUnit,
                value: selectedValue
            });
        }
    };

    const options = [
        {value: TimeUnit.MINUTE, label: 'min'},
        {value: TimeUnit.HOUR, label: 'hr'},
        {value: TimeUnit.DAY, label: 'day'},
    ]
        .filter(opt => supported.includes(opt.value))
        .map(opt => (
            <option key={opt.value} value={opt.value}>
                {opt.label}
            </option>
        ));

    if (options.length === 0) return null;

    return (
        <div
            id={id}
            className="flex items-center space-x-2">
            <input
                type="number"
                value={selectedValue ?? ''}
                onChange={handleInputChange}
                className="border p-2 w-full rounded"
                placeholder={placeholder || "Enter value"}
            />
            {!hideValueSelect && (
                <select
                    value={selectedUnit}
                    onChange={handleUnitChange}
                    className="border p-2 rounded bg-white"
                    disabled={options.length < 2}
                >
                    {options}
                </select>
            )}
        </div>
    );
};