import {FC, useEffect, useState} from "react";
import {Input} from "@components/ui/input";
import {Label} from "@components/ui/label";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card";
import {useRuleUpdate} from "@hooks/rule.ts";

export type RuleDetailFormProps = {
    id: string;
    name: string;
};

export const RuleDetailForm: FC<RuleDetailFormProps> = ({id, name}) => {
    const [updateRule, , ,] = useRuleUpdate();
    const [ruleName, setRuleName] = useState(name);

    useEffect(() => {
        const handler = setTimeout(() => {
            if (ruleName.trim() && ruleName !== name) {
                updateRule(id, {name: ruleName});
            }
        }, 1_000);

        return () => clearTimeout(handler);
    }, [id, ruleName, name]);

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
                    onChange={(e) => setRuleName(e.target.value)}
                />
            </CardContent>
        </Card>
    );
};