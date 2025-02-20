import {FC, useState} from "react";
import {Button} from "@components/ui/button.tsx";
import {Dialog, DialogContent, DialogFooter, DialogHeader} from "@components/ui/dialog.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";
import {Input} from "@components/ui/input.tsx";
import {ActionNotifyTelegram, TelegramButtonAction, TelegramButtonConfig, Value} from "@types";


export type TelegramButtonsProps = {
    action: ActionNotifyTelegram;
}

export const TelegramButtons: FC<TelegramButtonsProps> = ({action}) => {
    const [configs, setConfigs] = useState<(TelegramButtonConfig)[]>(action.buttons);
    const [open, setOpen] = useState(false);
    const [currentIndex, setCurrentIndex] = useState<number | null>(null);
    const [currentAction, setCurrentAction] = useState<TelegramButtonAction>('NONE');
    const [currentValue, setCurrentValue] = useState<Value>();

    const handleClick = (index: number) => {
        setCurrentIndex(index);
        const existingConfig = configs[index]!;

        setCurrentAction(existingConfig.action);
        setCurrentValue(existingConfig?.value);
        setOpen(true);
    };

    const handleSave = () => {
        if (currentIndex !== null && currentAction) {
            const newConfigs = [...configs];
            // newConfigs[currentIndex] = {action: currentAction, value: currentValue};
            // setConfigs(newConfigs);
            // setOpen(false);
        }
    };

    return (
        <div className="grid grid-cols-3 gap-4 pt-4">
            {configs.map((config, index) => {
                    if (config.action === 'NONE') {
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
                        {/*<DialogTitle>Button {currentIndex + 1}</DialogTitle>*/}
                    </DialogHeader>

                    <Select value={currentAction}
                            onValueChange={(value) => setCurrentAction(value as TelegramButtonAction)}
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

                    {currentAction === 'BUY' && (
                        <div className="mt-4">
                            <div className="flex gap-2 mb-2">
                                {['0.1', '0.5', '1'].map((val) => (
                                    <Button
                                        key={val} variant="outline"
                                        // onClick={() => setCurrentValue(val)}
                                    >
                                        {val}
                                    </Button>
                                ))}
                            </div>
                            <Input
                                placeholder={'Amount in SOL'}
                                // value={currentValue}
                                // onChange={(e) => setCurrentValue(e.target.value)}
                            />
                        </div>
                    )}

                    {currentAction === 'SELL' && (
                        <div className="mt-4">
                            <div className="flex gap-2 mb-2">
                                {['25%', '50%', '100%'].map((val) => (
                                    <Button
                                        key={val} variant="outline"
                                        // onClick={() => setCurrentValue(val)}
                                    >
                                        {val}
                                    </Button>
                                ))}
                            </div>
                            <Input
                                placeholder={'Percentage'}
                                // value={currentValue}
                                // onChange={(e) => setCurrentValue(e.target.value)}
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
