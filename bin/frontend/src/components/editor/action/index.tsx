import {FC} from "react";
import {Action, ActionType} from "@types";
import {SelectActionType} from "@components/editor/action/type.tsx";
import {Notify} from "@components/editor/action/notify.tsx";

export type ActionEditorProps = {
    action: Action;
    onChange(action: Action): void;
}

export const ActionEditor: FC<ActionEditorProps> = ({action}) => {

    return (
        <div>
            <SelectActionType
                defaultActionType={ActionType.NOTIFY}
                supported={[
                    ActionType.NOTIFY
                ]}
                onChange={(type) => {
                    console.log("Changed action type")
                }}
            />
            {action.type === ActionType.NOTIFY && <Notify action={action} onChange={(action) => {
                console.log("updated action", action)
            }}/>}
        </div>
    );
}