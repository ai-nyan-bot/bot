import React, {useContext} from 'react';
import {ContextAppState} from "@app/context.ts";
import {Balance} from "./components/balance";
import {Token, Tokens} from "./components/tokens";


export const tokens: Token[] = [
    {
        icon: "https://cryptologos.cc/logos/solana-sol-logo.png",
        name: "Solana",
        price: {
            quote: 1.0,
            usd: 25.67,
        },
        amount: 10,
        volume: {
            quote: 10,
            usd: 256.7,
        },
    },
    {
        icon: "https://cryptologos.cc/logos/tether-usdt-logo.png",
        name: "USDT",
        price: {
            quote: 0.039,
            usd: 1.0,
        },
        amount: 500,
        volume: {
            quote: 19.5,
            usd: 500,
        },
    },
    {
        icon: "https://cryptologos.cc/logos/ethereum-eth-logo.png",
        name: "Ethereum",
        price: {
            quote: 0.04,
            usd: 1600.0,
        },
        amount: 2,
        volume: {
            quote: 0.08,
            usd: 3200.0,
        },
    },
    {
        icon: "https://cryptologos.cc/logos/bitcoin-btc-logo.png",
        name: "Bitcoin",
        price: {
            quote: 0.001,
            usd: 40000.0,
        },
        amount: 0.5,
        volume: {
            quote: 0.0005,
            usd: 20000.0,
        },
    },
];

export const BalancePage: React.FC = () => {
    let appState = useContext(ContextAppState);
    return (
        <div className="relative">
            <div className="mb-4">
            </div>
            <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">
                {/*<h1> Wallet </h1>*/}
                <Balance balance={133.70}/>
                <Tokens tokens={tokens}/>
            </div>
        </div>
    )
}


