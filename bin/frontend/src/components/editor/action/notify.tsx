import {FC} from "react";
import {ActionNotifyTelegram} from "@types";
import {TelegramButtons} from "@components/editor/action/telegram.tsx";

export type NotifyProps = {
    action: ActionNotifyTelegram;
    onChange: (action: ActionNotifyTelegram) => void;
};

export const Notify: FC<NotifyProps> = ({action, onChange}) => {
    return (
        <>
            {/*<p>{JSON.stringify(action)}</p>*/}
            <TelegramButtons action={action}/>
        </>
    )
}

