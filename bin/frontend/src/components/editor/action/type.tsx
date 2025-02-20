import {ActionType} from "@types";
import React, {FC, useState} from "react";

export const useActionTypeOptions = (supported: Array<ActionType>): Array<{
    value: ActionType,
    label: string
}> => {
    // FIXME i18n
    return [
        {value: ActionType.NOTIFY_TELEGRAM, label: "Notify"},
    ].filter(opt => supported.find(v => v === opt.value));
}

export type SelectActionTypeProps = {
    defaultActionType?: ActionType;
    supported: Array<ActionType>
    onChange?: (value: ActionType) => void
}

export const SelectActionType: FC<SelectActionTypeProps> = ({defaultActionType, supported, onChange}) => {
    const [selected, setSelected] = useState<ActionType>(defaultActionType || supported[0]);

    const options = useActionTypeOptions(supported)
        .map(opt => <option value={opt.value}>{opt.label}</option>);

    if (supported.length === 0) {
        return null;
    }

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newActionType = e.target.value as ActionType;
        setSelected(newActionType);
        if (onChange) {
            onChange(newActionType)
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