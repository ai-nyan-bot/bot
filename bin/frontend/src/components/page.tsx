import React, {FC} from "react";

type PageProps = {
    children: React.ReactNode;
}

export const Page: FC<PageProps> = ({children}) => {
    return (
        <div className={"overflow-y-hidden h-screen w-full flex justify-center"}>
            <div className={"w-full bg-blue-300 text-blue-600 h-screen font-bold flex flex-col max-w-xl"}>
                {children}
            </div>
        </div>
    )
}
