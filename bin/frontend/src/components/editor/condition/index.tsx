import {ComposeWidget} from "@components/editor/condition/compose";
import {Card} from "@components/ui/card.tsx";
import {And, Compose, ComposeType, Condition, ConditionType, Field, Operator, Timeframe, Value} from "@types";
import React, {FC} from "react";
import {Button} from "@components/ui/button.tsx";
import {CompareWidget} from "./compare";

export * from './compare'
export * from './field'
export * from './operator'
export * from './timeframe'

export type ConditionListProps = {
    condition: Condition,
    isRoot: boolean

    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onFieldChange: (id: string, value: Field) => void;
    onComposeTypeChange: (id: string, value: ComposeType) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;

    onConditionChange: (condition: Condition) => void;

}


export const ConditionList: FC<ConditionListProps> = ({
                                                          condition,
                                                          isRoot,
                                                          onAdd,
                                                          onRemove,
                                                          onComposeTypeChange,
                                                          onFieldChange,
                                                          onOperatorChange,
                                                          onTimeframeChange,
                                                          onValueChange,
                                                          onConditionChange
                                                      }) => {
    return (
        <Card className="border bg-gray-50 relative">
            {
                !isRoot && (
                    <div className={"flex flex-row justify-end m-2"}>
                        <button
                            onClick={() => onRemove(condition.id)}
                            className="text-gray-500 hover:text-red-500"
                        >
                            ✖
                        </button>
                    </div>
                )}

            {condition.type === 'COMPARE' && (
                <CompareWidget
                    condition={condition}
                    // onFieldChange={onFieldChange}
                    onValueChange={onValueChange}
                    onTimeframeChange={onTimeframeChange}
                    onOperatorChange={onOperatorChange}
                />
            )}

            {condition.type === 'COMPOSE' && (
                <ComposeWidget
                    condition={condition}
                    onComposeTypeChange={onComposeTypeChange}
                    // onValueChange={onValueChange}
                    // onTimeframeChange={onTimeframeChange}
                    // onOperatorChange={onOperatorChange}
                    onConditionChange={onConditionChange}
                />
            )}

            {(condition.type === 'AND') && (
                <Group
                    condition={condition}
                    isRoot={isRoot}
                    onAdd={onAdd}
                    onRemove={onRemove}
                    onFieldChange={onFieldChange}
                    onComposeTypeChange={onComposeTypeChange}
                    onOperatorChange={onOperatorChange}
                    onTimeframeChange={onTimeframeChange}
                    onValueChange={onValueChange}
                    onConditionChange={onConditionChange}
                />

            )}
        </Card>
    )
}


type GroupProps = {
    condition: And,
    isRoot: boolean
    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onFieldChange: (id: string, value: Field) => void;
    onComposeTypeChange: (id: string, value: ComposeType) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
    onConditionChange: (condition: Condition) => void;
}

const Group: FC<GroupProps> = ({
                                   condition,
                                   isRoot,
                                   onAdd,
                                   onRemove,
                                   onFieldChange,
                                   onComposeTypeChange,
                                   onOperatorChange,
                                   onTimeframeChange,
                                   onValueChange,
                                   onConditionChange
                               }) => {

    return (
        <div key={condition.id}>
            <span>And</span>

            <div className="ml-4 mt-2 border-l-2 pl-4 border-gray-400 space-y-2">
                {condition.conditions?.map((child, index) =>
                    <ConditionList
                        key={child.id}
                        condition={child}
                        isRoot={false}
                        onAdd={onAdd}
                        onRemove={onRemove}
                        onFieldChange={onFieldChange}
                        onComposeTypeChange={onComposeTypeChange}
                        onOperatorChange={onOperatorChange}
                        onTimeframeChange={onTimeframeChange}
                        onValueChange={onValueChange}
                        onConditionChange={onConditionChange}
                    />
                )}
                <Button variant="outline" onClick={() => onAdd(condition.id, ConditionType.COMPOSE)}>
                    + Condition
                </Button>
                <Button variant="outline" onClick={() => onAdd(condition.id, ConditionType.AND)}>
                    + Group
                </Button>
            </div>
        </div>
    )
}