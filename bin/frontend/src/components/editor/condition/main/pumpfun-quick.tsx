import React, {FC} from "react";
import {ComposedCurveProgress, ComposedPumpFunQuick} from "@types";
import {CurveProgressCompose} from "@components/editor/condition/compose/curve-progress.tsx";
import {SimpleSwapCompose, SimpleSwapType} from "@components/editor/condition/compose/simple";

export type PumpFunComposeQuickProps = {
    condition: ComposedPumpFunQuick;
    onChange: (condition: ComposedPumpFunQuick) => void;

}

export const PumpFunComposeQuick: FC<PumpFunComposeQuickProps> = ({condition, onChange}) => {
    console.log(JSON.stringify(condition, null, 2));
    const curve_progress = condition.condition.conditions[0] as unknown as ComposedCurveProgress;

    return (
        <div className="w-full max-w-md mx-auto">
            <div className="w-full border-2">
                <h1>Bonding Curve Progress</h1>
                <CurveProgressCompose
                    condition={curve_progress}
                    onChange={updated => {
                        console.log("Curve progress updated", updated)
                    }}
                />
            </div>


            <div className={"flex flex-col border-2"}>
                <span> Total Txn </span>
                <SimpleSwapCompose
                    type={SimpleSwapType.TOTAL}
                />
            </div>

            <div className={"flex flex-col border-2"}>
                <span> Buy Txn </span>
                <SimpleSwapCompose
                    type={SimpleSwapType.BUY}
                />
            </div>

            <div className={"flex flex-col border-2"}>
                <span> Sell Txn </span>
                <SimpleSwapCompose
                    type={SimpleSwapType.SELL}
                />
            </div>
        </div>
    )

};

