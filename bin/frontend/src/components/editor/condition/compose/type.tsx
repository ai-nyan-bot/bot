import {ComposeType} from "@types";
import React, {FC, useState} from "react";

export const useComposeTypeOptions = (supported: Array<ComposeType>): Array<{
    value: ComposeType,
    label: string
}> => {
    // FIXME i18n
    return [
        {value: ComposeType.CURVE_PROGRESS, label: 'Curve Progress'},
    ]

}

export type SelectComposeTypeProps = {
    defaultType?: ComposeType;
    supported: Array<ComposeType>
    onChange?: (value: ComposeType) => void
}

export const SelectComposeType: FC<SelectComposeTypeProps> = ({defaultType, supported, onChange}) => {
    const [selected, setSelected] = useState<ComposeType>(defaultType || supported[0]);

    const options = useComposeTypeOptions(supported)
        .map(opt => <option key={opt.value} value={opt.value}>{opt.label}</option>);

    if (supported.length === 0) {
        return null;
    }

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newComposeType = e.target.value as ComposeType;
        setSelected(newComposeType);
        if (onChange) {
            onChange(newComposeType)
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