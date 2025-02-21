import {FC} from "react";
import {Action, ActionType} from "@types";
import {SelectActionType} from "@components/editor/action/type.tsx";
import {Notify} from "@components/editor/action/notify.tsx";

export type ActionEditorProps = {
    action: Action;
    onChange(action: Action): void;
}

export const ActionEditor: FC<ActionEditorProps> = ({action, onChange}) => {

    return (
        <div>
            <SelectActionType
                defaultActionType={ActionType.NOTIFY_TELEGRAM}
                supported={[
                    ActionType.NOTIFY_TELEGRAM
                ]}
                onChange={(type) => {
                }}
            />
            {/*{action.type === ActionType.NOTIFY_TELEGRAM && <Notify action={action} onChange={(action) => {*/}
            {/*    onChange(action)*/}
            {/*}}/>}*/}
        </div>
    );
}