import React, {useEffect, useState} from "react";
import {Action, ComposeType, Condition, ConditionType, Field, Operator, Sequence, TimeUnit, ValueType} from "@types";
import {ConditionList} from "./condition";
import {uuidv4} from "@app/utils/id.ts";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card.tsx";
import {ActionEditor} from "./action";

const createCondition = (type: ConditionType): Condition => {
    switch (type) {
        case 'AND':
            return {
                id: uuidv4(),
                type: ConditionType.AND,
                conditions: []
            }
        // case'OR':
        //     return {
        //         id: uuidv4(),
        //         type: 'OR',
        //         conditions: []
        //     }
        case 'COMPOSE':
            return {
                id: uuidv4(),
                type: ConditionType.COMPOSE,
                ty: ComposeType.CURVE_PROGRESS,
                condition: {
                    id: uuidv4(),
                    type: ConditionType.AND,
                    conditions: [
                        {
                            id: uuidv4(),
                            type: ConditionType.COMPARE,
                            field: Field.CURVE_PROGRESS,
                            operator: Operator.MORE_THAN,
                            value: {
                                type: ValueType.PERCENT,
                                value: 0
                            },
                            timeframe: undefined
                        },
                        {
                            id: uuidv4(),
                            type: ConditionType.COMPARE,
                            field: Field.CURVE_PROGRESS,
                            operator: Operator.MORE_THAN,
                            value: {
                                type: ValueType.PERCENT,
                                value: 95
                            },
                            timeframe: undefined
                        },
                        {
                            id: uuidv4(),
                            type: ConditionType.COMPARE,
                            field: Field.CURVE_PROGRESS_AGE,
                            operator: Operator.LESS_THAN,
                            value: {
                                type: ValueType.DURATION,
                                value: 1,
                                unit: TimeUnit.MINUTE
                            }
                        }
                    ]
                }
            }
        default:
            throw new Error(`type ${type} not supported`)
    }
}

const update = (
    id: string,
    updateFunc: (condition: Condition) => Condition,
    conditions: Condition[] | undefined
): Condition[] => {
    if (!conditions) return [];
    return conditions.map((cond) => {
        if (cond.id === id) {
            return updateFunc(cond);
        } else if (cond.type === 'AND') {
            return {...cond, conditions: update(id, updateFunc, cond.conditions || [])};
        }
        return cond;
    });
};

const findAndUpdateCondition = (id: string, condition: Condition, conditions: Condition[] | undefined): Condition[] => {
    if (!conditions) return [];
    return conditions.map((cond) => {
        if (cond.id === id) {
            return condition;
        } else if (cond.type === 'AND') {
            return {...cond, conditions: findAndUpdateCondition(id, condition, cond.conditions || [])};
        }
        return cond;
    });
};

const filter = (id: string, conditions: Condition[] | undefined): Condition[] => {
    if (!conditions) return [];
    return conditions
        .filter((cond) => cond.id !== id)
        .map((cond) =>
            cond.type === ConditionType.AND ? {
                ...cond,
                conditions: filter(id, cond.conditions || [])
            } : cond
        );
};

export type EditorProps = {
    sequence: Sequence,
    onChange?: (sequence: Sequence) => void,
};

export const Editor: React.FC<EditorProps> = ({sequence, onChange}) => {
    const [action, setAction] = useState<Action>(sequence.action);
    const [condition, setCondition] = useState<Condition>(sequence.condition);

    const updateCondition = (condition: Condition) => {
        setCondition((prev) => {
            if (prev.type === ConditionType.AND) {
                return {
                    ...prev,
                    conditions: findAndUpdateCondition(condition.id, condition, prev.conditions),
                }
            } else {
                throw new Error("Not a group");
            }
        });
    };

    const addCondition = (parentId: string, type: ConditionType) => {
        setCondition((prev) => {
            if (prev.type === ConditionType.AND) {
                if (prev.id === parentId) {
                    return {
                        ...prev,
                        conditions: [...(prev.conditions || []), createCondition(type)],
                    };
                }
                return {
                    ...prev,
                    conditions: update(parentId, (parent) => {
                            if (parent.type === ConditionType.AND) {
                                return {
                                    ...parent,
                                    conditions: [...(parent.conditions || []), createCondition(type)]
                                }
                            } else {
                                throw new Error("Not a group");
                            }
                        },
                        prev.conditions
                    ),
                };
            } else {
                throw new Error("Not a group");
            }
        });
    };

    const removeCondition = (id: string) => {
        setCondition((prev) => {
            if (prev.type === ConditionType.AND) {
                return {
                    ...prev,
                    conditions: filter(id, prev.conditions),
                }
            } else {
                throw new Error("Not a group")
            }
        })
    };

    useEffect(() => {
        const handler = setTimeout(() => {
            if (action && condition) {
                if (onChange) {
                    onChange({action, condition})
                }
            }
        }, 100);
        return () => clearTimeout(handler);
    }, [action, condition]);

    return (
        <div className={"flex flex-col space-y-2"}>
            <Card className="w-full">
                <CardHeader>
                    <CardTitle className="font-semibold text-yellow-600 flex items-center">IF</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="max-w-4xl mx-auto space-y-6">
                        <ConditionList
                            condition={condition}
                            isRoot={true}
                            onAdd={addCondition}
                            onRemove={removeCondition}
                            onConditionChange={(condition: Condition) => {
                                updateCondition(condition)
                            }}
                        />
                    </div>
                </CardContent>
            </Card>
            <Card className="w-full">
                <CardHeader>
                    <CardTitle className="font-semibold text-blue-600 flex items-center">THEN</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="max-w-4xl mx-auto space-y-6">
                        <div className="border-l-4 border-blue-500 pl-4">
                            <ActionEditor
                                action={action}
                                onChange={setAction}
                            />
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    );
}