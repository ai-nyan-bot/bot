import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card.tsx";
import {ComposedPumpFunQuick, ComposeType, Condition} from "@types";
import React, {FC, useState} from "react";
import {SelectConditionType} from "./type.tsx";
import {PumpFunComposeQuick} from "./main";

export type ConditionEditorProps = {
    condition: Condition;
};

export const ConditionEditor: FC<ConditionEditorProps> = ({condition}) => {
    const [composition, setComposition] = useState<ComposeType>(ComposeType.PUMP_FUN_QUICK)

    return (
        <Card className="w-full shadow-none border-0">
            <CardHeader className={"flex flex-row"}>
                <CardTitle className="pr-10 font-semibold text-yellow-600 flex items-center">IF</CardTitle>
                <SelectConditionType
                    defaultType={ComposeType.PUMP_FUN_QUICK}
                    supported={[
                        ComposeType.PUMP_FUN_QUICK,
                        ComposeType.GROUP
                    ]}
                    onChange={setComposition}
                />
            </CardHeader>
            <CardContent className="w-full p-0">

                {composition === ComposeType.PUMP_FUN_QUICK && (
                    <PumpFunComposeQuick
                        key={condition.id}
                        condition={condition as unknown as ComposedPumpFunQuick}
                        onChange={function (condition: ComposedPumpFunQuick): void {
                            throw new Error("Function not implemented.");
                        }}/>
                )}

                {composition === ComposeType.GROUP && (
                    <h1> Custom hard core mode</h1>
                )}


                {/*<div className="w-full">*/}
                {/*    <ConditionList*/}
                {/*        condition={condition}*/}
                {/*        isRoot={true}*/}
                {/*        onAdd={addCondition}*/}
                {/*        onRemove={removeCondition}*/}
                {/*        onConditionChange={(condition: Condition) => {*/}
                {/*            updateCondition(condition);*/}
                {/*        }}*/}
                {/*    />*/}
                {/*</div>*/}
            </CardContent>
        </Card>

    )

}
