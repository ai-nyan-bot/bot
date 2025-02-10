import React, {useContext} from 'react';
import {ContextAppState} from "@app/context.ts";
import {BottomBar} from "@pages/wallet/components/bottom-bar";


export const BrowserPage: React.FC = () => {
    let appState = useContext(ContextAppState);
    return (
        <div className="relative">
            <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">
                <h1> Browse </h1>
            </div>

            <div className="mb-4">
                <BottomBar/>
            </div>
        </div>
    )
}


