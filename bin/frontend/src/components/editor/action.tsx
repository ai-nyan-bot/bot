import {Action} from "@types";
import {FC} from "react";

export type ActionComponentProps = {
    action: Action;
};

export const ActionComponent: FC<ActionComponentProps> = ({}) => {
    return (
        <div className="border-l-4 border-blue-500 pl-4">
            <h3 className="font-semibold text-blue-600"> THEN</h3>
            <div className="flex items-center space-x-2 mt-2">
                <span>notify me</span>
            </div>
        </div>
    );
}