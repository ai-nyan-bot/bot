import React, {useState} from "react";
import {ConditionComponent} from "@components/editor/condition.tsx";
import {ActionComponent} from "@components/editor/action.tsx";
import {EventComponent} from "@components/editor/event.tsx";
import {ChevronDownIcon, ChevronRightIcon} from "@heroicons/react/20/solid";

const optionsEvent = [{value: "best_coin", label: "Best performing coin on the market"}];
const optionsCondition = [{value: "price_open", label: "Price Open"}];
const optionsComparison = [
    {value: "increase", label: "Increase by"},
    {value: "decrease", label: "Decrease by"},
];
const optionsPriceRef = [{value: "current_price", label: "Current Price"}];
const optionsLogic = [
    {value: "AND", label: "AND"},
    {value: "OR", label: "OR"},
];

// Default condition structure
const createCondition = () => ({
    event: null,
    condition: null,
    comparison: null,
    percentage: 10,
    priceRef: null,
    logic: "AND",
    nested: [],
});


const AccordionItem = ({title, children}) => {
    const [isOpen, setIsOpen] = useState(false);

    return (
        <div className="border-b border-gray-300">
            <button
                onClick={() => setIsOpen(!isOpen)}
                className="w-full flex justify-between items-center transition"
            >
                {isOpen ? (<ChevronDownIcon className={`h-8 w-8`}/>) : (<ChevronRightIcon className={`h-8 w-8`}/>)}
                {title}
            </button>
            <div
                className={`overflow-hidden transition-max-h duration-300 ease-in-out ${isOpen ? "max-h-96" : "max-h-0"}`}
            >
                <div className="p-4 bg-white">{children}</div>
            </div>
        </div>
    );
};


export const Editor = () => {
    const [conditions, setConditions] = useState([createCondition()]);
    const [executionTime, setExecutionTime] = useState("immediately");
    const [executionCount, setExecutionCount] = useState(1);
    const [ruleName, setRuleName] = useState("");

    // Add a new nested condition at a specific path
    const addCondition = (path) => {
        setConditions(updateConditions([...conditions], path, (cond) => cond.nested.push(createCondition())));
    };

    // Remove a condition at a specific path
    const removeCondition = (path) => {
        setConditions(updateConditions([...conditions], path, (parent, index) => parent.nested.splice(index, 1)));
    };

    // Update a condition value at a specific path
    const updateCondition = (path, key, value) => {
        setConditions(updateConditions([...conditions], path, (cond) => (cond[key] = value)));
    };

    // Recursive function to update a condition by path
    const updateConditions = (conditions, path, updateFn) => {
        if (path.length === 0) {
            updateFn(conditions);
            return conditions;
        }
        let parent = conditions;
        for (let i = 0; i < path.length - 1; i++) {
            parent = parent[path[i]].nested;
        }
        updateFn(parent[path[path.length - 1]], path[path.length - 1]);
        return conditions;
    };

    // Render nested conditions recursively
    // const renderConditions = (conditions, path = []) =>
    //     conditions.map((condition, index) => {
    //         const currentPath = [...path, index];
    //
    //         return (
    //             <div key={index} className="space-y-2 mt-2 p-2 border rounded bg-gray-50">
    //                 <div className="flex items-center space-x-2">
    //                     {path.length > 0 && (
    //                         <Select
    //                             options={optionsLogic}
    //                             value={optionsLogic.find((o) => o.value === condition.logic)}
    //                             onChange={(selected) => updateCondition(currentPath, "logic", selected.value)}
    //                             className="w-20"
    //                         />
    //                     )}
    //                     <Select
    //                         options={optionsEvent}
    //                         value={condition.event}
    //                         onChange={(selected) => updateCondition(currentPath, "event", selected)}
    //                         placeholder="Select event"
    //                     />
    //                     <span>has</span>
    //                     <Select
    //                         options={optionsCondition}
    //                         value={condition.condition}
    //                         onChange={(selected) => updateCondition(currentPath, "condition", selected)}
    //                         placeholder="Condition"
    //                     />
    //                     <Select
    //                         options={optionsComparison}
    //                         value={condition.comparison}
    //                         onChange={(selected) => updateCondition(currentPath, "comparison", selected)}
    //                         placeholder="Comparison"
    //                     />
    //                     <input
    //                         type="number"
    //                         value={condition.percentage}
    //                         onChange={(e) => updateCondition(currentPath, "percentage", e.target.value)}
    //                         className="border p-2 w-16 text-center rounded"
    //                     />
    //                     <span>%</span>
    //                     <Select
    //                         options={optionsPriceRef}
    //                         value={condition.priceRef}
    //                         onChange={(selected) => updateCondition(currentPath, "priceRef", selected)}
    //                         placeholder="Reference Price"
    //                     />
    //                     <button className="text-green-500 hover:text-green-700"
    //                             onClick={() => addCondition(currentPath)}>
    //                         {/*<PlusCircle size={20}/>*/}
    //                         Add
    //                     </button>
    //                     {path.length > 0 && (
    //                         <button className="text-red-500 hover:text-red-700"
    //                                 onClick={() => removeCondition(currentPath)}>
    //                             {/*<Trash size={20}/>*/}
    //                             Remove
    //                         </button>
    //                     )}
    //                 </div>
    //
    //                 {/* Render nested conditions */}
    //                 <div className="ml-10 border-l-2 border-gray-400 pl-4">
    //                     {renderConditions(condition.nested, currentPath)}
    //                 </div>
    //             </div>
    //         );
    //     });

    return (
        <div className="max-w-4xl mx-auto p-6 bg-white shadow-lg rounded-lg space-y-6">
            <div className="border-l-4 border-yellow-500 pl-4">
                <h3 className="font-semibold text-yellow-600 flex items-center">
                    <span className="mr-2">IF</span>
                </h3>
                <EventComponent/>
                {/*<h3 className="font-semibold text-yellow-600 flex items-center">*/}
                {/*    <span className="mr-2">IF</span>*/}
                {/*</h3>*/}

                <div className="w-full max-w-md mx-auto">
                    {/*<AccordionItem title="price increases by 14% within 1 minute">*/}
                        <ConditionComponent condition={
                            {
                                type: 'Compare',
                                fact: undefined,
                                operator: undefined,
                                value: undefined,
                                timeframe: undefined
                            }
                        }/>
                    And
                    <ConditionComponent condition={
                        {
                            type: 'Compare',
                            fact: undefined,
                            operator: undefined,
                            value: undefined,
                            timeframe: undefined
                        }
                    }/>
                    {/*</AccordionItem>*/}
                    {/*<AccordionItem title="and market cap is greater than 500K USD">*/}
                    {/*    <ConditionComponent condition={*/}
                    {/*        {*/}
                    {/*            type: 'Compare',*/}
                    {/*            fact: undefined,*/}
                    {/*            operator: undefined,*/}
                    {/*            value: undefined,*/}
                    {/*            timeframe: undefined*/}
                    {/*        }*/}
                    {/*    }/>*/}
                    {/*</AccordionItem>*/}

                    {/*<AccordionItem title="Section 2">This is the content for section 2.</AccordionItem>*/}
                    {/*<AccordionItem title="Section 3">This is the content for section 3.</AccordionItem>*/}
                </div>

                <div className="flex justify-between">
                    <button className="bg-red-500 text-white px-4 py-2 rounded">+</button>
                </div>

            </div>

            {/*<ActionComponent action={'Notify'}/>*/}

            {/*<div className="flex justify-between">*/}
            {/*    <button className="bg-red-500 text-white px-4 py-2 rounded">Save</button>*/}
            {/*</div>*/}

            <div className="flex right-0">
                <button className="bg-red-500 text-white px-4 py-2 rounded">Then</button>
            </div>
        </div>
    );
};