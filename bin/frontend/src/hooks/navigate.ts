import {PageType} from "@types";
import {useCallback, useContext} from "react";
import {ContextAppDispatch} from "@app/context";
import {useNavigate as delegate} from "react-router-dom";


export type NavigateAction = (page: PageType) => void;
export const useNavigate = (): NavigateAction => {
    const appDispatch = useContext(ContextAppDispatch);
    const navigate = delegate();

    return useCallback((page: PageType) => {
        switch (page) {
            case "PortfolioList": {
                navigate("/portfolios");
                break;
            }
            case "Wallet": {
                navigate("/wallet");
                break;
            }
            case "WalletBrowser": {
                navigate("/wallet/browser");
                break;
            }
            case "WalletHistory": {
                navigate("/wallet/history");
                break;
            }
            case "WalletSwap": {
                navigate("/wallet/swap");
                break;
            }
        }
        appDispatch({type: "APP_NAVIGATE_PAGE", page});
    }, [appDispatch, navigate])
}