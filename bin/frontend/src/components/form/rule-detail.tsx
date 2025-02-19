import {FC, useState} from "react";
import {Input} from "@components/ui/input";
import {Label} from "@components/ui/label";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card";

export type RuleDetailFormProps = {
    id: string;
    name: string;
    onNameChanged: (name: string) => void;
};

export const RuleDetailForm: FC<RuleDetailFormProps> = ({id, name, onNameChanged}) => {
    const [ruleName, setRuleName] = useState(name);
    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle>Rule {ruleName}</CardTitle>
            </CardHeader>
            <CardContent>
                <Label htmlFor="rule-name">Name of your rule</Label>
                <Input
                    key={id}
                    id="rule-name"
                    type="text"
                    placeholder="Enter rule name"
                    value={ruleName}
                    onChange={(e) => {
                        setRuleName(e.target.value)
                        onNameChanged(e.target.value)
                    }}
                />
            </CardContent>
        </Card>
    );
};