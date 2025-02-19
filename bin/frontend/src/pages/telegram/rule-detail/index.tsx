import React, {useEffect, useState} from "react";
import {Editor} from "@components/editor";
import {useRuleGet, useRuleUpdate} from "@hooks/rule.ts";
import {useParams} from "react-router-dom";
import {Sequence} from "@types";
import {injectId, uuidv4} from "@utils";
import {RuleDetailForm} from "@components/form";
import {Button} from "@components/ui/button.tsx";


const TelegramRuleDetailPage: React.FC = () => {
    const {id} = useParams();
    const [getRule, rule, loading, ruleError] = useRuleGet();
    const [updateRule, _, updating] = useRuleUpdate();
    const [ruleName, setRuleName] = useState<string>();
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
                onNameChanged={setRuleName}
            />

            <Editor
                sequence={sequence}
                onChange={(sequence) => {
                    setSequence(sequence)
                }}
            />

            <Button onClick={() => {
                updateRule(id, {sequence})
            }} disabled={updating}>Update</Button>
        </div>
    );
};

export default TelegramRuleDetailPage;