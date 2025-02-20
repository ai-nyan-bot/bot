import React, {FC, useState} from "react";
import {Button} from "@components/ui/button.tsx";
import {ActionNotifyTelegram, TelegramButtonAction, TelegramButtonConfig, ValueNumber, ValueType} from "@types";
import {Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle} from "@components/ui/dialog.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@components/ui/select.tsx";
import {ValueNumberInput} from "@components/editor/value.tsx";


export type TelegramButtonsProps = {
    action: ActionNotifyTelegram;
}

export const TelegramButtons: FC<TelegramButtonsProps> = ({action}) => {
    const [configs, setConfigs] = useState<(TelegramButtonConfig)[]>(action.buttons);

    const handleUpdate = (index: number, config: TelegramButtonConfig) => {
        const newConfigs = [...configs];
        newConfigs[index] = config;
        setConfigs(newConfigs);
    };

    return (
        <div className="grid grid-cols-3 gap-4 pt-4">
            {configs.map((config, index) => {
                return (
                    <TelegramButton
                        key={index}
                        index={index}
                        config={config}
                        onChange={(config) => handleUpdate(index, config)}
                    />
                )
            })}
        </div>
    );
};

export type TelegramButtonProps = {
    index: number,
    config: TelegramButtonConfig
    onChange: (config: TelegramButtonConfig) => void;
}

export const TelegramButton: FC<TelegramButtonProps> = ({index, config, onChange}) => {
    const [open, setOpen] = useState(false);

    const [action, setAction] = useState<TelegramButtonAction>(config.action);
    const [value, setValue] = useState<ValueNumber | undefined>(config.value);

    const handleActionUpdate = (action: TelegramButtonAction) => {
        setAction(action)
        setValue(undefined)
    }

    const handleValueUpdate = (value: ValueNumber) => {
        if (!isNaN(value.value)) {
            setValue(value);
        } else {
            setValue(undefined)
        }
    }

    return (
        <div className="w-full">
            {config.action === 'NONE' && (
                <Button
                    key={index}
                    className={'w-full'}
                    onClick={() => setOpen(true)}
                    variant={'secondary'}
                >
                    ?
                </Button>
            )}

            {config.action === 'BUY' && (
                <Button
                    key={index}
                    className={'w-full'}
                    onClick={() => setOpen(true)}
                    variant={'outline'}
                >
                    {`Buy ${config.value!.value} SOL`}
                </Button>
            )}

            {config.action === 'SELL' && (
                <Button
                    key={index}
                    className={'w-full'}
                    onClick={() => setOpen(true)}
                    variant={'outline'}
                >
                    {`Sell ${config.value!.value}%`}
                </Button>
            )}

            <Dialog open={open} onOpenChange={setOpen}
            >
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Button {index + 1}</DialogTitle>
                    </DialogHeader>

                    <Select value={action} onValueChange={(value) => {
                        handleActionUpdate(value as TelegramButtonAction)
                    }}
                    >
                        <SelectTrigger>
                            <SelectValue placeholder="Select Action"/>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="NONE">Nothing</SelectItem>
                            <SelectItem value="BUY">Buy</SelectItem>
                            <SelectItem value="SELL">Sell</SelectItem>
                        </SelectContent>
                    </Select>

                    {action === 'BUY' && (
                        <div className="mt-4">
                            <div className="flex gap-2 mb-2">
                                {['0.1', '0.5', '1'].map((val) => (
                                    <Button
                                        key={val}
                                        variant="outline"
                                        onClick={() => {
                                            setValue({
                                                type: ValueType.SOL,
                                                value: parseFloat(val)
                                            })
                                        }}
                                    >
                                        {val} SOL
                                    </Button>
                                ))}
                            </div>

                            <ValueNumberInput
                                supported={[ValueType.SOL]}
                                value={value}
                                onChange={handleValueUpdate}
                            />
                        </div>
                    )}

                    {action === 'SELL' && (
                        <div className="mt-4">
                            <div className="flex gap-2 mb-2">
                                {['25%', '50%', '100%'].map((val) => (
                                    <Button
                                        key={val} variant="outline"
                                        onClick={() => setValue({
                                            type: ValueType.PERCENT,
                                            value: parseFloat(val)
                                        })}
                                    >
                                        {val}
                                    </Button>
                                ))}
                            </div>

                            <ValueNumberInput
                                supported={[ValueType.PERCENT]}
                                value={value}
                                onChange={handleValueUpdate}
                            />
                        </div>
                    )}

                    <DialogFooter>
                        <Button
                            onClick={() => {
                                if (action !== 'NONE' && value || action === 'NONE' && !value) {
                                    onChange({action, value});
                                    setOpen(false);
                                }
                            }}
                        >Update</Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};
