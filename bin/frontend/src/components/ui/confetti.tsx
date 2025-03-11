import ReactConfetti from "react-confetti";
import React, {useContext, useEffect, useState} from "react";
import {ContextAppDispatch, ContextAppState} from "@app/context.ts";

export const Confetti = () => {
    const state = useContext(ContextAppState);
    let confetti = state.confetti;
    const dispatch = useContext(ContextAppDispatch);
    const [isConfettiActive, setIsConfettiActive] = useState(false);
    const [source, setSource] = useState<{x: number, y: number, w: number, h: number}>();

    useEffect(() => {
        if (confetti && confetti !== "OFF") {
            setIsConfettiActive(true);
            const timer = setTimeout(() => {
                setIsConfettiActive(false);
                dispatch({type: "APP_CONFETTI_HIDE"});
            }, 3000);
            return () => clearTimeout(timer);
        }
    }, [confetti]);

    return (
        isConfettiActive && (
            <ReactConfetti
                width={state.viewport.width}
                height={state.viewport.height}
                numberOfPieces={500}
                confettiSource={{
                    x: 0,
                    y: 0,
                    w: state.viewport.width,
                    h: state.viewport.height / 2
                }}
                gravity={0.5}
            />
        )
    )
}