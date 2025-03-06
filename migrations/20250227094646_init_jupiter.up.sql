-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create schema jupiter;

create function jupiter.update_updated_at_column()
returns trigger as $$
begin
    NEW.updated_at = timezone('utc', now());
    return NEW;
end;
$$ language plpgsql;


create table jupiter.trade
(
    slot                    int8 not null,
    address_id              int4 not null,
    token_pair_id           int4 not null,
    amount_base             numeric(36, 12) not null,
    amount_quote            numeric(36, 12) not null,
    price                   numeric(36, 12) not null,
    is_buy                  boolean not null,
    timestamp               timestamptz not null,
    signature               text not null,

    constraint fk_wallet
        foreign key (address_id)
        references solana.address(id),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id),

    constraint unique_signature
        unique (token_pair_id, signature)

 )  partition by hash (token_pair_id);
 
  create index trade_token_pair_idx on jupiter.trade(token_pair_id);
  
  create table jupiter.trade_1 partition of jupiter.trade
      for values with (modulus 8, remainder 0);
  
  create table jupiter.trade_2 partition of jupiter.trade
      for values with (modulus 8, remainder 1);
  
  create table jupiter.trade_3 partition of jupiter.trade
      for values with (modulus 8, remainder 2);
  
  create table jupiter.trade_4 partition of jupiter.trade
      for values with (modulus 8, remainder 3);
  
  create table jupiter.trade_5 partition of jupiter.trade
      for values with (modulus 8, remainder 4);
  
  create table jupiter.trade_6 partition of jupiter.trade
      for values with (modulus 8, remainder 5);
  
  create table jupiter.trade_7 partition of jupiter.trade
      for values with (modulus 8, remainder 6);
 
  create table jupiter.trade_8 partition of jupiter.trade
      for values with (modulus 8, remainder 7);