// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::RuleStatus::InactiveExhausted;
use crate::model::{AuthenticatedUser, Rule, RuleId, RuleName, RuleStatus, Sequence};
use crate::repo;
use crate::service::RuleService;
use common::service::{ServiceError, ServiceResult};
use log::warn;
use RuleStatus::{Active, ActiveExhausted, Archived, ArchivedExhausted, Inactive};

pub struct RuleUpdateCmd {
    pub name: Option<RuleName>,
    pub sequence: Option<Sequence>,
    pub status: Option<RuleStatus>,
}

impl RuleService {
    pub async fn update(
        &self,
        id: impl Into<RuleId>,
        cmd: RuleUpdateCmd,
        user: AuthenticatedUser,
    ) -> ServiceResult<Rule> {
        let mut tx = self.pool.begin().await?;
        let id = id.into();

        let rule = match self.repo.get_by_id(&mut tx, id).await {
            Ok(rule) => rule,
            Err(_) => return Err(ServiceError::not_found("Rule not found")),
        };

        if rule.user != user.id {
            return Err(ServiceError::not_found("Rule not found"));
        }

        let result = self
            .repo
            .update(
                &mut tx,
                repo::RuleUpdateCmd {
                    id,
                    user: user.id,
                    name: cmd.name.unwrap_or(rule.name),
                    sequence: cmd.sequence.unwrap_or(rule.sequence),
                    status: update_status(rule.status, cmd.status),
                },
            )
            .await?;

        tx.commit().await?;
        Ok(result)
    }
}

fn update_status(current: RuleStatus, new: Option<RuleStatus>) -> RuleStatus {
    if let Some(new) = new {
        match (current, new) {
            (Active, Inactive) => Inactive,
            (ActiveExhausted, Inactive) => InactiveExhausted,
            (Active, Archived) => Archived,
            (ActiveExhausted, Archived) => ArchivedExhausted,

            (Inactive, Active) => Active,
            (InactiveExhausted, Active) => ActiveExhausted,
            (Inactive, Archived) => Archived,
            (InactiveExhausted, Archived) => ArchivedExhausted,

            (Archived, Active) => Active,
            (ArchivedExhausted, Active) => ActiveExhausted,

            (Archived, Inactive) => Inactive,
            (ArchivedExhausted, Inactive) => InactiveExhausted,

            _ => {
                warn!("rule status transition from {current} to {new} is not supported");
                current
            }
        }
    } else {
        current
    }
}

#[cfg(test)]
mod test {

    mod update_status {
        use crate::model::RuleStatus::{
            Active, ActiveExhausted, Archived, ArchivedExhausted, Inactive, InactiveExhausted,
        };
        use crate::service::rule::update::update_status;

        #[test]
        fn test_nothing_to_change() {
            assert_eq!(Inactive, update_status(Inactive, None));
        }

        #[test]
        fn test_from_active_to_inactive() {
            assert_eq!(Inactive, update_status(Active, Some(Inactive)));
        }

        #[test]
        fn test_from_active_to_inactive_but_exhausted() {
            assert_eq!(
                InactiveExhausted,
                update_status(ActiveExhausted, Some(Inactive))
            );
        }

        #[test]
        fn test_from_active_to_archived() {
            assert_eq!(Archived, update_status(Active, Some(Archived)));
        }

        #[test]
        fn test_from_active_to_archived_but_exhausted() {
            assert_eq!(
                ArchivedExhausted,
                update_status(ActiveExhausted, Some(Archived))
            );
        }

        #[test]
        fn test_from_inactive_to_active() {
            assert_eq!(Active, update_status(Inactive, Some(Active)));
        }

        #[test]
        fn test_from_inactive_to_active_but_exhausted() {
            assert_eq!(
                ActiveExhausted,
                update_status(InactiveExhausted, Some(Active))
            );
        }

        #[test]
        fn test_from_inarchived_to_archived() {
            assert_eq!(Archived, update_status(Archived, Some(Archived)));
        }

        #[test]
        fn test_from_archived_to_archived_but_exhausted() {
            assert_eq!(
                ArchivedExhausted,
                update_status(ArchivedExhausted, Some(Archived))
            );
        }

        #[test]
        fn test_from_archived_to_inactive() {
            assert_eq!(Inactive, update_status(Archived, Some(Inactive)));
        }

        #[test]
        fn test_from_archived_to_inactive_but_exhausted() {
            assert_eq!(
                InactiveExhausted,
                update_status(ArchivedExhausted, Some(Inactive))
            );
        }

        #[test]
        fn test_from_archived_to_active() {
            assert_eq!(Active, update_status(Archived, Some(Active)));
        }

        #[test]
        fn test_from_archived_to_active_but_exhausted() {
            assert_eq!(
                ActiveExhausted,
                update_status(ArchivedExhausted, Some(Active))
            );
        }
    }
}
