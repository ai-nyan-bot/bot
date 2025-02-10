import 'react';
import {FC, useState} from "react";

interface WalletAddressProps {
    address: string;
}

export const WalletAddress: FC<WalletAddressProps> = ({address}) => {
    const [showToast, setShowToast] = useState(false);
    const copyToClipboard = () => {
        navigator.clipboard
            .writeText(address)
            .then(() => {
                setShowToast(true);
                setTimeout(() => setShowToast(false), 3000); // Hide toast after 3 seconds
            })
            .catch((err) => {
                console.error("Failed to copy address:", err);
            });
    };

    return (
        <div className="relative">
            <div
                className="flex items-center justify-center  text-white p-4 rounded-lg shadow-md cursor-pointer hover:bg-gray-700 transition"
                onClick={copyToClipboard}
                title="Click to copy"
            >
        <span className="text-sm font-medium">
          Your address: {address.slice(0, 4)}...{address.slice(-4)}
        </span>
            </div>

            {showToast && (
                <div
                    className="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-green-600 text-white px-4 py-2 rounded-lg shadow-lg">
                    Address copied to clipboard!
                </div>
            )}
        </div>
    );
};
