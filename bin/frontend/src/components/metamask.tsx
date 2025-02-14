import {FC, useEffect, useState} from "react";
import {useSDK} from "@metamask/sdk-react-ui";
import {useMetaMask} from "@hooks/auth";

type MetaMaskButtonProps = {
    className?: string;
}

export const MetaMaskButton: FC<MetaMaskButtonProps> = ({className}) => {
    const [connecting, setConnecting] = useState(false)
    const {sdk, account} = useSDK();

    const [requestToken, , , e] = useMetaMask()
    const [signedMessage, setSignedMessage] = useState(undefined);

    const handleConnectAndSign = async () => {
        try {
            const signature = await sdk?.connectAndSign({msg: "Please sign this message to login"});
            setSignedMessage(signature);
        } catch (error) {
            console.error("Error in signing:", error);
        }
    };

    useEffect(() => {
        if (account && signedMessage) {
            const invoke = async () => {
                requestToken(account!, signedMessage)
            }
            invoke()
        }
    }, [account, requestToken, signedMessage]);


    if (e) {
        return (
            <h1 className={"text-center text-blue-800 text-xl"}> You need a friend code to join the adventure</h1>
        )
    }

    return (
        <>
            <button
                onClick={handleConnectAndSign}
                // loading={connecting}
            >
                Connect with MetaMask
            </button>
        </>
    )
}

