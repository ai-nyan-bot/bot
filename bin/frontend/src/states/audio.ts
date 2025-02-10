import {LocalStorage} from "@states/local";

export type  AudioTrack =
    | 'None'
    // Played when player places a bet
    | 'Bet_Placed'
    // Played when button gets clicked
    | 'Button_Click'
    // Played when player looses rug tokens
    | 'Lost_Rugs'
    // Player selected a value with a slider
    | 'Slider_Selected'
    // Played when player wins rug tokens
    | 'Won_Rugs'

type GfxState = {
    active: boolean;
    volume: number;
    track: AudioTrack;
    delayInMs: number;
}

type BackgroundState = {
    active: boolean;
    volume: number;
}

export type AudioState = {
    gfx: GfxState;
    background: BackgroundState;
};

export type AudioAction =
    | { type: 'AUDIO_PLAY', track: AudioTrack, delayInMs?: number }
    | { type: 'AUDIO_TOGGLE_BACKGROUND' }
    | { type: 'AUDIO_TOGGLE_GFX' }
    | { type: 'AUDIO_DONE' }

export const audioReducer = (state: AudioState, action: AudioAction): AudioState => {
    switch (action.type) {
        case "AUDIO_DONE": {
            return {
                ...state,
                gfx: {
                    ...state.gfx,
                    track: 'None',
                    delayInMs: 0
                } satisfies GfxState
            }
        }
        case "AUDIO_PLAY":
            return {
                ...state,
                gfx: {
                    ...state.gfx,
                    track: action.track,
                    delayInMs: action.delayInMs || 0,
                } satisfies GfxState
            }
        case "AUDIO_TOGGLE_BACKGROUND": {
            return {
                ...state,
                background: {
                    ...state.background,
                    active: !state.background.active
                }
            }
        }
        case "AUDIO_TOGGLE_GFX": {
            return {
                ...state,
                gfx: {
                    ...state.gfx,
                    active: !state.gfx.active
                }
            }
        }
        default:
            throw new Error(`Not supported action type: ${action}`);
    }
}

export const audioInitialState = (localStorage: LocalStorage): AudioState => ({
    gfx: {
        // ...localStorage.audio.gfx,
        active: false,
        volume: 0,
        track: "None",
        delayInMs: 0
    },
    // background: {...localStorage.audio.background}
    background: {
        active: false,
        volume: 0
    }
})


