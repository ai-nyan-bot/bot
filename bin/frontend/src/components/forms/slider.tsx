// @ts-ignore
import RangeSlider from 'react-range-slider-input'
import 'react-range-slider-input/dist/style.css'

const Slider = ({
                    value,
                    max,
                    onChange,
                    step,
                }: {
    value: number[]
    max: number
    onChange: (x: number[]) => void
    step: number
}) => {
    const handleSliderChange = (v: number[]) => {
        onChange(v)
    }

    return (
        <RangeSlider
            id="range-slider-gradient"
            onInput={handleSliderChange}
            min={1}
            max={max}
            value={value}
            step={step}
        />
    )
}

export default Slider
