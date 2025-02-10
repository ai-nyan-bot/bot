import React, {FC, ReactNode, useContext, useEffect} from "react";
import {ContextAppDispatch, ContextAppState, ContextClient} from "@app/context";

type ClientProps = {
    children: ReactNode;
}

export const Client: FC<ClientProps> = ({children}) => {
    const {auth} = useContext(ContextAppState);
    if (auth.type === "Unauthorized") {
        return (children);
    }

    return (<Delegate userId={auth.user.id} token={auth.token}> {children}</Delegate>);
}

type DelegateProps = {
    userId: string;
    token: string;
    children: ReactNode
}

const Delegate: FC<DelegateProps> = ({userId, token, children}) => {
    const appState = useContext(ContextAppState);
    const appDispatch = useContext(ContextAppDispatch);

    useEffect(() => {
        appDispatch({type: "APP_CONNECTED"});
    }, [])

    // useWebSocket(
    //     `${import.meta.env.VITE_BASE_URL}/ws?access_token=valid-token`,
    //     {
    //         onOpen: (event) => {
    //             appDispatch({type: "APP_CONNECTED"});
    //             // console.log("connection opened");
    //         },
    //         onMessage: (msg) => {
    //             console.log("Received message", msg);
    //         },
    //         onError: (error) => {
    //             console.log("Received error", error)
    //         },
    //         onReconnectStop: (attempt) => {
    //             console.log("on reconnect stop")
    //         },
    //         onClose: (event) => {
    //             appDispatch({type: "APP_DISCONNECTED"});
    //         },
    //         reconnectAttempts: 10,
    //         reconnectInterval: 3000,
    //         heartbeat: true,
    //     }
    // );

    return (
        <ContextClient.Provider value={{}}>
            {children}
        </ContextClient.Provider>
    )
}