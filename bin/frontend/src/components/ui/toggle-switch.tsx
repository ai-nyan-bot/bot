import React, {FC} from "react";
import {Switch} from "./switch";
import {Label} from "./label";

export type ToggleSwitchProps = {
    activeLabel: string;
    inactiveLabel: string;
    value: boolean;
    onChange: (value: boolean) => void;
}

export const ToggleSwitch: FC<ToggleSwitchProps> = ({activeLabel, inactiveLabel, value, onChange}) => {
    const selectedStyle = "font-bold";
    const unselectedStyle = "text-zinc-400";
    return (

        <div className="flex items-center space-x-3">
            {<Label className={value ? unselectedStyle : selectedStyle}>{inactiveLabel}</Label>}
            <Switch
                checked={value}
                onCheckedChange={onChange}
                className="data-[state=checked]:bg-green-500"
            />
            {<Label className={value ? selectedStyle : unselectedStyle}>{activeLabel}</Label>}
        </div>
    );
};