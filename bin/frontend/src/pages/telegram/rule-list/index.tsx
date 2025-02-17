import {RuleCreateButton} from "@components/button";
import {RuleList} from "@components/list";


const TelegramRuleListPage = () => {
    return (
        <div className="max-w-4xl mx-auto p-6 space-y-6">
            <div className="flex items-center justify-between">
                <h1 className="text-2xl font-semibold">Rules</h1>
                <RuleCreateButton/>
            </div>
            <RuleList/>
        </div>
    )
}

export default TelegramRuleListPage;