import {ComposedPumpFunQuick, ComposeType, Condition} from "@types";
import React, {FC, useState} from "react";
import {SelectConditionType} from "./type.tsx";
import {PumpFunComposeQuick} from "./main";

export type ConditionEditorProps = {
    condition: Condition;
    onChange: (condition: Condition) => void;
};

export const ConditionEditor: FC<ConditionEditorProps> = ({condition, onChange}) => {
    const [type, setType] = useState<ComposeType>(ComposeType.PUMP_FUN_QUICK)
    return (
        <div className={"flex flex-col border-l-4 border-yellow-600"}>
            <div className={"px-4 flex flex-row"}>
                <span className="pr-10 font-semibold text-yellow-600 flex items-center">IF</span>
                <SelectConditionType
                    defaultType={ComposeType.PUMP_FUN_QUICK}
                    supported={[
                        ComposeType.PUMP_FUN_QUICK,
                    ]}
                    onChange={setType}
                />
            </div>

            <div className={"pt-4"}>
                {type === ComposeType.PUMP_FUN_QUICK && (
                    <PumpFunComposeQuick
                        key={condition.id}
                        condition={condition as unknown as ComposedPumpFunQuick}
                        onChange={(updated) => {
                            onChange(updated as unknown as Condition);
                        }}/>
                )}
            </div>
        </div>
    )
}
