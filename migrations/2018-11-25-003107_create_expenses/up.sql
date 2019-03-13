create table expenses (
  id integer primary key not null,
  payed_by integer not null,
  amount bigint not null,
  date bigint not null,

  foreign key(payed_by) REFERENCES users(id)
)