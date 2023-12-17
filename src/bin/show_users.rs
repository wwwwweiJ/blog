use self::models::*;
use diesel::prelude::*;
use blog::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(email.eq("leo2255leo2255"))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("display {} users", results.len());
    for user in results {
        println!("{}", user.email);
    }
}
