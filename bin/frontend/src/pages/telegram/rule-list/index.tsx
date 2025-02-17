import {RuleCreateButton} from "@components/button";
import {RuleList} from "@components/list";


const TelegramRuleListPage = () => {
    return (
        <div className="w-full">
            <h1> Rules </h1>
            <RuleList/>
            <RuleCreateButton/>
        </div>
    )
}

export default TelegramRuleListPage;