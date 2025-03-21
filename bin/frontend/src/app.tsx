import React, {FC, ReactNode, useCallback, useContext, useEffect, useReducer, useState} from "react";
import {BrowserRouter, Route, Routes, useLocation} from "react-router-dom";

import {modalInitialState, modalReducer} from "@states/modal";
import {ContextAppDispatch, ContextAppState, ContextModalDispatch, ContextModalState} from "./context.ts";

import {useLocalStorage, useSetMetaMaskAuth, useSetTelegramAuth, useSetUnauthorized, useSetWallet} from "@states/local";
import {appInitialState, appReducer} from "@states/app";
import {Client} from "./client";

import {Modal} from "./modal.tsx";

import Layout from "@app/layout.tsx";
import NotFound from "@app/not-found.tsx";

import './styles/globals.css'
import {MetaMaskUIProvider} from "@metamask/sdk-react-ui";

import TelegramHomePage from "@pages/telegram/home";
import TelegramRuleListPage from "@pages/telegram/rule-list";
import TelegramRuleDetailPage from "@pages/telegram/rule-detail";
import WebHomePage from "@pages/web/home";
import WebRuleListPage from "@pages/web/rule-list";
import WebRuleDetailPage from "@pages/web/rule-detail";
import {useTelegram} from "@hooks";
import {MetaMaskButton} from "@components/metamask.tsx";
import {Confetti} from "@components/ui/confetti.tsx";


const WebAuthenticated: FC<{ children: ReactNode }> = ({children}) => {
    const location = useLocation();
    const {auth} = useContext(ContextAppState);
    if (auth.type === "Unauthorized") {
        return (
            <div className={"w-full flex flex-row justify-center"}>
                <MetaMaskButton redirect={location.pathname}/>
            </div>
        )
    }
    return children;
};

const TelegramAuthenticated: FC<{ children: ReactNode }> = ({children}) => {
    const location = useLocation();
    const [telegramLogin, , loading, telegramErr] = useTelegram();
    const {telegramData, auth} = useContext(ContextAppState);
    const [retries, setRetries] = useState(0);
    const [manualRetry, setManualRetry] = useState(false);
    const MAX_RETRIES = 5;

    const doLogin = useCallback(() => {
        if (auth.type === "Unauthorized" && telegramData?.initData) {
            const abortController = new AbortController();
            telegramLogin(telegramData.initData, location.pathname, abortController);
        }
    }, [auth, telegramData, location.pathname, telegramLogin]);

    useEffect(() => {
        if (retries < MAX_RETRIES && !manualRetry) {
            const timeout = setTimeout(() => {
                if (auth.type === "Unauthorized") {
                    setRetries(prev => prev + 1);
                    doLogin();
                }
            }, 1500);

            return () => clearTimeout(timeout);
        }
    }, [auth, retries, manualRetry, doLogin]);

    const handleRetry = () => {
        setRetries(0);
        setManualRetry(false);
        doLogin();
    };

    if (!telegramData) {
        return (
            <div className="text-center">
                <h1 className="text-red-700 text-xl mb-4">Failed to retrieve data from Telegram</h1>
                <h2 className="text-center text-blue-800 text-xl"> Please close & reopen the bot</h2>
            </div>
        );
    }

    if (telegramErr) {
        return (
            <div className="text-center">
                <h1 className="text-red-700 text-xl mb-4">Telegram terminal says no</h1>
                <button
                    className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded"
                    onClick={handleRetry}
                >
                    Retry
                </button>
            </div>
        );
    }

    if (auth.type === "Unauthorized" && retries < MAX_RETRIES && loading) {
        return <h1 className="text-center text-blue-800 text-xl">Starting your telegram terminal</h1>;
    }

    if (auth.type === "Unauthorized" && retries >= MAX_RETRIES) {
        return (
            <div className="text-center">
                <h1 className="text-yellow-800 text-xl mb-4">We are sorry, login failed
                    after {MAX_RETRIES} attempts</h1>
                <button
                    className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded"
                    onClick={handleRetry}
                >
                    Retry
                </button>
            </div>
        );
    }

    return children;
};


const AppRouter = () => {
    const appState = useContext(ContextAppState);

    if (appState.type === 'Telegram') {
        return (
            <BrowserRouter>
                <Routes>
                    <Route
                        path={"/"}
                        element={
                            <TelegramAuthenticated>
                                <TelegramHomePage/>
                            </TelegramAuthenticated>
                        }
                    />
                    <Route
                        path={"/rules"}
                        element={
                            <TelegramAuthenticated>
                                <TelegramRuleListPage/>
                            </TelegramAuthenticated>
                        }
                    />
                    <Route
                        path={"/rules/:id"}
                        element={
                            <TelegramAuthenticated>
                                <TelegramRuleDetailPage/>
                            </TelegramAuthenticated>
                        }
                    />
                    <Route path='*' element={<NotFound/>}/>
                </Routes>
            </BrowserRouter>
        );
    }

    if (appState.type === 'WebApp') {
        return (
            <BrowserRouter>
                <Routes>
                    <Route
                        path={"/"}
                        element={
                            <WebAuthenticated>
                                <WebHomePage/>
                            </WebAuthenticated>
                        }
                    />
                    <Route
                        path={"/rules"}
                        element={
                            <WebAuthenticated>
                                <WebRuleListPage/>
                            </WebAuthenticated>
                        }
                    />
                    <Route
                        path={"/rules/:id"}
                        element={
                            <WebAuthenticated>
                                <WebRuleDetailPage/>
                            </WebAuthenticated>
                        }
                    />
                    <Route path='*' element={<NotFound/>}/>
                </Routes>
            </BrowserRouter>
        );
    }

}
const WebApp = () => {
    return (
        <MetaMaskUIProvider
            key={'netmask'}
            debug={false}
            sdkOptions={{
                checkInstallationImmediately: false,
                dappMetadata: {
                    name: "Nyanbot",
                    url: "https://nyan.bot",
                }
            }}>
            <AppRouter/>
        </MetaMaskUIProvider>
    )
}

const TelegramApp = () => {
    return (
        <AppRouter/>
    )
}


const App = () => {
    const [localStorage] = useLocalStorage();

    const setMetaMaskAuth = useSetMetaMaskAuth();
    const setTelegramAuth = useSetTelegramAuth();
    const setUnauthorized = useSetUnauthorized();
    const setWallet = useSetWallet();

    const [appState, appDispatch] = useReducer(appReducer, appInitialState(localStorage));
    const [modalState, modalDispatch] = useReducer(modalReducer, modalInitialState());

    useEffect(() => {
        if (appState.type == "Telegram") {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            const webapp = window?.Telegram?.WebApp;

            if (webapp && webapp.ready) {
                webapp.ready();
            }

            if (webapp && webapp.expand) {
                webapp.expand();
            }
        }
    }, [appState]);

    useEffect(() => {
        const auth = appState.auth;
        if (auth.type === "MetaMask") {
            setMetaMaskAuth(auth.user.id, auth.token);
        } else if (auth.type === "Telegram") {
            setTelegramAuth(auth.user.id, auth.token, auth.telegram!!.id);
        } else {
            setUnauthorized();
        }
    }, [appState, setMetaMaskAuth, setTelegramAuth, setUnauthorized]);

    useEffect(() => {
        setWallet(appState.wallet.solana)
    }, [appState.wallet, setWallet])

    const app = appState.type === "Telegram" ? <TelegramApp/> : <WebApp/>;
    document.documentElement.setAttribute('data-theme', "Dark");
    return (
        <>
            <ContextAppState.Provider value={appState}>
                <ContextAppDispatch.Provider value={appDispatch}>
                    <ContextModalState.Provider value={modalState}>
                        <ContextModalDispatch.Provider value={modalDispatch}>
                            <Client>
                                <Modal/>
                                <Confetti/>
                                <Layout>
                                    {app}
                                </Layout>
                            </Client>
                        </ContextModalDispatch.Provider>
                    </ContextModalState.Provider>
                </ContextAppDispatch.Provider>
            </ContextAppState.Provider>
        </>
    );
}

export default App


