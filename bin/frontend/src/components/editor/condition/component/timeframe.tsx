import {Timeframe} from "@types";
import React, {FC, useState} from "react";


export const useTimeframeOptions = (supported: Array<Timeframe>): Array<{ value: Timeframe, label: string }> => {
    // FIXME i18n
    return [
        {value: Timeframe.M1, label: "1 minute"},
        {value: Timeframe.M5, label: "5 minutes"},
        {value: Timeframe.M15, label: "15 minutes"},
        {value: Timeframe.H1, label: "1 hour"},
        {value: Timeframe.H6, label: "6 hours"},
        {value: Timeframe.D1, label: "1 day"},
    ].filter(opt => supported.find(v => v === opt.value));
}

export type SelectTimeframeProps = {
    defaultTimeframe?: Timeframe;
    supported: Array<Timeframe>
    onChange?: (value: Timeframe) => void
}

export const SelectTimeframe: FC<SelectTimeframeProps> = ({defaultTimeframe, supported, onChange}) => {
    const [selected, setSelected] = useState<Timeframe>(defaultTimeframe || supported[0]);

    const options = useTimeframeOptions(supported)
        .map(opt => <option key={opt.value} value={opt.value}>{opt.label}</option>);

    if (supported.length === 0) {
        return null;
    }

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newTimeframe = e.target.value as Timeframe;
        setSelected(newTimeframe);
        if (onChange) {
            onChange(newTimeframe)
        }
    };

    return (
        <select
            value={selected}
            onChange={handleChange}
            className="border p-1 rounded bg-white"
            disabled={options.length < 2}
        >
            {options}
        </select>
    );
}

export type TimeframeTextProps = {
    value: Timeframe
}

export const TimeframeText: FC<TimeframeTextProps> = ({value}) => {
    switch (value){
        case Timeframe.M1:
            return (
                <span>1 minute</span>
            )
        case Timeframe.M5:
            return (
                <span>5 minutes</span>
            )
        case Timeframe.M15:
            return (
                <span>15 minutes</span>
            )
        case Timeframe.H1:
            return (
                <span>1 hour</span>
            )
        case Timeframe.H6:
            return (
                <span>6 hours</span>
            )
        case Timeframe.D1:
            return (
                <span>24 hours</span>
            )
        default:
            throw Error(`Unsupported timeframe: ${value}`);
    }
}