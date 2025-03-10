import React, {FC, ReactNode, useContext, useEffect, useReducer} from "react";
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
    const [telegramLogin, , , telegramErr] = useTelegram();
    const {telegramData, auth} = useContext(ContextAppState);

    useEffect(() => {
        const abortController = new AbortController();

        const retryLogin = () => {
            if (auth.type === "Unauthorized" && telegramData?.initData) {
                telegramLogin(telegramData.initData, location.pathname, abortController);

                setTimeout(() => {
                    if (auth.type === "Unauthorized") {
                        retryLogin();
                    }
                }, 1500);
            }
        };

        retryLogin();

        return () => {
            abortController.abort();
        };
    }, [auth, telegramData, location.pathname, telegramLogin]);

    if (auth.type === "Unauthorized") {
        return <h1 className="text-center text-blue-800 text-xl">Starting your telegram terminal</h1>;
    }

    if (telegramErr) {
        return <h1>Telegram terminal says no</h1>;
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


