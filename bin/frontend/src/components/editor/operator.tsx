import {Operator} from "@types";
import React, {FC, useState} from "react";

export const useOperatorOptions = (supported: Array<Operator>): Array<{
    value: Operator,
    label: string
}> => {
    // FIXME i18n
    return [
        {value: Operator.INCREASED_BY_MORE_THAN, label: "increased by more than"},
        {value: Operator.INCREASED_BY_MORE_THAN_EQUAL, label: "increased by more than or equal to"},
        {value: Operator.INCREASED_BY_LESS_THAN, label: "increased by less than"},
        {value: Operator.INCREASED_BY_LESS_THAN_EQUAL, label: "increased by less than or equal to"},
        {value: Operator.DECREASED_BY_MORE_THAN, label: "decreased by more than"},
        {value: Operator.DECREASED_BY_MORE_THAN_EQUAL, label: "decreased by more than or equal to"},
        {value: Operator.DECREASED_BY_LESS_THAN, label: "decreased by less than"},
        {value: Operator.DECREASED_BY_LESS_THAN_EQUAL, label: "decreased by less than or equal to"},
        {value: Operator.MORE_THAN, label: "more than"},
        {value: Operator.MORE_THAN_EQUAL, label: "more than or equal to"},
        {value: Operator.LESS_THAN, label: "less than"},
        {value: Operator.LESS_THAN_EQUAL, label: "less than or equal to"},
    ].filter(opt => supported.find(v => v === opt.value));
}

export type SelectOperatorProps = {
    defaultOperator?: Operator;
    supported: Array<Operator>
    onChange?: (value: Operator) => void
}

export const SelectOperator: FC<SelectOperatorProps> = ({defaultOperator, supported, onChange}) => {
    const [selected, setSelected] = useState<Operator>(defaultOperator || supported[0]);

    const options = useOperatorOptions(supported)
        .map(opt => <option value={opt.value}>{opt.label}</option>);

    if (supported.length === 0) {
        return null;
    }

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newOperator = e.target.value as Operator;
        setSelected(newOperator);
        if (onChange) {
            onChange(newOperator)
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