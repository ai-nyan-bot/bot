type Values = string | number

interface ButtonGroupProps<T extends Values> {
    activeValue: T
    className?: string
    disabled?: boolean
    onChange: (x: T) => void
    unit?: string
    values: T[]
    names?: Array<string>
    large?: boolean
}

const ButtonGroup = <T extends Values>({
                                           activeValue,
                                           className,
                                           disabled,
                                           unit,
                                           values,
                                           onChange,
                                           names,
                                           large,
                                       }: ButtonGroupProps<T>) => {
    return (
        <div
            className={`bg-th-bkg-2 ${disabled ? 'opacity-50' : ''} ${
                large ? 'rounded-lg' : 'rounded-md'
            }`}
        >
            <div className="relative flex">
                {activeValue && values.includes(activeValue) ? (
                    <div
                        className={`default-transition absolute left-0 top-0 h-full transform ${
                            activeValue === 'Buy'
                                ? 'bg-th-up'
                                : activeValue === 'Sell'
                                    ? 'bg-th-down'
                                    : 'bg-th-bkg-4'
                        } ${large ? 'rounded-lg' : 'rounded-md'}`}
                        style={{
                            transform: `translateX(${
                                values.findIndex((v) => v === activeValue) * 100
                            }%)`,
                            width: `${100 / values.length}%`,
                        }}
                    />
                ) : null}
                {values.map((v, i) => (
                    <button
                        className={`${className} default-transition relative w-1/2 cursor-pointer px-3 text-center disabled:cursor-not-allowed ${
                            large ? 'h-12 rounded-md text-sm' : 'h-7 rounded text-xs'
                        } font-semibold
              ${
                            v === activeValue
                                ? v === 'Buy' || v === 'Sell'
                                    ? 'text-th-button-text'
                                    : `text-th-fgd-1`
                                : `text-th-fgd-4 md:hover:text-th-fgd-1`
                        }
            `}
                        disabled={disabled}
                        key={`${v}${i}`}
                        onClick={() => onChange(v)}
                        style={{
                            width: `${100 / values.length}%`,
                        }}
                        type="button"
                    >
                        {names ? (unit ? names[i] + unit : names[i]) : unit ? v + unit : v}
                    </button>
                ))}
            </div>
        </div>
    )
}

export default ButtonGroup
