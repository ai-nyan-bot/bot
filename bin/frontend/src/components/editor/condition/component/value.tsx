import React, {FC, useEffect, useState} from "react";
import {ValueNumber, ValueNumberType} from "@types";

export type ValueNumberInputProps = {
    id?: string;
    supported: Array<ValueNumberType>;
    defaultValue?: ValueNumber;
    value?: ValueNumber;
    onChange?: (value: ValueNumber) => void;
    hideValueSelect?: boolean;
    placeholder?: string;
};

export const ValueNumberInput: FC<ValueNumberInputProps> = ({
                                                                id,
                                                                supported,
                                                                value,
                                                                defaultValue,
                                                                onChange,
                                                                hideValueSelect,
                                                                placeholder,
                                                            }) => {

    const [selectedType, setSelectedType] = useState<ValueNumberType>(
        value?.type || defaultValue?.type || supported[0]
    );

    const [inputValue, setInputValue] = useState<number | undefined>(() => {
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
            setSelectedType(value.type);
            setInputValue(value.value);
        }
    }, [value]);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const input = e.target.value.trim();
        const parsedValue = selectedType === 'COUNT' ? parseInt(input, 10) : parseFloat(input);

        if (input === '' || isNaN(parsedValue)) {
            setInputValue(undefined);
            onChange?.({type: selectedType, value: NaN});
        } else {
            setInputValue(parsedValue);
            onChange?.({type: selectedType, value: parsedValue});
        }
    };

    const handleTypeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newType = e.target.value as ValueNumberType;
        setSelectedType(newType);

        let adjustedValue = inputValue;
        if (newType === 'COUNT' && inputValue !== undefined) {
            adjustedValue = Math.trunc(inputValue);
            setInputValue(adjustedValue);
        }

        onChange?.({
            type: newType,
            value: adjustedValue ?? NaN
        });
    };

    const options = [
        {value: 'COUNT', label: 'Count'},
        {value: 'PERCENT', label: '%'},
        {value: 'SOL', label: 'SOL'},
        {value: 'QUOTE', label: 'SOL'},
        {value: 'USD', label: 'USD'}
    ]
        .filter(opt => supported.includes(opt.value as ValueNumberType))
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
                value={inputValue ?? ''}
                onChange={handleInputChange}
                className="border p-2 w-full rounded"
                placeholder={placeholder || "Enter value"}
            />
            {!hideValueSelect && (
                <select
                    value={selectedType}
                    onChange={handleTypeChange}
                    className="border p-2 rounded bg-white"
                    disabled={options.length < 2}
                >
                    {options}
                </select>
            )}
        </div>
    );
};