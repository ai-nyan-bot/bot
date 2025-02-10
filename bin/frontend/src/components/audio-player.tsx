import {useContext, useEffect} from "react";
import {Howl} from "howler";

import {AudioTrack} from "@states/audio";
import {ContextAudioDispatch, ContextAudioState} from "@app/context";
import {audioUrl} from "@utils";

const backgroundSound = new Howl({
    src: [audioUrl("background.ogg")],
    loop: true,
    volume: 0.2
});

export const AudioPlayer = () => {
    const {gfx, background} = useContext(ContextAudioState)
    const dispatch = useContext(ContextAudioDispatch);

    useEffect(() => {
        if (gfx.track !== 'None' && gfx.active) {
            setTimeout(() => {
                const sound = new Howl({
                    src: [loadTrack(gfx.track)],
                    onend: () => {
                        dispatch({type: "AUDIO_DONE"})
                    },
                    volume: gfx.volume
                });
                sound.play()
            }, gfx.delayInMs)
        }

    }, [gfx, dispatch])

    useEffect(() => {
        if (background.active) {
            backgroundSound.play();
            backgroundSound.volume(background.volume);
        } else {
            backgroundSound.stop();
        }
    }, [background.active, background.volume]);

    return null;
}

const loadTrack = (track: AudioTrack) => {
    switch (track) {
        case 'Bet_Placed':
            return audioUrl("won-rugs.ogg");
        case 'Button_Click':
            return audioUrl("click.ogg");
        case 'Lost_Rugs':
            return audioUrl("lost-rugs.ogg");
        case 'Slider_Selected':
            return audioUrl("click.ogg");
        case 'Won_Rugs':
            return audioUrl("won-rugs.ogg");
        default:
            throw Error(`Not supported track: ${track}`)
    }
}