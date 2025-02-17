import {useRuleCreate, useRuleList} from "@hooks/rule.ts";
import {useEffect, useRef} from "react";
import {useNavigate} from "react-router-dom";
import {Button} from "@components/ui/button.tsx";
import {Card} from "@app/components/ui/card";


const Rule = ({id, name}: { id: number, name: string }) => {
    const navigate = useNavigate();
    return (
        <Card className="p-4 border bg-gray-50 relative" onClick={(_) => {
            navigate(`/rules/${id}`);
        }}>
            {name}
        </Card>
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
        createRule({
            name: 'test',
            sequence: {
                condition: {
                    id: 'root',
                    type: 'AND',
                    conditions: []
                },
                action: {
                    type: 'NOTIFY',
                }
            }

        }, newAbortController);
    };

    useEffect(() => {
        return () => {
            if (abortControllerRef.current) {
                abortControllerRef.current.abort();
            }
        };
    }, []);

    return (
        <Button className="w-full bg-green-500 text-white" onClick={handleClick} disabled={loading}>+ Rule</Button>
        // <button onClick={handleClick} disabled={loading}>
        //     Create Rule
        // </button>
    )
}

const TelegramRuleListPage = () => {
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
        <div className="max-w-4xl mx-auto space-y-6">
            <h1> Rules </h1>
            {rules}
            <CreateRuleButton/>
        </div>
    )
}

export default TelegramRuleListPage;