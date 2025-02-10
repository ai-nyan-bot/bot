import React, {FC, useContext} from "react";
import {useNavigate} from "@hooks";
import {PageType} from "@types";
import {ContextAppState} from "@app/context.ts";

// export type HomePageType = 'Portfolio' | 'Wallet' | 'Settings';

export const BottomBar: FC = () => {
    const navigate = useNavigate();
    const appState = useContext(ContextAppState);

    const styleIfActive = (page: PageType): string => {
        const activeTab = appState.page;
        if (page == activeTab) {
            return "bg-purple-500 rounded-xl"
        }
        return "";
    }

    return (
        <div
            className={"fixed py-1 bottom-0 w-full bg-purple-800 flex justify-around items-center z-10 text-xs"}>
            <button
                className={`m-2 p-2 text-center text-purple-50 w-1/3 ${styleIfActive("Wallet")}`}
                onClick={() => navigate("Wallet")}
            >
                {/*<SVGImage gradient={"None"} src={"bot.svg"} className={"w-8 h-8 mx-auto"}/>*/}
                <p className={"mt-1"}>Wallet</p>
            </button>
            <button
                className={`m-2 p-2 text-center text-purple-50 w-1/3 ${styleIfActive("WalletSwap")}`}
                onClick={() => navigate("WalletSwap")}
            >
                {/*<SVGImage gradient={"None"} src={"friends.svg"} className={"w-8 h-8 mx-auto"}/>*/}
                <p className={"mt-1"}>Swap</p>
            </button>
            <button
                className={`m-2 p-2 text-center text-purple-50 w-1/3 ${styleIfActive("WalletHistory")}`}
                onClick={() => navigate("WalletHistory")}
            >
                {/*<SVGImage gradient={"None"} src={"friends.svg"} className={"w-8 h-8 mx-auto"}/>*/}
                <p className={"mt-1"}>History</p>
            </button>
            <button
                className={`m-2 p-2 text-center text-purple-50 w-1/3 ${styleIfActive("WalletBrowser")}`}
                onClick={() => navigate("WalletBrowser")}
            >
                {/*<SVGImage gradient={"None"} src={"friends.svg"} className={"w-8 h-8 mx-auto"}/>*/}
                <p className={"mt-1"}>Browse</p>
            </button>
        </div>
    );
}
