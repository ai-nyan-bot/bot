import React from "react";
import {ArrowDownIcon, ArrowsRightLeftIcon, ArrowUpIcon} from '@heroicons/react/20/solid';
import {useNavigate} from "@hooks";

export const WalletActions = () => {
    const navigate = useNavigate();
    return (
        <div className="grid grid-cols-3 gap-4 p-4 text-white">
            {/* Send Button */}
            <button
                className="flex flex-col items-center justify-center bg-gray-800 hover:bg-gray-700 p-4 rounded-lg shadow-md focus:outline-none">
                <ArrowUpIcon className="h-8 w-8 text-blue-400 mb-2"/>
                <span className="text-sm font-medium">Send</span>
            </button>

            {/* Receive Button */}
            <button
                className="flex flex-col items-center justify-center bg-gray-800 hover:bg-gray-700 p-4 rounded-lg shadow-md focus:outline-none">
                <ArrowDownIcon className="h-8 w-8 text-green-400 mb-2"/>
                <span className="text-sm font-medium">Receive</span>
            </button>

            {/* Swap Button */}
            <button
                onClick={() => {
                    navigate("WalletSwap")
                }}
                className="flex flex-col items-center justify-center bg-gray-800 hover:bg-gray-700 p-4 rounded-lg shadow-md focus:outline-none">
                <ArrowsRightLeftIcon className="h-8 w-8 text-purple-400 mb-2"/>
                <span className="text-sm font-medium">Swap</span>
            </button>


        </div>
    );
};
