use authentication_password_strategy::hash_password;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Instantiate the database client
    let db = prisma_client::new_client()
        .await
        .expect("Failed to connect to database");

    // Seed base user
    let _user = db
        .user()
        .create(
            "admin".to_string(),
            "admin".to_string(),
            "admin@test.com".to_string(),
            hash_password("Qwerty123*"),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    Ok(())
}
