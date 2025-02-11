import React, {FC} from "react";
import Select from "react-select";


const eventOptions = [{
    value: "pumpfun",
    label: "Any coin on Pumpfun"
}];

export type EventComponentProps = {};

export const EventComponent: FC<EventComponentProps> = ({}) => {
    return (
        // <div className="space-y-2 mt-2 p-4">
        //     <div className="flex flex-row">
                <Select
                    options={eventOptions}
                    placeholder="Select"
                    className="w-full sm:w-auto"
                />
            // </div>
        // </div>
    );
}