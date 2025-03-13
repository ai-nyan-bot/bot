import React, {FC} from "react";
import {
    CompareVenuePumpfun,
    ComposedCurveProgress,
    ComposedPumpFunQuick,
    ComposedSimpleAge,
    ComposedSimpleSwapBuyAggregate,
    ComposedSimpleSwapSellAggregate,
    ComposedSimpleSwapTotalAggregate,
    DEFAULT_COMPARE_VENUE_PUMPFUN,
    DEFAULT_COMPOSED_CURVE_PROGRESS,
    DEFAULT_COMPOSED_SIMPLE_AGE,
    DEFAULT_COMPOSED_SIMPLE_SWAP_BUY_AGGREGATE,
    DEFAULT_COMPOSED_SIMPLE_SWAP_SELL_AGGREGATE,
    DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL_AGGREGATE
} from "@types";
import {CurveProgressCompose} from "@components/editor/condition/compose/curve-progress.tsx";
import {SimpleSwapAggregateCompose} from "@components/editor/condition/compose/simple";
import {SimpleAgeCompose} from "@components/editor/condition/compose/simple/age.tsx";

export type PumpFunComposeQuickProps = {
    condition: ComposedPumpFunQuick;
    onChange: (condition: ComposedPumpFunQuick) => void;

}

export const PumpFunComposeQuick: FC<PumpFunComposeQuickProps> = ({condition, onChange}) => {
    const _venue = condition.condition.conditions[0] || DEFAULT_COMPARE_VENUE_PUMPFUN as unknown as CompareVenuePumpfun;
    const age = condition.condition.conditions[1] || DEFAULT_COMPOSED_SIMPLE_AGE as unknown as ComposedSimpleAge;
    const curve_progress = condition.condition.conditions[2] || DEFAULT_COMPOSED_CURVE_PROGRESS as unknown as ComposedCurveProgress;
    const swap_total = condition.condition.conditions[3] || DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL_AGGREGATE as unknown as ComposedSimpleSwapTotalAggregate;
    const swap_buy = condition.condition.conditions[4] || DEFAULT_COMPOSED_SIMPLE_SWAP_BUY_AGGREGATE as unknown as ComposedSimpleSwapBuyAggregate;
    const swap_sell = condition.condition.conditions[5] || DEFAULT_COMPOSED_SIMPLE_SWAP_SELL_AGGREGATE as unknown as ComposedSimpleSwapSellAggregate;

    return (
        <div className="px-4 w-full max-w-md mx-auto space-y-4">

            <Container title={"Curve Progress"}>
                <CurveProgressCompose
                    condition={curve_progress}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[2] = updated;
                        onChange(changed);
                    }}
                />
            </Container>

            <Container title={"Age"}>
                <SimpleAgeCompose
                    condition={age}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[1] = updated;
                        onChange(changed);
                    }}
                />
            </Container>

            <Container title={"Total Txn"}>
                <SimpleSwapAggregateCompose
                    condition={swap_total}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[3] = updated as ComposedSimpleSwapTotalAggregate;
                        onChange(changed);
                    }}
                />
            </Container>

            <Container title={"Buy Txn"}>
                <SimpleSwapAggregateCompose
                    condition={swap_buy}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[4] = updated as ComposedSimpleSwapBuyAggregate;
                        onChange(changed);
                    }}
                />
            </Container>

            <Container title={"Sell Txn"}>
                <SimpleSwapAggregateCompose
                    condition={swap_sell}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[5] = updated as ComposedSimpleSwapSellAggregate;
                        onChange(changed);
                    }}
                />
            </Container>

        </div>
    )

};

type ContainerProps = {
    title: string;
    children: React.ReactNode;
};

const Container: FC<ContainerProps> = ({title, children}) => {
    return (
        <div className={"flex flex-col border-2 border-zinc-300 p-2"}>
            <span className={"pb-2 w-full text-center font-light"}>{title}</span>
            {children}
        </div>

    )
}

