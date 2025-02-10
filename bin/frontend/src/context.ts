import {createContext} from "react";
import {AppAction, AppState} from "@states/app";
import {AudioAction, AudioState} from "@states/audio";
import {ModalAction, ModalState} from "@states/modal";

interface Client {
}

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextAudioState = createContext<AudioState>();
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextAudioDispatch = createContext<Dispatch<AudioAction>>();

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextAppState = createContext<AppState>();
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextAppDispatch = createContext<Dispatch<AppAction>>();

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextClient = createContext<Client>();

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextModalState = createContext<ModalState>();
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const ContextModalDispatch = createContext<Dispatch<ModalAction>>();