import React, { useState } from "react";
import { Button } from "@components/ui/button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@components/ui/select";
import { Card } from "@components/ui/card";

const conditionTypes = ["Compare", "And", "Or"];
const eventOptions = ["Best performing coin on the market"];
const operatorOptions = ["Increase by", "Decrease by"];
const timeframeOptions = ["Current Price", "1m", "5m", "15m"];

type Condition = {
    type: "Compare" | "And" | "Or";
    event?: string;
    operator?: string;
    percentage?: number;
    timeframe?: string;
    nested?: Condition[];
};

const createCondition = (type: "Compare" | "And" | "Or"): Condition => ({
    type,
    ...(type === "Compare" ? { event: null, operator: null, percentage: 10, timeframe: null } : { nested: [] }),
});

export const RuleDetailPage: React.FC = () => {
    const [rootCondition, setRootCondition] = useState<Condition>({ type: "And", nested: [] });

    const updateCondition = (path: number[], key: keyof Condition, value: any) => {
        setRootCondition((prev) => updateNestedConditions({ ...prev }, path, (cond) => (cond[key] = value)));
    };

    const addCondition = (path: number[], type: "Compare" | "And") => {
        setRootCondition((prev) => updateNestedConditions({ ...prev }, path, (cond) => cond.nested?.push(createCondition(type))));
    };

    const removeCondition = (path: number[]) => {
        setRootCondition((prev) => updateNestedConditions({ ...prev }, path, (parent, index) => parent.nested?.splice(index, 1)));
    };

    const updateNestedConditions = (
        condition: Condition,
        path: number[],
        updateFn: (cond: Condition, index?: number, parent?: Condition) => void
    ) => {
        if (path.length === 0) {
            updateFn(condition);
            return condition;
        }
        let parent = condition;
        for (let i = 0; i < path.length - 1; i++) {
            parent = parent.nested![path[i]];
        }
        updateFn(parent.nested![path[path.length - 1]], path[path.length - 1], parent);
        return condition;
    };

    const renderConditions = (condition: Condition, path: number[] = []) => {
        return (
            <Card className="p-4 border bg-gray-50 mt-2 relative">
                {/* Remove Button - X in the Top-Right */}
                {path.length > 0 && (
                    <button
                        onClick={() => removeCondition(path)}
                        className="absolute top-2 right-2 text-gray-500 hover:text-red-500"
                    >
                        ✖
                    </button>
                )}

                <div className="flex flex-col space-y-2">
                    {/* Condition Type Selector */}
                    {path.length > 0 && (
                        <Select onValueChange={(value) => updateCondition(path, "type", value)}>
                            <SelectTrigger className="w-full">
                                <SelectValue placeholder={condition.type} />
                            </SelectTrigger>
                            <SelectContent>
                                {conditionTypes
                                    .filter((type) => (path.length === 0 ? type === "And" : true)) // Root must always be "And"
                                    .map((type) => (
                                        <SelectItem key={type} value={type}>
                                            {type}
                                        </SelectItem>
                                    ))}
                            </SelectContent>
                        </Select>
                    )}

                    {/* Compare Condition */}
                    {condition.type === "Compare" && (
                        <>
                            <Select onValueChange={(value) => updateCondition(path, "event", value)}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={condition.event || "Select Event"} />
                                </SelectTrigger>
                                <SelectContent>
                                    {eventOptions.map((ev) => (
                                        <SelectItem key={ev} value={ev}>
                                            {ev}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>

                            <Select onValueChange={(value) => updateCondition(path, "operator", value)}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={condition.operator || "Operator"} />
                                </SelectTrigger>
                                <SelectContent>
                                    {operatorOptions.map((op) => (
                                        <SelectItem key={op} value={op}>
                                            {op}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>

                            <input
                                type="number"
                                value={condition.percentage}
                                onChange={(e) => updateCondition(path, "percentage", parseInt(e.target.value))}
                                className="border p-2 w-full rounded"
                            />

                            <Select onValueChange={(value) => updateCondition(path, "timeframe", value)}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={condition.timeframe || "Select Timeframe"} />
                                </SelectTrigger>
                                <SelectContent>
                                    {timeframeOptions.map((time) => (
                                        <SelectItem key={time} value={time}>
                                            {time}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        </>
                    )}

                    {/* Nested Conditions for "And" and "Or" */}
                    {(condition.type === "And" || condition.type === "Or") && (
                        <div className="ml-4 mt-2 border-l-2 pl-4 border-gray-400 space-y-2">
                            {condition.nested?.map((nestedCondition, index) => renderConditions(nestedCondition, [...path, index]))}
                            <Button variant="outline" onClick={() => addCondition(path, "Compare")}>
                                + Condition
                            </Button>
                            <Button variant="outline" onClick={() => addCondition(path, "And")}>
                                + Group
                            </Button>
                        </div>
                    )}
                </div>
            </Card>
        );
    };

    return (
        <div className="max-w-4xl mx-auto space-y-6">
            {/* Query Builder */}
            <div className="border-l-4 border-blue-500 pl-4">
                <h3 className="font-semibold text-blue-600 flex items-center">
                    <span className="mr-2">⚡ EVENT</span>
                </h3>
                {renderConditions(rootCondition)}
            </div>

            {/* Execute Section */}
            <Button className="w-full bg-green-500 text-white">Launch Rule</Button>
        </div>
    );
};