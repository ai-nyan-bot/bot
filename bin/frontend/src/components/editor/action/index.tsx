import {FC} from "react";
import {Action} from "@types";

export type ActionEditorProps = {
    action: Action;

}

export const ActionEditor: FC<ActionEditorProps> = ({action}) => {
    return (
        <>
            <h1> Action </h1>
            <p>{JSON.stringify(action)}</p>
        </>
    );
}