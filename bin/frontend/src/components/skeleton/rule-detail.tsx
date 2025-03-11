import {Skeleton} from "@components/ui/skeleton.tsx";
import React from "react";

export const RuleDetailSkeleton = () => (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <Skeleton key={1} className="h-48 w-full rounded-lg">
            <div className={"p-24 w-full text-center"}>
                <span className="animate-spin mr-2">⏳</span> Loading Rule
            </div>
        </Skeleton>
        <Skeleton key={2} className="h-48 w-full rounded-lg"/>
        <Skeleton key={3} className="h-48 w-full rounded-lg"/>
    </div>
)