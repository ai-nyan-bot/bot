import {Condition} from "./rules";
import {ValueNumber} from "./value.ts";

export type Sequence = {
    condition: Condition,
    action: Action
}

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

export enum Field {
    CURVE_PROGRESS = 'CURVE_PROGRESS',
    CURVE_PROGRESS_AGE = 'CURVE_PROGRESS_AGE',
    TRADES = 'TRADES',
    TRADES_BUY = 'TRADES_BUY',
    TRADES_SELL = 'TRADES_SELL',
}

export enum Operator {
    EQUAL = "EQUAL",
    NOT_EQUAL = "NOT_EQUAL",

    INCREASED_BY_MORE_THAN = 'INCREASED_BY_MORE_THAN',
    INCREASED_BY_MORE_THAN_EQUAL = 'INCREASED_BY_MORE_THAN_EQUAL',
    INCREASED_BY_LESS_THAN = 'INCREASED_BY_LESS_THAN',
    INCREASED_BY_LESS_THAN_EQUAL = 'INCREASED_BY_LESS_THAN_EQUAL',

    DECREASED_BY_MORE_THAN = 'DECREASED_BY_MORE_THAN',
    DECREASED_BY_MORE_THAN_EQUAL = 'DECREASED_BY_MORE_THAN_EQUAL',
    DECREASED_BY_LESS_THAN = 'DECREASED_BY_LESS_THAN',
    DECREASED_BY_LESS_THAN_EQUAL = 'DECREASED_BY_LESS_THAN_EQUAL',

    MORE_THAN = 'MORE_THAN',
    MORE_THAN_EQUAL = 'MORE_THAN_EQUAL',
    LESS_THAN = 'LESS_THAN',
    LESS_THAN_EQUAL = 'LESS_THAN_EQUAL',

}
