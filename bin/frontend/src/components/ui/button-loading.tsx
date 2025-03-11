import * as React from "react"
import {FC} from "react"
import {Button, buttonVariants} from "./button";
import type {VariantProps} from "class-variance-authority";

export type LoadingButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & VariantProps<typeof buttonVariants> & {
    text: String,
    loadingText: String,
    loading: boolean
    onClick: () => void
}

export const LoadingButton: FC<LoadingButtonProps> =
    ({text, loadingText, loading, onClick, ...props}) => {
        return (
            <Button
                onClick={onClick}
                disabled={loading}
                {...props}
            >
                {loading ? (<><span className="animate-spin mr-2">⏳</span>{loadingText}</>) : (text)}
            </Button>
        );
    }