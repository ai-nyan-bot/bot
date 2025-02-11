import React, {useEffect} from "react";
import {useRuleGet} from "@hooks/rule.ts";
import {useParams} from "react-router-dom";

export const RuleDetailPage: React.FC = () => {
    let {id} = useParams();
    if (!id) return <>ERROR</>;

    const [getRule, response, ruleLoading, error] = useRuleGet();

    useEffect(() => {
        const abortController = new AbortController();
        getRule(id, abortController);
        return () => {
            abortController.abort();
        }
    }, [getRule, id]);

    if (ruleLoading || response == null) {
        return <>Loading</>
    }


    return (
        <div>
            Rule Details Page
            {response.id} - {response.name}
        </div>
    )
}

