import React, {useContext} from "react";
import {ContextAppState} from "@app/context.ts";
import {Editor} from "@components/editor";

export const StrategyCreatePage: React.FC = () => {
    let appState = useContext(ContextAppState);
    return (
        <div className="relative">
            <div className="mb-4">
            </div>
            <div className="mx-auto max-w-[1440px] px-4 pb-64 pt-4 md:px-6">
                <Editor/>
            </div>

            <div className="mb-4">
            </div>
        </div>
    )
}

