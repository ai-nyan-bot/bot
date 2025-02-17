import {useNavigate} from "react-router-dom";
import {Card} from "@components/ui/card.tsx";
import {useRuleList} from "@hooks/rule.ts";
import React, {useEffect} from "react";
import {Skeleton} from "@components/ui/skeleton.tsx";

const Rule = ({id, name}: { id: number; name: string }) => {
    const navigate = useNavigate();
    return (
        <Card
            className="p-4 border bg-white hover:bg-gray-100 transition-all cursor-pointer shadow-sm rounded-lg"
            onClick={() => navigate(`/rules/${id}`)}
        >
            <span className="text-lg font-medium">{name}</span>
        </Card>
    );
};

export const RuleList = () => {
    const [listRules, response, loading, error] = useRuleList();

    useEffect(() => {
        const abortController = new AbortController();
        listRules(abortController);
        return () => abortController.abort();
    }, [listRules]);

    if (!response || loading) {
        return (
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {[...Array(6)].map((_, index) => (
                    <Skeleton key={index} className="h-16 w-full rounded-lg"/>
                ))}
            </div>
        );
    }

    if (error) {
        return <p className="text-red-500">Error loading rules. Please try again.</p>;
    }

    if (response.rules.length === 0) {
        return (
            <div className={'pt-32 flex flex-col space-y-10 items-center justify-center'}>
                <p className="text-gray-500 text-center">No rules found</p>
            </div>
        );
    }

    return (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {response.rules.map((r) => (
                <Rule key={r.id} id={r.id} name={r.name}/>
            ))}
        </div>
    );
};