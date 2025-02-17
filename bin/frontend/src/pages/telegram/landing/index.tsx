import {useNavigate} from "react-router-dom";
import {useContext, useEffect} from "react";
import {ContextAppState} from "@app/context.ts";
import {useTelegram} from "@hooks";

const TelegramLandingPage = () => {
    const navigate = useNavigate();
    const appState = useContext(ContextAppState);
    const [telegramLogin, token, loading, e] = useTelegram();

    useEffect(() => {
        const abortController = new AbortController();
        const auth = appState.auth;
        if (auth.type === 'Telegram') {
            navigate("/home");
        }

        if (auth.type === 'Unauthorized') {
            telegramLogin(appState.telegramData!.initData!, abortController);
        }
        return () => {
            abortController.abort();
        }
    }, [appState, navigate, telegramLogin]);

    if (e) {
        return (
            <h1> Login failed with {JSON.stringify(e)}</h1>
        )
    }

    if (token == null || loading) {
        return (
            <h1 className={"text-center text-blue-800 text-xl"}>Starting your telegram terminal </h1>
        )
    }

    return (<h1>Telegram terminal says no </h1>);
}

export default TelegramLandingPage;