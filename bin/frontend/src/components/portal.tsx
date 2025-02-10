import React, {FC, ReactNode, useContext, useRef} from "react";
import {createPortal} from 'react-dom';
import {ContextModalDispatch, ContextModalState} from "@app/context";

type PortalProps = {
    title?: string;
    children: ReactNode;
    onClick?: (evt: React.MouseEvent<HTMLElement>) => void;
    onClose?: () => void;
}

export const Portal: FC<PortalProps> = ({title, onClose, onClick, children}) => {
    const modalState = useContext(ContextModalState);
    const modalDispatch = useContext(ContextModalDispatch);
    const modalRef = useRef<HTMLDivElement>(null);

    const close = (e: React.MouseEvent<HTMLElement>) => {
        if (modalRef.current) {
            if (onClose) {
                onClose()
            } else if (onClick) {
                onClick(e)
            } else {
                modalDispatch({type: "MODAL_CLOSE"})
            }
        }
    }

    if (modalState.type === "None") {
        return null;
    }

    return createPortal(
        <div
            ref={modalRef}
            onClick={(e) => {
                close(e);
            }}
            className={"absolute top-0 h-full w-full backdrop-brightness-50 z-30"}>
            <div className={"overflow-y-hidden overflow-x-hidden absolute bottom-0 top-[3.5rem] left-0 z-10 w-full "}>
                <div className={"w-full bg-transparent text-blue-600 font-bold flex flex-col max-w-xl h-full"}>
                    <div className={"flex-grow mt-4 bg-blue-300 rounded-t-[48px] relative z-0"}>
                        <div className={"mt-1 rounded-t-[48px] relative z-0 bg-blue-200 h-full"}>
                            <div className={"flex flex-col h-full"}>
                                <div className="p-2 w-full ">
                                    <div className="relative p-4 md:p-5 rounded-t">
                                        <p className={"w-full  text-center text-2xl text-blue-600"}> {title} </p>
                                        <button type="button"
                                                className="absolute top-4 right-4 text-blue-600 w-8 h-8"
                                                onClick={(e) => {
                                                    close(e);
                                                }}>
                                            <svg className="w-3 h-3" aria-hidden="true"
                                                 xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                                <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round"
                                                      strokeWidth="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                                            </svg>
                                            <span className="sr-only">Close modal</span>
                                        </button>
                                    </div>
                                </div>
                                {children}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>,
        document.getElementById("root")!
    )
}

