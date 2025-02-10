import React, {useEffect, useState} from 'react';
import Button from "@components/forms/button.tsx";
import {useSwap} from "@hooks/wallet.ts";
import {BottomBar} from "@pages/wallet/components/bottom-bar";

export interface Token {
    icon: string;
    name: string;
    amount: number;
}

const tokens = [
    {
        icon: "https://cryptologos.cc/logos/toncoin-ton-logo.png",
        name: "TON",
        amount: 10,
    }
]

export const SwapPage: React.FC = () => {

    const [fromToken, setFromToken] = useState<Token>({
        icon: "https://cryptologos.cc/logos/toncoin-ton-logo.png",
        name: "TON",
        amount: 10,
    });
    const [toToken, setToToken] = useState<Token>({
        icon: "https://cryptologos.cc/logos/toncoin-ton-logo.png",
        name: "TON",
        amount: 10,
    });
    const [amount, setAmount] = useState<number>(0);

    const handleSliderChange = (value: number) => {
        setAmount(value);
    };

    const [swap, swapResponse, loading, swapError] = useSwap();

    useEffect(() => {
        if (swapResponse != null) {
            console.log("Performed swap", swapResponse);
        }

    }, [swapResponse]);


    const handleSwap = () => {
        // if (amount <= 0) {
        //     alert("Please enter a valid amount to swap.");
        //     return;
        // }
        // if (fromToken === toToken) {
        //     alert("Please select different tokens to swap.");
        //     return;
        // }

        swap('1', {
            from: "So11111111111111111111111111111111111111112",
            to: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            amount: 1_000_000
        })


        console.log("Perform swap");

        // onSwap(fromToken, toToken, amount);
    };

    return (

        <div className="relative">
            <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">
                <div className="text-white p-6 rounded-lg shadow-md space-y-6">
                    {/* From Token */}
                    <div className="p-4 rounded-lg">
                        <div className="flex items-center justify-between mb-2">
                            <label className="text-sm text-gray-400">You Send</label>
                            <div className="flex items-center space-x-2">
                                <img
                                    src={fromToken.icon}
                                    alt={fromToken.name}
                                    className="h-8 w-8 rounded-full"
                                />
                                <select
                                    value={fromToken.name}
                                    onChange={(e) =>
                                        setFromToken(tokens.find((t) => t.name === e.target.value)!)
                                    }
                                    className="text-white text-sm p-2 rounded-md focus:outline-none"
                                >
                                    {tokens.map((token) => (
                                        <option key={token.name} value={token.name}>
                                            {token.name}
                                        </option>
                                    ))}
                                </select>
                            </div>
                        </div>
                        <div className="text-3xl font-bold mb-2">{amount}</div>
                        <input
                            type="range"
                            min={0}
                            max={fromToken.amount}
                            value={amount}
                            onChange={(e) => handleSliderChange(Number(e.target.value))}
                            className="w-full"
                        />
                        <div className="flex justify-between text-sm text-gray-400 mt-2">
                            <span>0</span>
                            <span>{fromToken.amount}</span>
                        </div>
                    </div>

                    {/* Swap Icon */}
                    <div className="flex justify-center">
                        <button
                            onClick={() => {
                                const temp = fromToken;
                                setFromToken(toToken);
                                setToToken(temp);
                            }}
                            className="bg-purple-600 p-3 rounded-full hover:bg-purple-500 transition"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                className="h-6 w-6 text-white"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                strokeWidth={2}
                            >
                                <path
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    d="M8 16l-4-4m0 0l4-4m-4 4h16m-6-4l4 4m0 0l-4 4"
                                />
                            </svg>
                        </button>
                    </div>

                    {/* To Token */}
                    <div className="p-4 rounded-lg">
                        <div className="flex items-center justify-between mb-2">
                            <label className="text-sm text-gray-400">You Receive</label>
                            <div className="flex items-center space-x-2">
                                <img
                                    src={toToken.icon}
                                    alt={toToken.name}
                                    className="h-8 w-8 rounded-full"
                                />
                                <select
                                    value={toToken.name}
                                    onChange={(e) =>
                                        setToToken(tokens.find((t) => t.name === e.target.value)!)
                                    }
                                    className="text-white text-sm p-2 rounded-md focus:outline-none"
                                >
                                    {tokens.map((token) => (
                                        <option key={token.name} value={token.name}>
                                            {token.name}
                                        </option>
                                    ))}
                                </select>
                            </div>
                        </div>
                        <div className="text-3xl font-bold">{(amount * 0.98).toFixed(2)}</div>
                    </div>

                    {/* Swap Button */}
                    <Button
                        onClick={handleSwap}
                    >
                        Swap
                    </Button>
                    {/*<button*/}
                    {/*    className="w-full bg-blue-600 hover:bg-blue-500 text-white font-bold py-3 rounded-lg transition"*/}
                    {/*    onClick={handleSwap}*/}
                    {/*>*/}
                    {/*    Swap*/}
                    {/*</button>*/}
                </div>
            </div>

            <div className="mb-4">
                <BottomBar/>
            </div>
        </div>
    );
}


