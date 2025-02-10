import {useNavigate} from "react-router-dom";
import {MetaMaskButton} from "@components/metamask";
import {useContext, useEffect} from "react";
import {ContextAppState} from "@app/context";
import {useTelegram} from "@hooks/auth";
import {imageUrl} from "@utils";

const LandingPage = () => {
    const appState = useContext(ContextAppState);
    return (
        <div className={"w-full h-screen flex flex-col justify-evenly"}>
            <img src={imageUrl("bot.png")} className={"w-full"}/>
            {appState.type === "Telegram" && <TelegramLanding/>}
            {appState.type === "WebApp" && <MetaMaskLanding/>}
        </div>
    );
}

export default LandingPage

const TelegramLanding = () => {
    const navigate = useNavigate();
    const appState = useContext(ContextAppState);
    const code = appState.telegramData?.initDataUnsafe?.start_param || null;

    const [telegramLogin, , , e] = useTelegram();

    useEffect(() => {
        const abortController = new AbortController();

        const auth = appState.auth;
        if (auth.type === "Telegram") {
            navigate("/wallet");
        }

        if (auth.type === 'Unauthorized') {
            telegramLogin(appState.telegramData!.initData!, code, abortController);
        }
        return () => {
            abortController.abort();
        }
    }, [appState.auth, appState.telegramData, navigate, telegramLogin, code]);

    return (
        <h1 className={"text-center text-blue-800 text-xl"}>Starting your terminal</h1>
    )

}

const MetaMaskLanding = () => {
    const appState = useContext(ContextAppState);
    const navigate = useNavigate();
    const friendCode = new URLSearchParams(location.search).get('friend_code') || null;

    useEffect(() => {
        const abortController = new AbortController();

        const auth = appState.auth;
        if (auth.type === "MetaMask") {
            navigate("/wallet");
        }

        return () => {
            abortController.abort();
        }
    }, [appState.auth, appState.telegramData, navigate]);


    if (appState.auth.type == 'Unauthorized') {
        return (
            <div className={"w-full flex flex-row justify-center"}>
                <MetaMaskButton code={friendCode}/>
            </div>
        )
    }
    return (
        <div className={"page"}>
            <h1 className={"text-center text-blue-800 text-xl"}>Starting your terminal</h1>
        </div>
    )
}