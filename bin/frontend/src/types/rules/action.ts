import {ValueNumber} from "@types";

export enum ActionType {
    NOTIFY_TELEGRAM = 'NOTIFY_TELEGRAM'
}

export enum NotificationChannel {
    TELEGRAM = 'TELEGRAM'
}

export type TelegramButtonAction = 'NOTHING' | 'BUY' | 'SELL'

export type TelegramButtonConfig = {
    action: TelegramButtonAction;
    value?: ValueNumber;
}

export type Action = ActionNotifyTelegram;

export type ActionNotifyTelegram = {
    type: ActionType,
    buttons: Array<TelegramButtonConfig>,
}


