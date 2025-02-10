import React, {FC} from "react";
import {imageUrl} from "@utils";

export type SVGImageGradient = "None" | "Lollipop"

export type SVGImageProps = {
    gradient?: SVGImageGradient;
    className?: string;
    src: string
}
export const SVGImage: FC<SVGImageProps> = ({gradient, className, src}) => {
    return (
        <svg className={className} focusable="false">
            {(!gradient || gradient === "Lollipop") && (<GradientLollipop/>)}
            {(gradient === "None") && (<GradientNone/>)}
            <svg fill={`url(#${gradient || "Lollipop"})`}>
                <use href={`${imageUrl(src)}#img`}/>
            </svg>
        </svg>
    );
}

const GradientNone = () => (
    <linearGradient id="None" x2="1" y2="1">
        <stop offset="0%" stopColor="#dddddd"/>
        <stop offset="25%" stopColor="#dddddd"/>
        <stop offset="75%" stopColor="#dddddd"/>
    </linearGradient>
)

const GradientLollipop = () => (
    <linearGradient id="Lollipop" x2="1" y2="1">
        <stop offset="0%" stopColor="#a770ef"/>
        <stop offset="25%" stopColor="#cf8bf3"/>
        <stop offset="75%" stopColor="#fdb99b"/>
    </linearGradient>
)