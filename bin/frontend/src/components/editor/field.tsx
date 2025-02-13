import {Field} from "@types";
import React, {FC} from "react";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";

export const useFieldOptions = (supported: Array<Field>): Array<{
    value: Field,
    label: string
}> => {
    // FIXME i18n
    return [
        {value: Field.PRICE, label: 'Price'},
        {value: Field.TRADES, label: 'Trades'},
        {value: Field.VOLUME, label: 'Volume'},
    ]
}

export type SelectFieldProps = {
    defaultField?: Field;
    supported: Array<Field>
    onChange?: (value: Field) => void
}

export const SelectField: FC<SelectFieldProps> = ({defaultField, supported, onChange}) => {
    const fieldOptions = useFieldOptions(supported);
    if (supported.length === 0) {
        return null;
    }
    return (
        <Select defaultValue={defaultField ?? supported[0]}
                onValueChange={(value) => {
                    if (onChange) {
                        onChange(value as Field);
                    }
                }}>
            <SelectTrigger className="w-full">
                <SelectValue/>
            </SelectTrigger>
            <SelectContent>
                {fieldOptions.map(({value, label}) => (
                    <SelectItem key={value} value={value}>
                        {label}
                    </SelectItem>
                ))}
            </SelectContent>
        </Select>

    );
}