-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.
create schema nyanbot;
create schema solana;

create function nyanbot.update_updated_at_column()
returns trigger as $$
begin
    NEW.updated_at = timezone('utc', now());
    return NEW;
end;
$$ language plpgsql;

create function solana.update_updated_at_column()
returns trigger as $$
begin
    NEW.updated_at = timezone('utc', now());
    return NEW;
end;
$$ language plpgsql;
