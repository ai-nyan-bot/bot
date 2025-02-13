import {Operator} from "@types";
import React, {FC} from "react";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";

export const useOperatorOptions = (supported: Array<Operator>): Array<{
    value: Operator,
    label: string
}> => {
    // FIXME i18n
    return [
        {value: Operator.GREATER_THAN, label: "greater than"},
        {value: Operator.INCREASED_BY, label: "increased by"},
    ].filter(opt => supported.find(v => v === opt.value));
}

export type SelectOperatorProps = {
    defaultOperator?: Operator;
    supported: Array<Operator>
    onChange?: (value: Operator) => void
}

export const SelectOperator: FC<SelectOperatorProps> = ({defaultOperator, supported, onChange}) => {
    const operatorOptions = useOperatorOptions(supported);
    if (supported.length === 0) {
        return null;
    }
    return (
        <Select defaultValue={defaultOperator ?? supported[0]}
                onValueChange={(value) => {
                    if (onChange) {
                        onChange(value as Operator);
                    }
                }}>
            <SelectTrigger className="w-full">
                <SelectValue/>
            </SelectTrigger>
            <SelectContent>
                {operatorOptions.map(({value, label}) => (
                    <SelectItem key={value} value={value}>
                        {label}
                    </SelectItem>
                ))}
            </SelectContent>
        </Select>

    );
}