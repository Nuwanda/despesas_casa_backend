CREATE TABLE expense_user (
  id integer primary key not null,
  expense_id integer key not null,
  user_id integer key not null,

  foreign key(expense_id) REFERENCES expenses(id),
  foreign key(user_id) REFERENCES users(id)
)