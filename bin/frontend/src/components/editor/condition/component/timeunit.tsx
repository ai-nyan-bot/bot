import {TimeUnit, ValueDuration} from "@types";
import React, {FC} from "react";

export type DurationTextProps = {
    value: number;
    unit: TimeUnit
}

export const DurationText: FC<DurationTextProps | ValueDuration> = ({value, unit}) => {
    switch (unit) {
        case TimeUnit.SECOND:
            if (value === 1) {
                return (<span>1 second</span>)
            }
            return (<span>{value} seconds</span>)
        case TimeUnit.MINUTE:
            if (value === 1) {
                return (<span>1 minute</span>)
            }
            return (<span>{value} minutes</span>)
        case TimeUnit.HOUR:
            if (value === 1) {
                return (<span>1 hour</span>)
            }
            return (<span>{value} hours</span>)
        case TimeUnit.DAY:
            if (value === 1) {
                return (<span>1 day</span>)
            }
            return (<span>{value} days</span>)
        default:
            throw Error(`Unsupported TimeUnit: ${unit}`);
    }
}