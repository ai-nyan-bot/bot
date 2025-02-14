import {useNavigate} from "react-router-dom";
import {MetaMaskButton} from "@components/metamask";
import {useContext, useEffect} from "react";
import {ContextAppState} from "@app/context";
import {useTelegram} from "@hooks/auth";

const LandingPage = () => {
    const appState = useContext(ContextAppState);
    return (
        <div className={"w-full h-screen flex flex-col justify-evenly"}>
            {appState.type === "Telegram" && <TelegramLanding/>}
            {appState.type === "WebApp" && <MetaMaskLanding/>}
        </div>
    );
}

export default LandingPage

const TelegramLanding = () => {
    const navigate = useNavigate();
    const appState = useContext(ContextAppState);
    const [telegramLogin, response, loading, e] = useTelegram();

    useEffect(() => {
        const abortController = new AbortController();

        const auth = appState.auth;
        if (auth.type === 'Telegram') {
            // navigate("/telegram/home");
        }

        if (auth.type === 'Unauthorized') {
            // telegramLogin(appState.telegramData!.initData!, abortController);
        }
        return () => {
            abortController.abort();
        }
    }, [appState.auth, appState.telegramData, navigate, telegramLogin]);

    if (e) {
        return (
            <h1> Login failed with {JSON.stringify(e)}</h1>
        )
    }

    if (response == null || loading) {
        return (
            <h1 className={"text-center text-blue-800 text-xl"}>Starting your telegram terminal</h1>
        )
    }

    return (<h1>Telegram terminal says no</h1>);
}

const MetaMaskLanding = () => {
    const appState = useContext(ContextAppState);
    const navigate = useNavigate();

    useEffect(() => {
        const abortController = new AbortController();

        const auth = appState.auth;
        if (auth.type === "MetaMask") {
            navigate("/web/home");
        }

        return () => {
            abortController.abort();
        }
    }, [appState.auth, appState.telegramData, navigate]);


    if (appState.auth.type == 'Unauthorized') {
        return (
            <div className={"w-full flex flex-row justify-center"}>
                <MetaMaskButton/>
            </div>
        )
    }
    return (
        <div className={"page"}>
            <h1 className={"text-center text-blue-800 text-xl"}>Starting your web terminal</h1>
        </div>
    )
}