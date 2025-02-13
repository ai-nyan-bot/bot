import {useRuleCreate, useRuleList} from "@hooks/rule.ts";
import {useEffect, useRef} from "react";
import {useNavigate} from "react-router-dom";


const Rule = ({id, name}: { id: number, name: string }) => {
    const navigate = useNavigate();
    return (
        <div onClick={(_) => {
            navigate(`/rules/${id}`);
        }}>
            {id} - {name}
        </div>
    )
}

const CreateRuleButton = () => {
    const [createRule, response, loading, error] = useRuleCreate();
    const abortControllerRef = useRef<AbortController | null>(null);

    const handleClick = () => {
        if (abortControllerRef.current) {
            abortControllerRef.current.abort();
        }
        const newAbortController = new AbortController();
        abortControllerRef.current = newAbortController;
        // createRule({}, newAbortController);
    };

    useEffect(() => {
        return () => {
            if (abortControllerRef.current) {
                abortControllerRef.current.abort();
            }
        };
    }, []);

    return (
        <button onClick={handleClick} disabled={loading}>
            Create Rule
        </button>
    )
}

export const RuleListPage = () => {
    const [listRules, response, rulesLoading, error] = useRuleList();

    useEffect(() => {
        const abortController = new AbortController();
        listRules(abortController);
        return () => {
            abortController.abort();
        }
    }, [listRules]);

    if (rulesLoading || response == null) {
        return <h1> Loading rules </h1>
    }

    let rules = response.rules.map(r => <Rule key={r.id} id={r.id} name={r.name}/>);

    return (
        <>
            <h1> Rules </h1>
            {rules}
            <CreateRuleButton/>
        </>
    )
}