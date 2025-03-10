import {FC, useState} from "react";
import {Input} from "@components/ui/input";
import {Label} from "@components/ui/label";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card";
import {RuleStatus} from "@types";
import {Button} from "@components/ui/button.tsx";
import {cn} from "@utils";

export type RuleDetailFormProps = {
    id: string;
    name: string;
    status: RuleStatus,
    onChange: (name: string, status: RuleStatus) => void;
};

export const RuleDetailForm: FC<RuleDetailFormProps> = ({id, name, status, onChange}) => {
    const [ruleName, setRuleName] = useState(name);

    return (
        <Card className="w-full bg-gray-50 border-0 shadow-none">
            <CardHeader>
                <CardTitle>Rule {ruleName}</CardTitle>
            </CardHeader>
            <CardContent>
                <div className={"flex items-center space-x-2"}>
                    <Label htmlFor="rule-name">Name</Label>
                    <Input
                        key={`rule-name-${id}`}
                        id="rule-name"
                        type="text"
                        placeholder="Enter rule name"
                        value={ruleName}
                        onChange={(e) => {
                            const name = e.target.value;
                            setRuleName(_ => {
                                onChange(name, status);
                                return name;
                            })
                        }}
                    />
                </div>

                <StatusButton id={id} status={status} onChange={(newStatus) => {
                    onChange(name, newStatus);
                }}/>

            </CardContent>
        </Card>
    );
};


type StatusButtonProps = {
    id: string;
    status: RuleStatus;
    onChange: (value: RuleStatus) => void;
}

const StatusButton: FC<StatusButtonProps> = ({id, status, onChange}) => {
    const toggled = status === RuleStatus.ACTIVE || status === RuleStatus.ACTIVE_EXHAUSTED;
    return (
        <div className={"pt-4 flex items-center space-x-2"}>
            <Label htmlFor="rule-status">Status</Label>
            <Button
                key={`rule-status-${id}`}
                id={"rule-status"}
                onClick={() => {
                    switch (status) {
                        case RuleStatus.ACTIVE:
                            onChange(RuleStatus.INACTIVE);
                            break;
                        case RuleStatus.ACTIVE_EXHAUSTED:
                            onChange(RuleStatus.INACTIVE_EXHAUSTED);
                            break;
                        case RuleStatus.INACTIVE:
                            onChange(RuleStatus.ACTIVE);
                            break;
                        case RuleStatus.INACTIVE_EXHAUSTED:
                            onChange(RuleStatus.ACTIVE_EXHAUSTED);
                            break

                    }
                }}
                className={cn(
                    "transition-colors", toggled ? "bg-green-500 hover:bg-green-600" : "bg-gray-500 hover:bg-gray-600"
                )}
            >
                {toggled ? "Active" : "Inactive"}
            </Button>
        </div>

    );
}

