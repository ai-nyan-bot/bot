import ReactConfetti from "react-confetti";
import React, {useContext, useEffect, useState} from "react";
import {ContextAppDispatch, ContextAppState} from "@app/context.ts";

export const Confetti = () => {
    const state = useContext(ContextAppState);
    let confetti = state.confetti;
    const dispatch = useContext(ContextAppDispatch);
    const [isConfettiActive, setIsConfettiActive] = useState(false);

    const [dimensions, setDimensions] = useState({
        width: window.innerWidth,
        height: document.documentElement.scrollHeight,
    });

    useEffect(() => {
        const updateSize = () => {
            setDimensions({
                width: window.innerWidth,
                height: document.documentElement.scrollHeight,
            });
        };

        window.addEventListener("resize", updateSize);
        window.addEventListener("scroll", updateSize);

        return () => {
            window.removeEventListener("resize", updateSize);
            window.removeEventListener("scroll", updateSize);
        };
    }, []);

    useEffect(() => {
        if (confetti && confetti !== "OFF") {
            setIsConfettiActive(true);
            const timer = setTimeout(() => {
                setIsConfettiActive(false);
                dispatch({type: "APP_CONFETTI_HIDE"});
            }, 2000);
            return () => clearTimeout(timer);
        }
    }, [confetti]);

    return (
        isConfettiActive && (
            <div className="fixed top-0 left-0 w-screen h-full pointer-events-none">
                <ReactConfetti
                    width={state.viewport.width}
                    height={state.viewport.height}
                    numberOfPieces={500}
                    tweenDuration={1000}
                    confettiSource={{
                        x: 0,
                        y: 0,
                        w: dimensions.width,
                        h: dimensions.height,
                    }}
                    gravity={0.5}
                    recycle={false}
                />
            </div>
        )
    )
}