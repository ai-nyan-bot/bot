import {FC, useEffect, useRef, useState} from "react";
import {Input} from "@components/ui/input";
import {Label} from "@components/ui/label";
import {Card, CardContent, CardHeader, CardTitle} from "@components/ui/card";
import {useRuleUpdate} from "@hooks/rule.ts";

export type RuleDetailFormProps = {
    id: string;
    name: string;
};

export const RuleDetailForm: FC<RuleDetailFormProps> = ({id, name}) => {
    const [updateRule, updatedRule, loading, error] = useRuleUpdate();
    const [ruleName, setRuleName] = useState(name);
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        const handler = setTimeout(() => {
            if (ruleName.trim() && ruleName !== name) {
                updateRule(id, {name: ruleName});
            }
        }, 100);

        return () => clearTimeout(handler);
    }, [ruleName, name]);

    useEffect(() => {
        if (updatedRule) {
            if (inputRef.current && document.activeElement !== inputRef.current) {
                inputRef.current.focus();
            }
        }
    }, [updatedRule]);

    return (
        <Card className="w-full">
            <CardHeader>
                <CardTitle>Rule {ruleName}</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="space-y-2">
                    <Label htmlFor="rule-name">Name of your rule</Label>
                    <Input
                        ref={inputRef}
                        id="rule-name"
                        type="text"
                        placeholder="Enter rule name"
                        value={ruleName}
                        onChange={(e) => setRuleName(e.target.value)}
                        disabled={loading}
                    />
                </div>
            </CardContent>
        </Card>
    );
};