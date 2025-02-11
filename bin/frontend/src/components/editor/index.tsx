import React, {useState} from "react";
import {Card} from "@components/ui/card.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";
import {Button} from "@components/ui/button.tsx";
import {Condition, ConditionType, Timeframe, ValuePercent} from "@types";
import {SelectTimeframe} from "@components/editor/timeframe.tsx";

const conditionTypes = ["Compare", "And", "Or"];


const fieldOptions: Array<{ value: string, label: string }> = [
    {value: 'Price', label: 'Price'}
];

const operatorOptions: Array<{ value: string, label: string }> = [
    {value: 'GreaterThan', label: 'greater than'},
    {value: 'IncreaseBy', label: 'increase by'}
];

// const timeframeOptions: Array<{ value: string, label: string }> = [
//     {value: "M1", label: "1 minute"},
//     {value: "M5", label: "5 minutes"},
//     {value: "M15", label: "15 minutes"},
// ];


const createCondition = (type: ConditionType): Condition => {
    switch (type) {
        case "And":
            return {
                type: "And",
                conditions: []
            }
        case "Or":
            return {
                type: "Or",
                conditions: []
            }
        case "Compare":
            return {
                type: "Compare",
            }
    }
}

export const Editor: React.FC = () => {
    const [rootCondition, setRootCondition] = useState<Condition>({type: "And", conditions: []});

    const updateCondition = (path: number[], key: keyof Condition, value: any) => {
        setRootCondition((prev) => updateNestedConditions({...prev}, path, (cond) => (cond[key] = value)));
    };

    const addCondition = (path: number[], type: ConditionType) => {
        setRootCondition((prev) => updateNestedConditions({...prev}, path, (cond) => cond.conditions?.push(createCondition(type))));
    };

    const removeCondition = (path: number[]) => {
        // @ts-ignore
        setRootCondition((prev) => updateNestedConditions({...prev}, path, (parent, index) => parent.conditions?.splice(index, 1)));
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
            parent = parent.conditions![path[i]];
        }
        updateFn(parent.conditions![path[path.length - 1]], path[path.length - 1], parent);
        return condition;
    };

    // const timeframeOptions = useTimeframeOptions([
    //     Timeframe.M1,
    //     Timeframe.M5,
    // ]);
    //
    // console.log(timeframeOptions);

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
                    {/*{path.length > 0 && (*/}

                    {/*)}*/}

                    {/* Compare Condition */}
                    {condition.type !== "Compare" && (
                        <>
                            <Select onValueChange={(value) => updateCondition(path, "type", value)}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={condition.type}/>
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
                        </>
                    )}
                    {condition.type === "Compare" && (
                        <>
                            <Select onValueChange={(value) => updateCondition(path, "field", value)}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={condition.field || "What?"}/>
                                </SelectTrigger>
                                <SelectContent>
                                    {fieldOptions.map(({value, label}) => (
                                        <SelectItem key={value} value={value}>
                                            {label}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>

                            <Select onValueChange={(value) => updateCondition(path, "operator", value)}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder={condition.operator || "How?"}/>
                                </SelectTrigger>
                                <SelectContent>
                                    {operatorOptions.map(({value, label}) => (
                                        <SelectItem key={value} value={value}>
                                            {label}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>

                            <input
                                type="number"
                                value={((condition?.value || {type: "Percent", value: 0}) as ValuePercent)?.value}
                                onChange={(e) => updateCondition(path, "value", {
                                    type: "Percent",
                                    value: parseFloat(e.target.value)
                                })}
                                className="border p-2 w-full rounded"
                            />

                            {/*<Select defaultValue={Timeframe.M5}*/}
                            {/*        onValueChange={(value) => updateCondition(path, "timeframe", value)}>*/}
                            {/*    <SelectTrigger className="w-full">*/}
                            {/*        <SelectValue/>*/}
                            {/*    </SelectTrigger>*/}
                            {/*    <SelectContent>*/}
                            {/*        {timeframeOptions.map(({value, label}) => (*/}
                            {/*            <SelectItem key={value} value={value}>*/}
                            {/*                {label}*/}
                            {/*            </SelectItem>*/}
                            {/*        ))}*/}
                            {/*    </SelectContent>*/}
                            {/*</Select>*/}

                            <SelectTimeframe
                                supported={[
                                    Timeframe.M1,
                                    Timeframe.M5,
                                    Timeframe.M15
                                ]}
                                onChange={(value) => updateCondition(path, "timeframe", value)}
                            >
                            </SelectTimeframe>
                        </>
                    )}

                    {(condition.type === "And") && (
                        <div className="ml-4 mt-2 border-l-2 pl-4 border-gray-400 space-y-2">
                            {condition.conditions?.map((nestedCondition, index) => renderConditions(nestedCondition, [...path, index]))}
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
            <Button className="w-full bg-green-500 text-white" onClick={() => {
                console.log(JSON.stringify(rootCondition))

            }}>Launch Rule</Button>
        </div>
    );
}