import {useRuleList} from "@hooks/rule.ts";
import {useEffect} from "react";
import {useNavigate} from "react-router-dom";
import {Card} from "@app/components/ui/card";
import {RuleCreateButton} from "@components/button";


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
            <RuleCreateButton/>
        </div>
    )
}

export default TelegramRuleListPage;