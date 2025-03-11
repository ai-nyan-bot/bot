import React, {FC} from "react";
import {Action, ActionType} from "@types";
import {SelectActionType} from "@components/editor/action/type.tsx";

export type ActionEditorProps = {
    action: Action;
    onChange(action: Action): void;
}

export const ActionEditor: FC<ActionEditorProps> = ({action, onChange}) => {
    return (
        <div className={"flex flex-col border-l-4 border-blue-600"}>
            <div className={"px-4 flex flex-row"}>
                <span className="pr-10 font-semibold text-blue-600 flex items-center">THEN</span>
                <SelectActionType
                    defaultActionType={ActionType.NOTIFY_TELEGRAM}
                    supported={[
                        ActionType.NOTIFY_TELEGRAM
                    ]}
                    onChange={(type) => {
                    }}
                />
            </div>

            <div className={"pt-4"}>
            </div>
        </div>
    );
}