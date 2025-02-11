import useLocalStorageState from "use-local-storage-state";
import {useCallback} from "react";
import {AuthType} from "@types";

export const LOCAL_STORAGE_KEY = 'nyan'

export type Auth = {
    type: AuthType;
    user: {
        id: string;
    };
    token: string;
    telegram: {
        id: string;
    }
}

export type Audio = {
    gfx: {
        active: boolean;
        volume: number;
    }
    background: {
        active: boolean;
        volume: number;
    }
}

export type Wallet = {
    solana: string;
}

export type LocalStorage = {
    auth: Auth;
    audio: Audio;
    wallet: Wallet;
};

const LOCAL_STORAGE_DEFAULT = {
    auth: {
        type: 'Unauthorized',
        user: {id: ''},
        token: '',
        telegram: {
            id: '',
        }
    },
    audio: {
        gfx: {
            active: true,
            volume: 0.4
        },
        background: {
            active: true,
            volume: 0.4
        }
    },
    wallet: {
        solana: ''
    }
} satisfies LocalStorage;

export const useLocalStorage = () => {
    return useLocalStorageState<LocalStorage>(LOCAL_STORAGE_KEY, {
        defaultValue: {...LOCAL_STORAGE_DEFAULT}
    });
}

type SetMetamaskAuth = (userId: string, token: string) => void
export const useSetMetaMaskAuth = (): SetMetamaskAuth => {
    const [, setLocalStorage] = useLocalStorage()

    return useCallback((userId, token) => {
        setLocalStorage(prev => {
            return {
                ...prev,
                auth: {
                    type: "MetaMask",
                    user: {
                        id: userId
                    },
                    token,
                    telegram: {
                        id: '',
                    }
                }
            }
        })
    }, [setLocalStorage])
}

type SetTelegramAuth = (userId: string, token: string, telegramId: string) => void
export const useSetTelegramAuth = (): SetTelegramAuth => {
    const [, setLocalStorage] = useLocalStorage()
    return useCallback((userId, token, telegramId) => {
        setLocalStorage(prev => {
            return {
                ...prev,
                auth: {
                    type: "Telegram",
                    user: {
                        id: userId
                    },
                    token,
                    telegram: {
                        id: telegramId,
                    }
                }
            }
        })
    }, [setLocalStorage])
}

type SetUnauthorized = () => void
export const useSetUnauthorized = (): SetUnauthorized => {
    const [, setLocalStorage] = useLocalStorage()

    return useCallback(() => {
        setLocalStorage(prev => {
            return {
                ...prev,
                auth: {
                    type: "Unauthorized",
                    user: {id: ''},
                    token: '',
                    telegram: {
                        id: '',
                    }
                }
            }
        })
    }, [setLocalStorage])
}

type SetWallet = (solana: string) => void
export const useSetWallet = (): SetWallet => {
    const [, setLocalStorage] = useLocalStorage()

    return useCallback((solana: string) => {
        setLocalStorage(prev => {
            return {
                ...prev,
                wallet: {solana}
            }
        })
    }, [setLocalStorage])
}