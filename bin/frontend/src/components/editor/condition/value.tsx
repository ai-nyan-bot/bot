import React, {FC, useState} from "react";
import {ValueNumber, ValueNumberType} from "@types";

export type ValueNumberInputProps = {
    supported: Array<ValueNumberType>;
    defaultValue?: ValueNumber;
    value?: ValueNumber;
    onChange?: (value: ValueNumber) => void;
};

export const ValueNumberInput: FC<ValueNumberInputProps> = ({supported, value, defaultValue, onChange}) => {
    const [selectedType, setSelectedType] = useState<ValueNumberType>(value?.type || defaultValue?.type || supported[0]);
    const [inputValue, setInputValue] = useState<number | undefined>(value?.value || defaultValue?.value);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const input = e.target.value.trim();
        if (input === "") {
            setInputValue(undefined);
        } else {
            const parsedValue = (selectedType === 'COUNT')
                ? parseInt(input, 10)
                : parseFloat(input);

            setInputValue(parsedValue)

            if (onChange) {
                onChange(
                    {
                        type: selectedType,
                        value: parsedValue
                    }
                )
            }
        }
    }

    const handleTypeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newType = e.target.value as ValueNumberType;
        setSelectedType(newType);

        let value = inputValue;
        if (newType === 'COUNT') {
            if (value) {
                value = Math.trunc(value);
                setInputValue(value);
            }
        }

        if (value) {
            if (onChange) {
                onChange(
                    {
                        type: newType,
                        value
                    }
                )
            }
        }
    };

    const options = [
        {value: 'COUNT', label: "Count"},
        {value: 'PERCENT', label: '%'},
        {value: 'QUOTE', label: 'SOL'},
        {value: 'USD', label: 'USD'}
    ].filter(opt => supported.find(t => opt.value === t))
        .map(opt => <option value={opt.value}>{opt.label}</option>);

    if (options.length === 0) {
        return null;
    }

    return (
        <div className="flex items-center space-x-2">
            <input
                type="number"
                value={inputValue}
                onChange={handleInputChange}
                className="border p-2 w-full rounded"
            />
            <select
                value={selectedType}
                onChange={handleTypeChange}
                className="border p-2 rounded bg-white"
                disabled={options.length < 2}
            >
                {options}
            </select>
        </div>
    );
}
