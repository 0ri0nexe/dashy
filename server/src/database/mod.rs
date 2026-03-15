use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use argon2::{
    Argon2,
    password_hash::{
        PasswordHasher,
        PasswordVerifier,
        PasswordHash,
        SaltString,
        rand_core::OsRng,
    },
};
use time::{OffsetDateTime, Duration};
use tower_cookies::{Cookies, Cookie};
use log::{debug, info}; 

#[derive(FromRow)]
struct User {
    id: Uuid,
    email: String,
    password_hash: String,
}

#[derive(Deserialize)]
struct AuthInput {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct MeResponse {
    id: Uuid,
    email: String,
}

pub async fn init_database() -> PgPool {
    info!("initialising database");

    dotenvy::dotenv().ok();

    let url =
        std::env::var("DATABASE_URL")
        .expect("DATABASE_URL missing");

    let pool =
        PgPool::connect(&url)
        .await
        .expect("DB connect failed");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migration failed");

    pool
}

async fn register(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<AuthInput>,
) -> Result<impl IntoResponse, StatusCode> {
    log::info!("New register post request with json input :\n - email : {:#}\n - password : {:#}", input.email, input.password);

    let salt =
        SaltString::generate(&mut OsRng);

    let hash =
        Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id,email,password_hash)
         VALUES ($1,$2,$3)",
        id,
        input.email,
        hash
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(StatusCode::CREATED)
}

async fn login(
    Extension(pool): Extension<PgPool>,
    cookies: Cookies,
    Json(input): Json<AuthInput>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("New login request with json input :\n - email : {}\n - password : {}", input.email, input.password);
    
    let user =
        sqlx::query_as!(
            User,
            "SELECT id,email,password_hash
             FROM users
             WHERE email=$1",
            input.email
        )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let parsed =
        PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Argon2::default()
        .verify_password(
            input.password.as_bytes(),
            &parsed
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let session_id = Uuid::new_v4();

    let expires =
        OffsetDateTime::now_utc()
        + Duration::days(1);

    sqlx::query!(
        "INSERT INTO sessions (id,user_id,expires_at)
         VALUES ($1,$2,$3)",
        session_id,
        user.id,
        expires
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie =
        Cookie::build(("session_id", session_id.to_string()))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(1))
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .build();

    cookies.add(cookie);

    debug!("Request ok");
    Ok(StatusCode::OK)
}

async fn me(
    Extension(pool): Extension<PgPool>,
    cookies: Cookies,
) -> Result<Json<MeResponse>, StatusCode> {

    let cookie =
        cookies.get("session_id")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let session =
        Uuid::parse_str(cookie.value())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user =
        sqlx::query!(
            "
            SELECT users.id,users.email
            FROM sessions
            JOIN users
            ON users.id=sessions.user_id
            WHERE sessions.id=$1
            AND sessions.expires_at>NOW()
            ",
            session
        )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(MeResponse {
        id: user.id,
        email: user.email,
    }))
}

async fn logout(
    Extension(pool): Extension<PgPool>,
    cookies: Cookies,
) -> impl IntoResponse {

    if let Some(cookie) = cookies.get("session_id") {

        if let Ok(id) =
            Uuid::parse_str(cookie.value()) {

            let _ =
                sqlx::query!(
                    "DELETE FROM sessions WHERE id=$1",
                    id
                )
                .execute(&pool)
                .await;
        }
    }

    cookies.remove(
        Cookie::build(("session_id",""))
            .path("/")
            .build()
    );

    StatusCode::OK
}

pub fn setup_endpoints(router:Router, pool: PgPool) -> Router {
    info!("Database endpoints setup");
    router.route("/api/auth/register", post(register))
          .route("/api/auth/login", post(login))
          .route("/api/auth/me", get(me))
          .route("/api/auth/logout", post(logout))
          .layer(Extension(pool))
}