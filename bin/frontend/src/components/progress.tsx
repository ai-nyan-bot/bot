import {FC} from "react";
import './progress.css'

export type ProgressProps = {
    value?: number;
    maxValue?: number;
}

export const Progress: FC<ProgressProps> = ({value, maxValue}) => {
    return (<>
        <progress value={value || 0} max={maxValue || 100} className={"progress-2"}></progress>
    </>);
}