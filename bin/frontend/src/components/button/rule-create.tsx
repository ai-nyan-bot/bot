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

    return (
        <>
            <Button className="w-full bg-green-500 text-white" onClick={handleCreateClick} disabled={loading}>
                + Rule
            </Button>

            {/* Modal */}
            <Dialog open={isModalOpen} onOpenChange={setIsModalOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Enter Rule Name</DialogTitle>
                    </DialogHeader>
                    <Input
                        type="text"
                        placeholder="Rule Name"
                        value={ruleName}
                        onChange={(e) => setRuleName(e.target.value)}
                        className="mt-2"
                    />
                    <DialogFooter>

                        <Button variant="outline" onClick={() => {
                            setRuleName('');
                            setIsModalOpen(false);
                        }}>Cancel</Button>

                        <Button variant={'default'} onClick={handleConfirmCreate} disabled={!ruleName.trim()}>
                            Create
                        </Button>

                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </>
    );
};
