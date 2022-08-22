-- Add migration script here
create table if not exists users (
  id bigserial primary key,
  name text,
  email text unique
);