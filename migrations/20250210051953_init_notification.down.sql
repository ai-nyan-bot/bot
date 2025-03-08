-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.
drop trigger trigger_store_notification_sent on nyanbot.notification;
drop function nyanbot.store_notification_sent;
drop table nyanbot.notification cascade;
drop table nyanbot.notification_sent cascade;