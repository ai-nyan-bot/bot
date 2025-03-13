// import React, {FC, useEffect, useState} from "react";
// import {ComposedSimpleMarketCap, ValueCount, ValueNumber, ValueType} from "@types";
// import {ValueNumberInput} from "@components/editor/value";
//
// export type SimpleMarketCapComposeProps = {
//     condition: ComposedSimpleMarketCap;
//     onChange: (condition: ComposedSimpleMarketCap) => void;
// };
//
// export const SimpleMarketCapCompose: FC<SimpleMarketCapComposeProps> = ({condition, onChange}) => {
//     const min = condition.condition.conditions[0];
//     const max = condition.condition.conditions[1];
//
//     const [minValue, setMinValue] = useState<ValueNumber | undefined>(min.value);
//     const [maxValue, setMaxValue] = useState<ValueNumber | undefined>(max.value);
//
//     useEffect(() => {
//         if (min.value !== minValue || max.value !== maxValue) {
//             onChange({
//                 ...condition,
//                 condition: {
//                     ...condition.condition,
//                     conditions: [
//                         {...min, value: minValue},
//                         {...max, value: maxValue}
//                     ]
//                 }
//             });
//         }
//     }, [minValue, maxValue]);
//
//     return (
//         <div className={"flex flex-row"}>
//             <div className={"flex flex-col"}>
//                 <div className={"flex flex-row space-x-4"}>
//                     <span className={"flex items-center text-zinc-500"}>Min</span>
//                     <ValueNumberInput
//                         value={minValue}
//                         // onChange={(value) => setMinValue(_ => {
//                         //     if (!value || isNaN(value.value)) {
//                         //         return undefined;
//                         //     }
//                         //     return value as ValueCount
//                         // })}
//                         supported={[ValueType.SOL]}
//                         minValue={1}
//                         placeholder={`min market cap`}
//                         hideValueSelect
//                     />
//                 </div>
//
//                 <div className={"pt-4 flex flex-row space-x-4"}>
//                     <span className={"flex items-center text-zinc-500"}>Max</span>
//                     <ValueNumberInput
//                         value={maxValue}
//                         // onChange={(value) => setMinValue(_ => {
//                         //     if (!value || isNaN(value.value)) {
//                         //         return undefined;
//                         //     }
//                         //     return value as ValueCount
//                         // })}
//                         supported={[ValueType.SOL]}
//                         minValue={1}
//                         placeholder={`max market cap`}
//                         hideValueSelect
//                     />
//                 </div>
//
//                 {/*<RenderText*/}
//                 {/*    minValue={minValue}*/}
//                 {/*    maxValue={maxValue}*/}
//                 {/*/>*/}
//             </div>
//         </div>
//     )
// }
