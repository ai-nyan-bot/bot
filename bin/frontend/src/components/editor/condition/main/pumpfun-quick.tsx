import React, {FC} from "react";
import {
    ComposedCurveProgress,
    ComposedPumpFunQuick,
    ComposedSimpleSwapBuy,
    ComposedSimpleSwapSell,
    ComposedSimpleSwapTotal,
    DEFAULT_COMPOSED_CURVE_PROGRESS,
    DEFAULT_COMPOSED_SIMPLE_SWAP_BUY,
    DEFAULT_COMPOSED_SIMPLE_SWAP_SELL,
    DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL
} from "@types";
import {CurveProgressCompose} from "@components/editor/condition/compose/curve-progress.tsx";
import {SimpleSwapCompose} from "@components/editor/condition/compose/simple";

export type PumpFunComposeQuickProps = {
    condition: ComposedPumpFunQuick;
    onChange: (condition: ComposedPumpFunQuick) => void;

}

export const PumpFunComposeQuick: FC<PumpFunComposeQuickProps> = ({condition, onChange}) => {
    const curve_progress = condition.condition.conditions[0] || DEFAULT_COMPOSED_CURVE_PROGRESS as unknown as ComposedCurveProgress;
    const swap_total = condition.condition.conditions[1] || DEFAULT_COMPOSED_SIMPLE_SWAP_TOTAL as unknown as ComposedSimpleSwapTotal;
    const swap_buy = condition.condition.conditions[2] || DEFAULT_COMPOSED_SIMPLE_SWAP_BUY as unknown as ComposedSimpleSwapBuy;
    const swap_sell = condition.condition.conditions[3] || DEFAULT_COMPOSED_SIMPLE_SWAP_SELL as unknown as ComposedSimpleSwapSell;


    return (
        <div className="w-full max-w-md mx-auto">
            <div className="w-full border-2">
                <h1>Bonding Curve Progress</h1>
                <CurveProgressCompose
                    condition={curve_progress}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[0] = updated;
                        onChange(changed);
                    }}
                />
            </div>


            <div className={"flex flex-col border-2"}>
                <span> Total Txn </span>
                <SimpleSwapCompose
                    condition={swap_total}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[1] = updated as ComposedSimpleSwapTotal;
                        onChange(changed);
                    }}
                />
            </div>

            <div className={"flex flex-col border-2"}>
                <span> Buy Txn </span>
                <SimpleSwapCompose
                    condition={swap_buy}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[2] = updated as ComposedSimpleSwapBuy;
                        onChange(changed);
                    }}
                />
            </div>


            <div className={"flex flex-col border-2"}>
                <span> Sell Txn </span>
                <SimpleSwapCompose
                    condition={swap_sell}
                    onChange={updated => {
                        const changed = {...condition};
                        changed.condition.conditions[3] = updated as ComposedSimpleSwapSell;
                        onChange(changed);
                    }}
                />
            </div>

        </div>
    )

};

