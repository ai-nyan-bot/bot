import {And, ComposeType, Condition, ConditionType} from "@types";
import React, {FC, useState} from "react";
import {Card} from "@components/ui/card.tsx";
import {Button} from "@components/ui/button.tsx";

export type ConditionListProps = {
    condition: Condition,
    isRoot: boolean

    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onConditionChange: (condition: Condition) => void;

}


export const ConditionList: FC<ConditionListProps> = ({
                                                          condition,
                                                          isRoot,
                                                          onAdd,
                                                          onRemove,
                                                          onConditionChange
                                                      }) => {

    const [composeType, setComposeType] = useState<ComposeType>(ComposeType.PUMP_FUN_QUICK)


    return (
        <Card className={`p-1 rounded-none ${!isRoot ? "p-4 border-4" : ""} bg-gray-50 relative shadow-none`}>


            {/*{condition.type === 'COMPOSE' && (*/}
            {/*    <ComposeWidget*/}
            {/*        condition={condition}*/}
            {/*        onRemove={onRemove}*/}
            {/*        onConditionChange={onConditionChange}*/}
            {/*    />*/}
            {/*)}*/}

            {/*{(condition.type === 'AND') && (*/}
            {/*    <Group*/}
            {/*        condition={condition}*/}
            {/*        isRoot={isRoot}*/}
            {/*        onAdd={onAdd}*/}
            {/*        onRemove={onRemove}*/}
            {/*        onConditionChange={onConditionChange}*/}
            {/*    />*/}

            {/*)}*/}
        </Card>
    )
}


type GroupProps = {
    condition: And,
    isRoot: boolean
    onAdd: (parentId: string, type: ConditionType) => void;
    onRemove: (id: string) => void;
    onConditionChange: (condition: Condition) => void;
}

const Group: FC<GroupProps> = ({
                                   condition,
                                   isRoot,
                                   onAdd,
                                   onRemove,
                                   onConditionChange
                               }) => {

    return (
        <div key={condition.id}>
            <span className={"text-yellow-600 font-bold"}>And</span>

            <div className="ml-4 mt-2 border-l-2 pl-4 border-yellow-600 space-y-2">
                {condition.conditions?.map((child, index) =>
                    <ConditionList
                        key={child.id}
                        condition={child}
                        isRoot={false}
                        onAdd={onAdd}
                        onRemove={onRemove}
                        onConditionChange={onConditionChange}
                    />
                )}
                <Button variant="outline" onClick={() => onAdd(condition.id, ConditionType.COMPOSE)}>
                    + Condition
                </Button>
                {/*<Button variant="outline" onClick={() => onAdd(condition.id, ConditionType.AND)}>*/}
                {/*    + And*/}
                {/*</Button>*/}
            </div>
        </div>
    )
}