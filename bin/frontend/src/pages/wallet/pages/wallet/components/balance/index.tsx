import React, {FC} from "react";

interface WalletBalanceProps {
    balance: number;
}

export const WalletBalance: FC<WalletBalanceProps> = ({balance}) => {
    return (
        <div className="flex flex-col items-center justify-center text-white p-2 rounded-lg shadow-md">
            <div className="text-4xl font-bold mt-2">
                ${balance.toLocaleString("en-US", {minimumFractionDigits: 2})}
            </div>
        </div>
    );
};
