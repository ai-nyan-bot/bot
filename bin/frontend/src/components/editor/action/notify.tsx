import {FC} from "react";
import {ActionNotifyTelegram, ActionType} from "@types";
import {TelegramButtons} from "@components/editor/action/telegram.tsx";

export type NotifyProps = {
    action: ActionNotifyTelegram;
    onChange: (action: ActionNotifyTelegram) => void;
};

export const Notify: FC<NotifyProps> = ({action, onChange}) => {
    return (
        <>
            <TelegramButtons
                action={action}
                onChange={(buttons) => {
                    onChange({
                        type: ActionType.NOTIFY_TELEGRAM,
                        buttons

                    })
                }}
            />
        </>
    )
}

