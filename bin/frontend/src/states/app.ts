import {LocalStorage} from "@states/local";
import {AppType, AuthType, PageType, TelegramWebApp} from "@types";

export type AppTheme = "LIGHT" | "DARK";

export type AuthState = {
    type: AuthType;
    user: {
        id: string;
    };
    token: string;
    telegram?: {
        id: string;
    }
}

export type ConnectionStatus = "CONNECTED" | "DISCONNECTED";

export type ViewportState = {
    width: number;
    height: number;
}

export type AppState = {
    type: AppType;
    theme: AppTheme;
    telegramData?: TelegramWebApp;
    page: PageType;
    auth: AuthState;
    connection: {
        status: ConnectionStatus
    },
    wallet: {
        solana: string;
    },
    viewport: ViewportState;
};

export type AppAction =
    | { type: 'APP_NAVIGATE_PAGE', page: PageType }
    | { type: 'APP_LOGIN_METAMASK', user: { id: string }, token: string, wallet: { solana: string } }
    | {
    type: 'APP_LOGIN_TELEGRAM',
    user: { id: string },
    token: string,
    telegram: { id: string, username: string },
    wallet: { solana: string }
}
    | { type: 'APP_CONNECTED' }
    | { type: 'APP_DISCONNECTED' }
    | { type: 'APP_LOGOUT' }

export const appReducer = (state: AppState, action: AppAction): AppState => {
    switch (action.type) {
        case "APP_NAVIGATE_PAGE": {
            return {
                ...state,
                page: action.page
            }
        }
        case "APP_CONNECTED": {
            return {
                ...state,
                connection: {
                    status: "CONNECTED"
                }
            }
        }
        case "APP_DISCONNECTED": {
            return {
                ...state,
                connection: {
                    status: "DISCONNECTED"
                }
            }
        }
        case "APP_LOGIN_METAMASK": {
            return {
                ...state,
                auth: {
                    type: 'MetaMask',
                    user: {
                        id: action.user.id
                    },
                    token: action.token,
                    telegram: {
                        id: '',
                    },
                },
                wallet: action.wallet,
            } satisfies AppState
        }

        case "APP_LOGIN_TELEGRAM": {
            return {
                ...state,
                auth: {
                    type: 'Telegram',
                    user: {
                        id: action.user.id
                    },
                    token: action.token,
                    telegram: action.telegram,
                },
                wallet: action.wallet,
            }
        }
        case "APP_LOGOUT": {
            return {
                ...state,
                auth: {
                    type: "Unauthorized",
                    user: {
                        id: '',
                    },
                    token: '',
                    telegram: {
                        id: '',
                    }
                },
                wallet: {
                    solana: ''
                },
                connection: {
                    status: "DISCONNECTED"
                }
            } satisfies AppState
        }
        default:
            throw new Error(`Not supported action type: ${action}`);
    }
}

export const appInitialState = (localStorage: LocalStorage): AppState => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    let isTelegram = (window?.Telegram?.WebApp?.initData || "") !== "";
    let type: AppType = isTelegram ? "Telegram" : "WebApp";

    return ({
        type,
        theme: "DARK",
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        telegramData: window?.Telegram?.WebApp,
        page: "Wallet",
        auth: {
            type: localStorage.auth.type,
            user: {
                id: localStorage.auth.user.id
            },
            token: localStorage.auth.token,
            telegram: {
                id: localStorage.auth.telegram.id,
            }
        },
        connection: {
            status: "DISCONNECTED",
        },
        wallet: {
            solana: localStorage.wallet.solana
        },
        viewport: {
            width: window.innerWidth,
            height: window.innerHeight,
        }
    })
}


