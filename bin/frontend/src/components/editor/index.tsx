import React, {useEffect, useState} from "react";
import {Action, Condition, ConditionType, DEFAULT_CONDITION, Sequence} from "@types";
import {uuidv4} from "@app/utils/id.ts";
import {ActionEditor} from "./action";
import {ConditionEditor} from "./condition";

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
            return {...DEFAULT_CONDITION} as unknown as Condition;
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
            let conditions: Condition[] = [];
            if (prev.type === ConditionType.COMPOSE) {
                if (prev.condition.type === ConditionType.AND) {
                    conditions = prev.condition.conditions;
                }
            } else if (prev.type === ConditionType.AND) {
                conditions = prev.conditions;
            } else {
                throw new Error("Neither compose nor group");
            }

            return {
                ...prev,
                conditions: findAndUpdateCondition(condition.id, condition, conditions),
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
            <ConditionEditor condition={sequence.condition} onChange={(condition) => {
                updateCondition(condition);
            }}/>
            <ActionEditor action={sequence.action} onChange={(_) => {
            }}/>
        </div>
    );
}