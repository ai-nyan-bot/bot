import {FC, useState} from "react";
import {ActionNotify} from "@types";
import {Button} from "@components/ui/button.tsx";
import {Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle} from "@components/ui/dialog.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";
import {Input} from "@components/ui/input.tsx";

export type NotifyProps = {
    action: ActionNotify;
    onChange: (action: ActionNotify) => void;
};

export const Notify: FC<NotifyProps> = ({action, onChange}) => {
    return (
        <>
            {/*<p>{JSON.stringify(action)}</p>*/}
            <TelegramButtons/>
        </>
    )
}

type ButtonAction = 'None' | 'Buy' | 'Sell'

type ButtonConfig = {
    action: ButtonAction;
    value: string;
}

const defaultConfigs: Array<ButtonConfig> = Array(6).fill({
    action: 'None',
    value: ''
});

const TelegramButtons: FC = () => {
    const [configs, setConfigs] = useState<(ButtonConfig)[]>(defaultConfigs);
    const [open, setOpen] = useState(false);
    const [currentIndex, setCurrentIndex] = useState<number | null>(null);
    const [currentAction, setCurrentAction] = useState<ButtonAction>('None');
    const [currentValue, setCurrentValue] = useState('');

    const handleClick = (index: number) => {
        setCurrentIndex(index);
        const existingConfig = configs[index]!;

        setCurrentAction(existingConfig.action);
        setCurrentValue(existingConfig?.value || '');
        setOpen(true);
    };

    const handleSave = () => {
        if (currentIndex !== null && currentAction) {
            const newConfigs = [...configs];
            newConfigs[currentIndex] = {action: currentAction, value: currentValue};
            setConfigs(newConfigs);
            setOpen(false);
        }
    };


    const suggestedValues = currentAction === 'Buy' ? ['0.1', '0.5', '1'] : ['25%', '50%', '100%'];
    return (
        <div className="grid grid-cols-3 gap-4 pt-4">
            {configs.map((config, index) => {
                    if (config.action === 'None') {
                        return (
                            <Button
                                key={index}
                                onClick={() => handleClick(index)}
                                variant={'secondary'}
                            >
                                ?
                            </Button>
                        )
                    }
                    return (
                        <Button
                            key={index}
                            onClick={() => handleClick(index)}
                            variant={'outline'}
                        >
                            {`${config.action} ${config.value}`}
                        </Button>
                    )
                }
            )}

            <Dialog open={open} onOpenChange={setOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Configure Button</DialogTitle>
                    </DialogHeader>

                    <Select value={currentAction}
                            onValueChange={(value) => setCurrentAction(value as 'Buy' | 'Sell')}
                    >
                        <SelectTrigger>
                            <SelectValue placeholder="Select Action"/>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="None">Nothing</SelectItem>
                            <SelectItem value="Buy">Buy</SelectItem>
                            <SelectItem value="Sell">Sell</SelectItem>
                        </SelectContent>
                    </Select>

                    {currentAction && (
                        <div className="mt-4">
                            <div className="flex gap-2 mb-2">
                                {suggestedValues.map((val) => (
                                    <Button key={val} variant="outline" onClick={() => setCurrentValue(val)}>
                                        {val}
                                    </Button>
                                ))}
                            </div>
                            <Input
                                placeholder={currentAction === 'Buy' ? 'Amount in SOL' : 'Percentage'}
                                value={currentValue}
                                onChange={(e) => setCurrentValue(e.target.value)}
                            />
                        </div>
                    )}

                    <DialogFooter>
                        <Button onClick={handleSave}>Save</Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};
