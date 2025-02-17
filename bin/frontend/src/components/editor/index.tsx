import React, {useEffect, useState} from "react";
import {Action, Condition, ConditionType, Field, Operator, Sequence, Timeframe} from "@types";
import {ConditionList} from "@components/editor/condition.tsx";
import {uuidv4} from "@app/utils/id.ts";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card.tsx";

const createCondition = (type: ConditionType): Condition => {
    switch (type) {
        case 'AND':
            return {
                id: uuidv4(),
                type: 'AND',
                conditions: []
            }
        case'OR':
            return {
                id: uuidv4(),
                type: 'OR',
                conditions: []
            }
        case 'COMPARE':
            return {
                id: uuidv4(),
                type: 'COMPARE',
                field: Field.PRICE,
                operator: Operator.GREATER_THAN,
                value: {
                    type: 'PERCENT',
                    value: 15
                },
                timeframe: Timeframe.M15
            }
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

const filter = (id: string, conditions: Condition[] | undefined): Condition[] => {
    if (!conditions) return [];
    return conditions
        .filter((cond) => cond.id !== id)
        .map((cond) =>
            cond.type !== 'COMPARE' ? {
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

    const updateCondition = (id: string, key: keyof Condition, value: any) => {
        setCondition((prev) => ({
            ...prev,
            conditions: update(id, (cond) => ({
                ...cond,
                [key]: value
            }), prev.conditions),
        }));
    };

    const addCondition = (parentId: string, type: ConditionType) => {
        setCondition((prev) => {
            if (prev.id === parentId) {
                return {
                    ...prev,
                    conditions: [...(prev.conditions || []), createCondition(type)],
                };
            }
            return {
                ...prev,
                conditions: update(parentId, (parent) => ({
                    ...parent,
                    conditions: [...(parent.conditions || []), createCondition(type)]
                }), prev.conditions),
            };
        });
    };

    const removeCondition = (id: string) => {
        setCondition((prev) => ({
            ...prev,
            conditions: filter(id, prev.conditions),
        }));
    };

    useEffect(() => {
        if (condition) {
            if (onChange) {
                console.log(condition)
                onChange({action, condition})
            }
        }
    }, [condition]);


    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle className="font-semibold text-yellow-600 flex items-center">IF</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="max-w-4xl mx-auto space-y-6">
                    <div className="border-l-4 border-yellow-500 pl-4">
                        <ConditionList
                            condition={condition}
                            isRoot={true}
                            onAdd={addCondition}
                            onRemove={removeCondition}
                            onFieldChange={(id, value) => {
                                updateCondition(id, "field", value)
                            }}
                            onOperatorChange={(id, value) => {
                                updateCondition(id, "operator", value)
                            }}
                            onTimeframeChange={(id, value) => {
                                updateCondition(id, "timeframe", value)
                            }}
                            onValueChange={(id, value) => {
                                updateCondition(id, "value", value)
                            }}
                        />
                    </div>
                </div>
            </CardContent>
        </Card>

    );
}