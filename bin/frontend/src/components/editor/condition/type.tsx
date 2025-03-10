import {ComposeType} from "@types";
import React, {FC, useState} from "react";

export const useConditionTypeOptions = (supported: Array<ComposeType>): Array<{
    value: ComposeType,
    label: string
}> => {
    // FIXME i18n
    return [
        {value: ComposeType.PUMP_FUN_QUICK, label: 'PumpFun quick & easy'},
        {value: ComposeType.GROUP, label: 'Custom'},
    ]

}

export type SelectTypeProps = {
    defaultType?: ComposeType;
    supported: Array<ComposeType>
    onChange?: (value: ComposeType.PUMP_FUN_QUICK | ComposeType.GROUP) => void
}

export const SelectConditionType: FC<SelectTypeProps> = ({defaultType, supported, onChange}) => {
    const [selected, setSelected] = useState<ComposeType>(defaultType || supported[0]);

    const options = useConditionTypeOptions(supported)
        .map(opt => <option key={opt.value} value={opt.value}>{opt.label}</option>);

    if (supported.length === 0) {
        return null;
    }

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newComposeType = e.target.value as ComposeType;
        setSelected(newComposeType);
        if (onChange) {
            if (newComposeType === ComposeType.PUMP_FUN_QUICK || newComposeType === ComposeType.GROUP) {
                onChange(newComposeType)
            }
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