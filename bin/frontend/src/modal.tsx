import {FC, useContext} from "react";
import {ContextModalState} from "@app/context";


export const Modal: FC = () => {
    const state = useContext(ContextModalState);
    switch (state.type) {
        default:
            return null;
    }
}

