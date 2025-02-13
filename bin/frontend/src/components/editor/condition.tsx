import {Card} from "@components/ui/card.tsx";
import {Condition, ConditionType, Field, Operator, Timeframe, Value} from "@types";
import React, {FC} from "react";
import {Button} from "@components/ui/button.tsx";
import {Compare} from "@components/editor/condition-compare.tsx";

export type ConditionListProps = {
    condition: Condition,
    isRoot: boolean

    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onFieldChange: (id: string, value: Field) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}


export const ConditionList: FC<ConditionListProps> = ({
                                                          condition,
                                                          isRoot,
                                                          onAdd,
                                                          onRemove,
                                                          onFieldChange,
                                                          onOperatorChange,
                                                          onTimeframeChange,
                                                          onValueChange
                                                      }) => {
    return (
        <Card className="p-4 border bg-gray-50 mt-2 relative">
            {
                !isRoot && (
                    <button
                        onClick={() => onRemove(condition.id)}
                        className="absolute top-2 right-2 text-gray-500 hover:text-red-500"
                    >
                        ✖
                    </button>
                )}

            {condition.type === 'COMPARE' && (
                <Compare
                    condition={condition}
                    onFieldChange={onFieldChange}
                    onValueChange={onValueChange}
                    onTimeframeChange={onTimeframeChange}
                    onOperatorChange={onOperatorChange}
                />
            )}

            {(condition.type === 'AND') && (
                <Group
                    condition={condition}
                    isRoot={isRoot}
                    onAdd={onAdd}
                    onRemove={onRemove}
                    onFieldChange={onFieldChange}
                    onOperatorChange={onOperatorChange}
                    onTimeframeChange={onTimeframeChange}
                    onValueChange={onValueChange}
                />

            )}
        </Card>
    )
}


type GroupProps = {
    condition: Condition,
    isRoot: boolean
    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onFieldChange: (id: string, value: Field) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}

const Group: FC<GroupProps> = ({
                                   condition,
                                   isRoot,
                                   onAdd,
                                   onRemove,
                                   onFieldChange,
                                   onOperatorChange,
                                   onTimeframeChange,
                                   onValueChange
                               }) => {

    return (
        <div key={condition.id}>
            <span>And</span>

            <div className="ml-4 mt-2 border-l-2 pl-4 border-gray-400 space-y-2">
                {condition.conditions?.map((child, index) =>
                    <ConditionList
                        key={child.id}
                        condition={child}
                        isRoot={isRoot}
                        onAdd={onAdd}
                        onRemove={onRemove}
                        onFieldChange={onFieldChange}
                        onOperatorChange={onOperatorChange}
                        onTimeframeChange={onTimeframeChange}
                        onValueChange={onValueChange}
                    />
                )}
                <Button variant="outline" onClick={() => onAdd(condition.id, 'COMPARE')}>
                    + Condition
                </Button>
                <Button variant="outline" onClick={() => onAdd(condition.id, 'AND')}>
                    + Group
                </Button>
            </div>
        </div>
    )
}