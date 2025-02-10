import {useContext, useEffect} from "react";
import {ContextAppState} from "@app/context";
import {useNavigate} from "react-router-dom";

export const ConnectionLostPage = () => {
    const appState = useContext(ContextAppState);
    const authType = appState.auth.type;
    const status = appState.connection.status
    const navigate = useNavigate();

    useEffect(() => {
        if (authType === "Unauthorized") {
            navigate("/");
        }
        if (status === "CONNECTED") {
            navigate("/portfolios");
        }
    }, [authType, status, navigate]);

    return (<div>
        <h1>Connection lost</h1>
    </div>);
}