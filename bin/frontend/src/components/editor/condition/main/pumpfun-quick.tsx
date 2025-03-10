import React, {FC} from "react";
import {ComposedCurveProgress, ComposedPumpFunQuick, ComposedSimpleSwapTotal} from "@types";
import {CurveProgressCompose} from "@components/editor/condition/compose/curve-progress.tsx";
import {SimpleSwapCompose} from "@components/editor/condition/compose/simple";

export type PumpFunComposeQuickProps = {
    condition: ComposedPumpFunQuick;
    onChange: (condition: ComposedPumpFunQuick) => void;

}

export const PumpFunComposeQuick: FC<PumpFunComposeQuickProps> = ({condition, onChange}) => {
    const curve_progress = condition.condition.conditions[0] as unknown as ComposedCurveProgress;
    const swap_total = condition.condition.conditions[1] as unknown as ComposedSimpleSwapTotal;


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

            {/*<div className={"flex flex-col border-2"}>*/}
            {/*    <span> Buy Txn </span>*/}
            {/*    <SimpleSwapCompose*/}
            {/*        type={SimpleSwapType.BUY}*/}
            {/*    />*/}
            {/*</div>*/}

            {/*<div className={"flex flex-col border-2"}>*/}
            {/*    <span> Sell Txn </span>*/}
            {/*    <SimpleSwapCompose*/}
            {/*        type={SimpleSwapType.SELL}*/}
            {/*    />*/}
            {/*</div>*/}
        </div>
    )

};

