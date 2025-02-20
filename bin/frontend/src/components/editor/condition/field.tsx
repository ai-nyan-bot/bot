import {Field} from "@types";
import React, {FC, useState} from "react";

export const useFieldOptions = (supported: Array<Field>): Array<{
    value: Field,
    label: string
}> => {
    // FIXME i18n
    return [
        // {value: Field.PRICE, label: 'Price'},
        {value: Field.TRADES, label: 'Trades'},
        {value: Field.TRADES_BUY, label: 'Buy trades'},
        {value: Field.TRADES_SELL, label: 'Sell trades'},
        // {value: Field.VOLUME, label: 'Volume'},
    ]
}

export type SelectFieldProps = {
    defaultField?: Field;
    supported: Array<Field>
    onChange?: (value: Field) => void
}

export const SelectField: FC<SelectFieldProps> = ({defaultField, supported, onChange}) => {
    const [selected, setSelected] = useState<Field>(defaultField || supported[0]);

    const options = useFieldOptions(supported)
        .map(opt => <option value={opt.value}>{opt.label}</option>);

    if (supported.length === 0) {
        return null;
    }

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newField = e.target.value as Field;
        setSelected(newField);
        if (onChange) {
            onChange(newField)
        }
    };

    return (
        <select
            value={selected}
            onChange={handleChange}
            className="w-full border p-2 rounded bg-white"
            disabled={options.length < 2}
        >
            {options}
        </select>
    );
}