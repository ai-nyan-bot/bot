-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.invocation
(
    user_id         int not null,
    strategy_id     int not null,
    token_pair_id   int not null,
    sequence        json not null,
    created_at      timestamptz default (timezone('utc', now())),

    primary key (strategy_id, token_pair_id),

    constraint fk_user
        foreign key (user_id)
        references nyanbot.user(id),

    constraint fk_strategy
        foreign key (strategy_id)
        references nyanbot.strategy(id),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);