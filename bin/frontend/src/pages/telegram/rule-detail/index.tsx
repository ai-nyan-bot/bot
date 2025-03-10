import React, {useEffect, useState} from "react";
import {Editor} from "@components/editor";
import {useRuleGet, useRuleUpdate} from "@hooks/rule.ts";
import {useParams} from "react-router-dom";
import {RuleStatus, Sequence} from "@types";
import {injectId, uuidv4} from "@utils";
import {RuleDetailForm} from "@components/form";
import {Button} from "@components/ui/button.tsx";


const TelegramRuleDetailPage: React.FC = () => {
    const {id} = useParams();
    const [getRule, rule, loading, ruleError] = useRuleGet();
    const [updateRule, _, updating] = useRuleUpdate();
    const [ruleName, setRuleName] = useState<string>();
    const [ruleStatus, setRuleStatus] = useState<RuleStatus>();
    const [sequence, setSequence] = useState<Sequence>();

    useEffect(() => {
        // @ts-ignore
        const tg = window.Telegram.WebApp;
        tg.BackButton.show();
        const handleBackClick = () => {
            window.history.back();
        };
        tg.BackButton.onClick(handleBackClick);

        return () => {
            tg.BackButton.hide();
            tg.BackButton.offClick(handleBackClick);
        };
    }, []);


    useEffect(() => {
        if (!id) return;

        const abortController = new AbortController();
        getRule(id, abortController);
        return () => {
            abortController.abort()
        }
    }, [id, getRule]);

    useEffect(() => {
        let injectedSequence = injectId(rule?.sequence, uuidv4) as Sequence;
        setRuleName(rule?.name)
        setRuleStatus(rule?.status)
        setSequence(injectedSequence)
    }, [rule]);

    if (!id) {
        return <div>Invalid id</div>;
    }

    if (loading || rule == null || sequence == null) {
        return <h1>Loading rule...</h1>;
    }

    return (
        <div className="w-full flex flex-col space-y-2 bg-zinc-50 p-2">
            <RuleDetailForm
                id={id}
                name={ruleName || ''}
                status={ruleStatus || RuleStatus.INACTIVE}
                onChange={(name, status) => {
                    setRuleName(name);
                    setRuleStatus(status);
                }}
            />

            <Editor
                sequence={sequence}
                onChange={(sequence) => {
                    setSequence(sequence)
                }}
            />

            <div className={"w-full sticky bottom-4 flex justify-center"}>
                <Button
                    className={"w-full"}
                    onClick={() => {
                        updateRule(id, {name: ruleName, status: ruleStatus, sequence})
                    }} disabled={updating}>Update</Button>
            </div>
        </div>
    );
};

export default TelegramRuleDetailPage;