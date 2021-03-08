-- Your SQL goes here
create unique index ip_index on locations (from_ip_numeric, to_ip_numeric);
