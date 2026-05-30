// Outil hors-ligne de (ré)initialisation d'un compte admin.
//
// Usage :
//   cargo run --bin admin_reset -- <db_path> <username> <password>
//
// (Re)crée la ligne `admin_users` correspondante avec un hash argon2id
// produit par le même code que celui qui vérifie le login. Si le pseudo
// existe déjà, son mot de passe est remplacé.
//
// Rien à voir avec la Forge (/la_forge), qui s'authentifie par email.

use sqlx::sqlite::SqlitePoolOptions;
use xenna_paie_lib::admin::auth::hash_password;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let (db_path, username, password) = match (args.next(), args.next(), args.next()) {
        (Some(d), Some(u), Some(p)) => (d, u, p),
        _ => {
            eprintln!("Usage : admin_reset <db_path> <username> <password>");
            std::process::exit(2);
        }
    };

    if password.len() < 8 {
        eprintln!("Mot de passe : 8 caractères minimum.");
        std::process::exit(2);
    }

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite://{db_path}"))
        .await?;

    let hash = hash_password(&password);

    sqlx::query(
        "INSERT INTO admin_users (username, password_hash)
         VALUES (?, ?)
         ON CONFLICT(username) DO UPDATE SET password_hash = excluded.password_hash",
    )
    .bind(&username)
    .bind(&hash)
    .execute(&pool)
    .await?;

    println!("Compte admin « {username} » prêt dans {db_path}.");
    Ok(())
}
