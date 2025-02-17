import {useRuleCreate} from "@hooks/rule.ts";
import {FC, useEffect, useRef, useState} from "react";
import {useNavigate} from "react-router-dom";
import {Button} from "@components/ui/button.tsx";
import {Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle} from "@components/ui/dialog";
import {Input} from "@components/ui/input";

export type RuleCreateButtonProps = {}

export const RuleCreateButton: FC<RuleCreateButtonProps> = ({}) => {
    const [createRule, createdRule, loading, error] = useRuleCreate();
    const abortControllerRef = useRef<AbortController | null>(null);
    const navigate = useNavigate();

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [ruleName, setRuleName] = useState("");

    const handleCreateClick = () => {
        setIsModalOpen(true);
    };

    const handleConfirmCreate = () => {
        if (!ruleName.trim()) return;

        if (abortControllerRef.current) {
            abortControllerRef.current.abort();
        }

        const newAbortController = new AbortController();
        abortControllerRef.current = newAbortController;

        createRule({
            name: ruleName,
            sequence: {
                condition: {
                    id: "root",
                    type: "AND",
                    conditions: []
                },
                action: {
                    type: "NOTIFY"
                }
            }
        }, newAbortController);

        setIsModalOpen(false);
        setRuleName(""); // Reset input
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
            navigate(`/rules/${createdRule.id}`);
        }
    }, [createdRule]);

    const handleClose = () => {
        setRuleName('');
        setIsModalOpen(false);
    }

    return (
        <>
            <Button className="w-full bg-green-500 text-white" onClick={handleCreateClick} disabled={loading}>
                + Rule
            </Button>

            <Dialog open={isModalOpen} onOpenChange={(open) => {
                if (!open) {
                    handleClose()
                }
            }}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle className={"text-zinc-500"}>Name your rule</DialogTitle>
                    </DialogHeader>
                    <Input
                        type="text"
                        placeholder="Name of your rule"
                        value={ruleName}
                        onChange={(e) => setRuleName(e.target.value)}
                        className="mt-2"
                    />
                    <DialogFooter className="flex flex-row justify-end gap-2">
                        <Button variant="outline" onClick={handleClose}>Cancel</Button>
                        <Button variant="default" onClick={handleConfirmCreate} disabled={!ruleName.trim()}>
                            Create
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </>
    );
};
