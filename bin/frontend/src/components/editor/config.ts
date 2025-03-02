import {ALL_TIMEFRAMES, COUNT_AND_PERCENT, Field, Operator, Timeframe, ValueType} from "@types";


export const config: Record<
    Field,
    {
        operators: {
            [key in Operator]?: {
                valueTypes: Array<ValueType>;
                timeframes: Array<Timeframe>;
            };
        };
    }
> = {
    [Field.CURVE_PROGRESS]: {
        operators: {
            [Operator.MORE_THAN]: {
                valueTypes: [ValueType.PERCENT],
                timeframes: []
            },
            [Operator.LESS_THAN]: {
                valueTypes: [ValueType.PERCENT],
                timeframes: []
            }
        }
    },
    [Field.CURVE_PROGRESS_AGE]: {
        operators: {
            [Operator.MORE_THAN]: {
                valueTypes: [ValueType.DURATION],
                timeframes: []
            },
            [Operator.LESS_THAN]: {
                valueTypes: [ValueType.DURATION],
                timeframes: []
            }
        }
    },
    [Field.TRADES]: {
        operators: {
            [Operator.INCREASED_BY_MORE_THAN]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
            [Operator.INCREASED_BY_MORE_THAN_EQUAL]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
            [Operator.DECREASED_BY_MORE_THAN]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
            [Operator.DECREASED_BY_MORE_THAN_EQUAL]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
        },
    },
    [Field.TRADES_BUY]: {
        operators: {
            [Operator.MORE_THAN]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
            [Operator.LESS_THAN]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
        },
    },
    [Field.TRADES_SELL]: {
        operators: {
            [Operator.MORE_THAN]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
            [Operator.LESS_THAN]: {
                valueTypes: COUNT_AND_PERCENT,
                timeframes: ALL_TIMEFRAMES
            },
        },
    },
};
