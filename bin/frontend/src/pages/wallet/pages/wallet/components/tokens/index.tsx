import React, {FC} from "react";

export interface Token {
    icon: string;
    name: string;
    price: {
        quote: number;
        usd: number;
    }
    amount: number;
    volume: {
        quote: number;
        usd: number;
    }
}

export interface WalletTokensProps {
    tokens: Array<Token>;
}

export const WalletTokens: FC<WalletTokensProps> = ({tokens}) => {
    return (
        <div className=" text-white p-4 rounded-lg space-y-4">
            {tokens.map((token, index) => (
                <div
                    key={index}
                    className="flex items-center bg-gray-800 p-4 rounded-lg shadow-md space-x-4"
                >
                    <img
                        src={token.icon}
                        alt={`${token.name} icon`}
                        className="h-12 w-12 rounded-full"
                    />

                    <div className="flex-1">
                        <div className="flex items-center justify-between">
                            <span className="text-base font-medium">{token.name}</span>
                            <span className="text-base font-medium">{token.amount}</span>
                        </div>
                        <div className="flex items-center justify-between text-sm text-gray-400">
                            <div>${token.price.usd.toFixed(2)}</div>
                            <span>{`$${token.volume.usd.toFixed(2)}`}</span>
                        </div>
                        {/*<div className="flex items-center justify-between text-sm text-gray-400">*/}
                        {/*    <span>{token.price.quote.toFixed(6)} SOL</span>*/}
                        {/*    <span className="ml-4">{`${token.volume.quote.toFixed(4)} SOL`}</span>*/}
                        {/*</div>*/}
                    </div>
                </div>
            ))}
        </div>
    );
};