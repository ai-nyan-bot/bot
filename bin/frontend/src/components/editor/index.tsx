import React, {useState} from "react";
import {Button} from "@components/ui/button.tsx";
import {Condition, ConditionType} from "@types";
import {ConditionList} from "@components/editor/condition.tsx";
import {v4 as uuidv4} from "uuid";

const createCondition = (type: ConditionType): Condition => {
    switch (type) {
        case "And":
            return {
                id: uuidv4(),
                type: "And",
                conditions: []
            }
        case "Or":
            return {
                id: uuidv4(),
                type: "Or",
                conditions: []
            }
        case "Compare":
            return {
                id: uuidv4(),
                type: "Compare",
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
        } else if (cond.type === "And") {
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
            cond.type !== "Compare" ? {
                ...cond,
                conditions: filter(id, cond.conditions || [])
            } : cond
        );
};


export const Editor: React.FC = () => {
    const [rootCondition, setRootCondition] = useState<Condition>(createCondition("And"));

    const updateCondition = (id: string, key: keyof Condition, value: any) => {
        setRootCondition((prev) => ({
            ...prev,
            conditions: update(id, (cond) => ({
                ...cond,
                [key]: value
            }), prev.conditions),
        }));
    };

    const addCondition = (parentId: string, type: ConditionType) => {
        setRootCondition((prev) => {
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
        setRootCondition((prev) => ({
            ...prev,
            conditions: filter(id, prev.conditions),
        }));
    };

    return (
        <div className="max-w-4xl mx-auto space-y-6">
            <div className="border-l-4 border-yellow-500 pl-4">
                <h3 className="font-semibold text-yellow-600 flex items-center">
                    <span className="mr-2">IF</span>
                </h3>
                <ConditionList
                    condition={rootCondition}
                    isRoot={true}
                    onAdd={addCondition}
                    onRemove={removeCondition}
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

            {/* Execute Section */}
            <Button className="w-full bg-green-500 text-white" onClick={() => {
                console.log(JSON.stringify(rootCondition))

            }}>Launch Rule</Button>
        </div>
    );
}