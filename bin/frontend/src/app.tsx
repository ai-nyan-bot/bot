import React, {FC, ReactNode, useContext, useEffect, useReducer} from "react";
import {BrowserRouter, Route, Routes, useNavigate} from "react-router-dom";

import {audioInitialState, audioReducer} from "@states/audio";
import {modalInitialState, modalReducer} from "@states/modal";
import {
    ContextAppDispatch,
    ContextAppState,
    ContextAudioDispatch,
    ContextAudioState,
    ContextModalDispatch,
    ContextModalState
} from "./context.ts";

import {
    useLocalStorage,
    useSetAudioBackground,
    useSetMetaMaskAuth,
    useSetTelegramAuth,
    useSetUnauthorized,
    useSetWallet
} from "@states/local";
import {appInitialState, appReducer} from "@states/app";
import {Client} from "@components";

import LandingPage from "@pages/landing";

import {Modal} from "./modal.tsx";

import {ConnectionLostPage} from "@pages/connection-lost";
import Layout from "@app/layout.tsx";
import NotFound from "@app/not-found.tsx";

import './styles/globals.css'
import {MetaMaskUIProvider} from "@metamask/sdk-react-ui";

import {BrowserPage, HistoryPage, WalletPage} from "@pages/wallet";
import {BalancePage} from "@pages/balance";
import {SwapPage} from "@pages/wallet/pages/swap";
import {StrategyCreatePage} from "@pages/strategy-create";

const Authenticated: FC<{ children: ReactNode }> = ({children}) => {
    const {auth, connection} = useContext(ContextAppState);
    const navigate = useNavigate();

    useEffect(() => {
        if (auth.type === "Unauthorized") {
            navigate("/")
        }
    }, [auth, navigate]);

    useEffect(() => {
        if (connection.status === "DISCONNECTED") {
            navigate("/connection-lost")
        }
    }, [connection, navigate]);

    if (auth.type === "Unauthorized" || connection.status === "DISCONNECTED") {
        return null;
    }

    return (children)
}

const AppRouter = () => (
    <BrowserRouter>
        <Routes>
            <Route path={"/"} element={<LandingPage/>}/>
            <Route path={"/connection-lost"} element={<ConnectionLostPage/>}/>

            <Route path={"/balance"} element={<Authenticated><BalancePage/></Authenticated>}/>
            {/*<Route path={"/home/portfolios"} element={<Authenticated><HomePage/></Authenticated>}/>*/}


            <Route path={"/strategy/create"} element={<Authenticated><StrategyCreatePage/></Authenticated>}></Route>

            <Route path={"/wallet"} element={<Authenticated><WalletPage/></Authenticated>}/>
            <Route path={"/wallet/swap"} element={<Authenticated><SwapPage/></Authenticated>}/>
            <Route path={"/wallet/history"} element={<Authenticated><HistoryPage/></Authenticated>}/>
            <Route path={"/wallet/browser"} element={<Authenticated><BrowserPage/></Authenticated>}/>

            {/*<Route path={"/portfolio/:id"} element={<Authenticated><PortfolioPage/></Authenticated>}/>*/}
            {/*<Route path={"/friends"} element={<Authenticated><FriendsPage/></Authenticated>}/>*/}

            {/*<Route path={"/privacy-policy"} element={<PrivacyPolicyPage/>}/>*/}
            <Route path='*' element={<NotFound/>}/>
        </Routes>
    </BrowserRouter>
)

const WebApp = () => {
    return (
        <MetaMaskUIProvider
            key={'netmask'}
            debug={false}
            sdkOptions={{
                checkInstallationImmediately: false,
                dappMetadata: {
                    name: "NyanBot",
                    url: "https://nyanbot.com",
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

    const setAudioBackground = useSetAudioBackground();
    const setMetaMaskAuth = useSetMetaMaskAuth();
    const setTelegramAuth = useSetTelegramAuth();
    const setUnauthorized = useSetUnauthorized();
    const setWallet = useSetWallet();

    const [appState, appDispatch] = useReducer(appReducer, appInitialState(localStorage));
    const [audioState, audioDispatch] = useReducer(audioReducer, audioInitialState(localStorage));
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
        setAudioBackground(audioState.background.active, audioState.background.volume);
    }, [audioState.background, setAudioBackground]);

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
                    <ContextAudioState.Provider value={audioState}>
                        <ContextAudioDispatch.Provider value={audioDispatch}>
                            <ContextModalState.Provider value={modalState}>
                                <ContextModalDispatch.Provider value={modalDispatch}>
                                    <Client>
                                        <Modal/>
                                        <Layout>
                                            {app}
                                        </Layout>
                                        {/*<AudioPlayer/>*/}
                                    </Client>
                                </ContextModalDispatch.Provider>
                            </ContextModalState.Provider>
                        </ContextAudioDispatch.Provider>
                    </ContextAudioState.Provider>
                </ContextAppDispatch.Provider>
            </ContextAppState.Provider>
        </>
    );
}

export default App


