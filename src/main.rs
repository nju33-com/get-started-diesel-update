use diesel::pg::PgConnection;
use diesel::prelude::*;
use get_started_diesel_update::schema;

const DATABASE_URL: &'static str = "postgres://postgres:@localhost:5432";

fn main() {
  let connection =
    PgConnection::establish(DATABASE_URL).expect(&format!("Error connecting to {}", DATABASE_URL));

  diesel::update(schema::users::dsl::users.find(1))
    .set(schema::users::dsl::name.eq("baz"))
    .execute(&connection)
    .expect("Error updating the user");

  let target = schema::users::dsl::users.filter(schema::users::dsl::name.eq("bar"));
  diesel::update(target)
    .set(schema::users::dsl::name.eq("baz"))
    .execute(&connection)
    .expect("Error updating an users");
}
