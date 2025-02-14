import React, {useEffect, useState} from "react";
import {Editor} from "@components/editor";
import {useRuleGet, useRuleUpdate} from "@hooks/rule.ts";
import {useParams} from "react-router-dom";
import {Button} from "@components/ui/button.tsx";
import {Sequence} from "@types";


export const RuleDetailPage: React.FC = () => {
    const {id} = useParams();
    const [getRule, rule, loading, ruleError] = useRuleGet();
    const [updateRule] = useRuleUpdate();
    const [sequence, setSequence] = useState<Sequence>();

    useEffect(() => {
        if (!id) return;

        const abortController = new AbortController();
        getRule(id, abortController);
        return () => {
            abortController.abort()
        }
    }, [id, getRule]);

    useEffect(() => {
        setSequence(rule?.sequence)
    }, [rule]);

    if (!id) {
        return <div>Invalid id</div>;
    }

    if (loading || rule == null || sequence == null) {
        return <h1>Loading rule...</h1>;
    }

    return (
        <div className="max-w-4xl mx-auto space-y-6">
            <Editor
                sequence={rule.sequence}
                onChange={(sequence) => {
                    console.log("update sequence", sequence);
                    setSequence(sequence)
                }}
            />
            {JSON.stringify(sequence, null, 2)}
            <Button className="w-full bg-green-500 text-white" onClick={() => {
                updateRule(id, {
                    name: rule?.name,
                    sequence
                })
            }}>Update Rule</Button>
        </div>
    );
};