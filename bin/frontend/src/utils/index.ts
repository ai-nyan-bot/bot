import {Decimal} from "decimal.js";

export * from './theme.ts';

export const imageUrl = (name: string) => {
    return `/images/${name}`;
}

export const audioUrl = (name: string) => {
    return `/audio/${name}`;
}


export const formatDecimal = (value: Decimal) => {
    return value.toFixed(0);
}