import {FC} from "react";
import {Condition} from "@types";
import Select from "react-select";

const eventOptions = [{
    value: "pumpfun",
    label: "Any coin on Pumpfun"
}];


const groupOptions = [
    {value: "and", label: "And"},
    {value: "or", label: "Or"}
];


const conditionOptions = [{
    value: "price",
    label: "Price"
}];


const operatorOptions = [{
    value: ">",
    label: "increase by"
}];


const numberOptions = [
    {
        value: "percent",
        label: "%"
    },
    {
        value: "usd",
        label: "USD"
    },
    {
        value: "sol",
        label: "SOL"
    }
];

const timeframeOptions = [
    {value: "M1", label: "1 minute"},
    {value: "M5", label: "5 minutes"},
    {value: "M15", label: "15 minutes"},
];


export type ConditionComponentProps = {
    condition: Condition
};

// export const ConditionComponent: FC<ConditionComponentProps> = ({condition}) => {
//     return (
//         <div className="space-y-2 mt-2 p-2 border rounded bg-gray-50">
//             <div className="flex items-center space-x-2">
//                 {/*<Select*/}
//                 {/*    options={eventOptions}*/}
//                 {/*    // value={condition.event}*/}
//                 {/*    // onChange={(selected) => updateCondition(currentPath, "event", selected)}*/}
//                 {/*    placeholder="Select"*/}
//                 {/*/>*/}
//                 {/*<span>has</span>*/}
//                 <Select
//                     options={conditionOptions}
//                     // value={condition.condition}
//                     // onChange={(selected) => updateCondition(currentPath, "condition", selected)}
//                     placeholder="Condition"
//
//                 />
//                 <Select
//                     options={operatorOptions}
//                     // value={condition.comparison}
//                     // onChange={(selected) => updateCondition(currentPath, "comparison", selected)}
//                     placeholder="Operator"
//                 />
//                 <input
//                     type="number"
//                     // value={condition.percentage}
//                     // onChange={(e) => updateCondition(currentPath, "percentage", e.target.value)}
//                     className="border p-2 w-16 text-center rounded"
//                 />
//                 <Select
//                     options={numberOptions}
//                     defaultValue={numberOptions[0]}
//                     // value={condition.comparison}
//                     // onChange={(selected) => updateCondition(currentPath, "comparison", selected)}
//                 />
//                 {/*<span>with in</span>*/}
//                 <Select
//                     options={timeframeOptions}
//                     defaultValue={timeframeOptions[0]}
//                     // value={condition.priceRef}
//                     // onChange={(selected) => updateCondition(currentPath, "priceRef", selected)}
//                     placeholder="time frame"
//                 />
//
//             </div>
//
//             {/* Render nested conditions */}
//             {/*<div className="ml-10 border-l-2 border-gray-400 pl-4">*/}
//             {/*    {renderConditions(condition.nested, currentPath)}*/}
//             {/*</div>*/}
//         </div>
//     )
// }

export const ConditionComponent: FC<ConditionComponentProps> = ({condition}) => {
    return (
        <div className="space-y-2 mt-2 ">
            {/*<div className="flex flex-col sm:flex-row sm:items-center sm:space-x-2 space-y-2 sm:space-y-0">*/}
            {/*    <span className="text-gray-600">Event</span>*/}
            {/*    <Select*/}
            {/*        options={eventOptions}*/}
            {/*        placeholder="Select"*/}
            {/*        className="w-full sm:w-auto"*/}
            {/*    />*/}
            {/*</div>*/}

            <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-2 space-y-2 sm:space-y-0">
                {/*<span className="text-gray-600">Has</span>*/}
                <Select
                    options={conditionOptions}
                    placeholder="Condition"
                    className="w-full sm:w-auto"
                />
            </div>

            <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-2 space-y-2 sm:space-y-0">
                <Select
                    options={operatorOptions}
                    placeholder="Operator"
                    className="w-full sm:w-auto"
                />
            </div>

            <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-2 space-y-2 sm:space-y-0">
                <div className={"flex flex-row"}>
                    <input
                        type="number"
                        // value={condition.percentage}
                        // onChange={(e) => updateCondition(currentPath, "percentage", e.target.value)}
                        className="border p-2 w-48 text-center rounded"
                    />
                    <Select
                        options={numberOptions}
                        defaultValue={numberOptions[0]}
                        // value={condition.comparison}
                        // onChange={(selected) => updateCondition(currentPath, "comparison", selected)}
                    />
                </div>
            </div>

            <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-2 space-y-2 sm:space-y-0">
                <span className="text-gray-600">Within</span>
                <Select
                    options={timeframeOptions}
                    defaultValue={timeframeOptions[0]}
                    placeholder="Time Frame"
                    className="w-full sm:w-auto"
                />
            </div>
        </div>
    )
}

