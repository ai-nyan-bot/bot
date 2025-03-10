import React, {FC} from "react";
import {Action, ActionType} from "@types";
import {SelectActionType} from "@components/editor/action/type.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card.tsx";

export type ActionEditorProps = {
    action: Action;
    onChange(action: Action): void;
}

export const ActionEditor: FC<ActionEditorProps> = ({action, onChange}) => {

    return (
        <Card className="w-full shadow-none border-0">
            <CardHeader>
                <CardTitle className="font-semibold text-blue-600 flex items-center">THEN</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="max-w-4xl mx-auto space-y-6">
                    <div className="border-l-4 border-blue-500 pl-4">
                        <SelectActionType
                            defaultActionType={ActionType.NOTIFY_TELEGRAM}
                            supported={[
                                ActionType.NOTIFY_TELEGRAM
                            ]}
                            onChange={(type) => {
                            }}
                        />
                        {/*{action.type === ActionType.NOTIFY_TELEGRAM && <Notify action={action} onChange={(action) => {*/}
                        {/*    onChange(action)*/}
                        {/*}}/>}*/}
                    </div>
                </div>
            </CardContent>
        </Card>
    );
}