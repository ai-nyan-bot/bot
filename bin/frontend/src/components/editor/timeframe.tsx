import {Timeframe} from "@types";
import React, {FC} from "react";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";


export const useTimeframeOptions = (supported: Array<Timeframe>): Array<{ value: Timeframe, label: string }> => {
    // FIXME i18n
    return [
        {value: Timeframe.M1, label: "1 minute"},
        {value: Timeframe.M5, label: "5 minutes"},
        {value: Timeframe.M15, label: "15 minutes"},
    ].filter(opt => supported.find(v => v === opt.value));
}

export type SelectTimeframeProps = {
    defaultValue?: Timeframe;
    supported: Array<Timeframe>
    onChange?: (value: Timeframe) => void
}

export const SelectTimeframe: FC<SelectTimeframeProps> = ({defaultValue, supported, onChange}) => {
    const timeframeOptions = useTimeframeOptions(supported);
    if (supported.length === 0) {
        return null;
    }
    return (
        <Select defaultValue={defaultValue ?? supported[0]}
                onValueChange={(value) => {
                    if (onChange) {
                        onChange(value as Timeframe);
                    }
                }}>
            <SelectTrigger className="w-full">
                <SelectValue/>
            </SelectTrigger>
            <SelectContent>
                {timeframeOptions.map(({value, label}) => (
                    <SelectItem key={value} value={value}>
                        {label}
                    </SelectItem>
                ))}
            </SelectContent>
        </Select>

    );
}