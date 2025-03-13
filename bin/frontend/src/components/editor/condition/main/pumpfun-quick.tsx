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
    DEFAULT_COMPOSED_SIMPLE_AGE, DEFAULT_COMPOSED_SIMPLE_MARKET_CAP,
    DEFAULT_COMPOSED_SIMPLE_SWAP_BUY_AGGREGATE,
    DEFAULT_COMPOSED_SIMPLE_SWAP_SELL_AGGREGATE,
    DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL_AGGREGATE
} from "@types";
import {CurveProgressCompose} from "@components/editor/condition/compose/curve-progress.tsx";
import {SimpleSwapAggregateCompose} from "@components/editor/condition/compose/simple";
import {SimpleAgeCompose} from "@components/editor/condition/compose/simple/age.tsx";
import {SimpleMarketCapCompose} from "@components/editor/condition/compose/simple/market-cap.tsx";

export type PumpFunComposeQuickProps = {
    condition: ComposedPumpFunQuick;
    onChange: (condition: ComposedPumpFunQuick) => void;

}

export const PumpFunComposeQuick: FC<PumpFunComposeQuickProps> = ({condition, onChange}) => {
    const _venue = condition.condition.conditions[0] || DEFAULT_COMPARE_VENUE_PUMPFUN;
    const age = condition.condition.conditions[1] || DEFAULT_COMPOSED_SIMPLE_AGE;
    const curve_progress = condition.condition.conditions[2] || DEFAULT_COMPOSED_CURVE_PROGRESS;
    const swap_total = condition.condition.conditions[3] || DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL_AGGREGATE;
    const swap_buy = condition.condition.conditions[4] || DEFAULT_COMPOSED_SIMPLE_SWAP_BUY_AGGREGATE;
    const swap_sell = condition.condition.conditions[5] || DEFAULT_COMPOSED_SIMPLE_SWAP_SELL_AGGREGATE;
    const market_cap = condition.condition.conditions[6] || DEFAULT_COMPOSED_SIMPLE_MARKET_CAP;

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

            <Container title={"Market Cap"}>
                <SimpleMarketCapCompose
                    condition={market_cap}
                    onChange={updated => {
                        const changed = {...condition};
                        console.log(updated)
                        changed.condition.conditions[6] = updated;
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

