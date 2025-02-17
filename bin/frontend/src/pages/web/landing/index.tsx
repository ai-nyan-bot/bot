import {useContext, useEffect} from "react";
import {ContextAppState} from "@app/context.ts";
import {useNavigate} from "react-router-dom";
import {MetaMaskButton} from "@components/metamask.tsx";

const MetaMaskLandingPage = () => {
    const appState = useContext(ContextAppState);
    const navigate = useNavigate();

    useEffect(() => {
        const abortController = new AbortController();

        const auth = appState.auth;
        if (auth.type === "MetaMask") {
            navigate("/home");
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

export default MetaMaskLandingPage;