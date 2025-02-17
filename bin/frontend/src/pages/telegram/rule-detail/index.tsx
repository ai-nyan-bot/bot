import React, {useEffect, useState} from "react";
import {Editor} from "@components/editor";
import {useRuleGet, useRuleUpdate} from "@hooks/rule.ts";
import {useParams} from "react-router-dom";
import {Sequence} from "@types";
import {injectId, uuidv4} from "@utils";
import {RuleDetailForm} from "@components/form";


const TelegramRuleDetailPage: React.FC = () => {
    const {id} = useParams();
    const [getRule, rule, loading, ruleError] = useRuleGet();
    const [updateRule] = useRuleUpdate();
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
        setSequence(injectedSequence)
    }, [rule]);

    if (!id) {
        return <div>Invalid id</div>;
    }

    if (loading || rule == null || sequence == null) {
        return <h1>Loading rule...</h1>;
    }

    return (
        <div className="w-full">
            <RuleDetailForm
                id={id}
                name={rule.name}
            />

            <Editor
                sequence={sequence}
                onChange={(sequence) => {
                    updateRule(id, {sequence})
                }}
            />
        </div>
    );
};

export default TelegramRuleDetailPage;