import {Card} from "@components/ui/card.tsx";
import {Condition, ConditionType, Operator, Timeframe, Value} from "@types";
import React, {FC} from "react";
import {Button} from "@components/ui/button.tsx";
import {SelectOperator} from "@components/editor/operator.tsx";
import {ValuePercentInput} from "@components/editor/value.tsx";
import {SelectTimeframe} from "@components/editor/timeframe.tsx";

export type ConditionListProps = {
    condition: Condition,
    isRoot: boolean

    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}


export const ConditionList: FC<ConditionListProps> = ({
                                                          condition,
                                                          isRoot,
                                                          onAdd,
                                                          onRemove,
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

            {condition.type === "Compare" && (
                <Compare
                    condition={condition}
                    onValueChange={onValueChange}
                    onTimeframeChange={onTimeframeChange}
                    onOperatorChange={onOperatorChange}
                />
            )}

            {(condition.type === "And") && (
                <Group condition={condition} isRoot={isRoot} onAdd={onAdd} onRemove={onRemove}
                       onOperatorChange={onOperatorChange} onTimeframeChange={onTimeframeChange}
                       onValueChange={onValueChange}/>

            )}
        </Card>
    )
}

type CompareProps = {
    condition: Condition,
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}

const Compare: FC<CompareProps> = ({condition, onOperatorChange, onTimeframeChange, onValueChange}) => {
    return (
        <div key={condition.id}>
            {/*<Select onValueChange={(value) => updateCondition(path, "field", value)}>*/}
            {/*    <SelectTrigger className="w-full">*/}
            {/*        <SelectValue placeholder={condition.field || "What?"}/>*/}
            {/*    </SelectTrigger>*/}
            {/*    <SelectContent>*/}
            {/*        {fieldOptions.map(({value, label}) => (*/}
            {/*            <SelectItem key={value} value={value}>*/}
            {/*                {label}*/}
            {/*            </SelectItem>*/}
            {/*        ))}*/}
            {/*    </SelectContent>*/}
            {/*</Select>*/}

            <SelectOperator
                supported={[
                    Operator.INCREASED_BY,
                    Operator.GREATER_THAN
                ]}
                onChange={(value) => onOperatorChange(condition.id, value)}
            />

            <ValuePercentInput
                onChange={(value) => onValueChange(condition.id, value)}
            />

            <SelectTimeframe
                supported={[
                    Timeframe.M1,
                    Timeframe.M5,
                    Timeframe.M15
                ]}
                onChange={(value) => onTimeframeChange(condition.id, value)}
            >
            </SelectTimeframe>
        </div>
    )
}


type GroupProps = {
    condition: Condition,
    isRoot: boolean
    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onOperatorChange: (id: string, value: Operator) => void;
    onTimeframeChange: (id: string, value: Timeframe) => void;
    onValueChange: (id: string, value: Value) => void;
}

const Group: FC<GroupProps> = ({
                                   condition,
                                   isRoot,
                                   onAdd,
                                   onRemove,
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
                        onOperatorChange={onOperatorChange}
                        onTimeframeChange={onTimeframeChange}
                        onValueChange={onValueChange}
                    />
                )}
                <Button variant="outline" onClick={() => onAdd(condition.id, "Compare")}>
                    + Condition
                </Button>
                <Button variant="outline" onClick={() => onAdd(condition.id, "And")}>
                    + Group
                </Button>
            </div>
        </div>
    )
}