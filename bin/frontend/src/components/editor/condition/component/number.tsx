import {ValueNumber, ValueType} from "@types";
import React, {FC} from "react";

export type NumberTextProps = {
    type: ValueType;
    value: number;
}

export const NumberText: FC<NumberTextProps | ValueNumber> = ({value, type}) => {
    switch (type) {
        case ValueType.SOL:
            return (<span>{value} SOL</span>)
        case ValueType.USD:
            return (<span>${value}</span>)
        default:
            throw Error(`Unsupported type: ${type}`);
    }
}