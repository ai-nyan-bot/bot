-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.invocation
(
    id              serial not null,
    user_id         int not null,
    rule_id     int not null,
    token_pair_id   int not null,
    next            json,
    created_at      timestamptz default (timezone('utc', now())),

    primary key (rule_id, token_pair_id),

    constraint fk_user
        foreign key (user_id)
        references nyanbot.user(id),

    constraint fk_rule
        foreign key (rule_id)
        references solana.rule(id),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);