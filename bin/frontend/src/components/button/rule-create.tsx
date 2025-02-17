import {useRuleCreate} from "@hooks/rule.ts";
import {FC, useEffect, useRef} from "react";
import {useNavigate} from "react-router-dom";
import {Button} from "@components/ui/button.tsx";

export type RuleCreateButtonProps = {}

export const RuleCreateButton: FC<RuleCreateButtonProps> = ({}) => {
    const [createRule, createdRule, loading, error] = useRuleCreate();
    const abortControllerRef = useRef<AbortController | null>(null);
    const navigate = useNavigate();

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

    useEffect(() => {
        if (createdRule) {
            navigate(`/rules/${createdRule.id}`)
        }
    }, [createdRule]);

    return (
        <Button className="w-full bg-green-500 text-white" onClick={handleClick} disabled={loading}>+ Rule</Button>
    )
}