create table if not exists todos (
  id           integer primary key not null,
  description  text                not null,
  done         boolean             not null default false
);
