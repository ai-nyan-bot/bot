import React, {FC} from "react";
import {ComposedCurveProgress, ComposedPumpFunQuick} from "@types";
import {CurveProgressCompose} from "@components/editor/condition/compose/curve-progress.tsx";

export type PumpFunComposeQuickProps = {
    condition: ComposedPumpFunQuick;
    onChange: (condition: ComposedPumpFunQuick) => void;

}

export const PumpFunComposeQuick: FC<PumpFunComposeQuickProps> = ({condition, onChange}) => {
    console.log(JSON.stringify(condition, null, 2));
    const curve_progress = condition.condition.conditions[0] as unknown as ComposedCurveProgress;

    return (
        <div className="w-full max-w-md mx-auto">
            <CurveProgressCompose
                condition={curve_progress}
                onChange={updated => {
                    console.log("Curve progress updated", updated)
                }}
            />

        </div>
    );
};

