use axum::{
    Extension, Form, Json, Router,
    body::StreamBody,
    extract::Multipart,
    http::{HeaderMap, header},
    response::{Html, IntoResponse},
    routing::{get, post},
};

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Row, Sqlite};
use std::{net::SocketAddr, path::Path};
use tokio::task::spawn_blocking;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and build environment variables
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Catch up on missing thumbnails
    fill_missing_thumbnails(&pool).await?;

    // Build Axum with an "extension" to hold the database connection pool
    // let app = Router::new().route("/", get(test)).layer(Extension(pool));

    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .route("/image/:id", get(get_image))
        .route("/images", get(list_images))
        .route("/search", post(search_image))
        .route("/thumb/:id", get(get_thumbnail))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
//     let result = sqlx::query("SELECT COUNT(id) FROM images")
//         .fetch_one(&pool)
//         .await
//         .unwrap();

//     let count = result.get::<i64, _>(0);
//     format!("{count} images in the database")
// }

async fn index_page() -> Html<String> {
    let path = Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();

    Html(content)
}

async fn insert_image_into_database(pool: &sqlx::SqlitePool, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("INSERT INTO images (tags) VALUES (?) RETURNING id")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    // Check that the images folder exists and is a directory
    // If it doesn't, create it.
    let base_path = Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // Use "join" to create a path to the image file. Join is platform aware,
    // it will handle the differences between Windows and Linux.
    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        // The file exists. That shouldn't happen.
        anyhow::bail!("File already exists");
    }

    // Write the image to the file
    tokio::fs::write(image_path, bytes).await?;
    Ok(())
}

async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart,
) -> Html<String> {
    let mut tags = None; // "None" means "no tags yet"
    let mut image = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()), // Using Some means we can check we receive
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field: {name}"),
        }
    }

    // Destructuring both Options at once
    if let (Some(tags), Some(image)) = (tags, image) {
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
        save_image(new_image_id, &image).await.unwrap();
        spawn_blocking(move || {
            make_thumbnail(new_image_id).unwrap();
        });
    } else {
        panic!("Missing field")
    }

    let path = Path::new("src/redirect.html");
    let contents = tokio::fs::read_to_string(path).await.unwrap();

    Html(contents)
}

async fn get_image(axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}.jpg");
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap(),
    );

    let file = tokio::fs::File::open(&filename).await.unwrap();
    axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/jpeg"),
        )
        .header(
            header::CONTENT_DISPOSITION,
            header::HeaderValue::from_str(&attachment).unwrap(),
        )
        .body(StreamBody::new(ReaderStream::new(file)))
        .unwrap()
}

fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let image_path = format!("images/{id}.jpg");
    let thumbnail_path = format!("images/{id}_thumb.jpg");
    let image_bytes: Vec<u8> = std::fs::read(image_path)?;

    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };

    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;
    Ok(())
}

async fn fill_missing_thumbnails(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut rows = sqlx::query("SELECT id FROM images").fetch(pool);

    while let Some(row) = rows.try_next().await? {
        let id = row.get::<i64, _>(0);
        let thumbnail_path = format!("images/{id}_thumb.jpg");
        if !Path::new(&thumbnail_path).exists() {
            spawn_blocking(move || make_thumbnail(id)).await??;
        }
    }

    Ok(())
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct ImageRecord {
    id: i64,
    tags: String,
}

async fn list_images(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<ImageRecord>> {
    sqlx::query_as::<_, ImageRecord>("SELECT id, tags FROM images ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into()
}

async fn get_thumbnail(axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}_thumb.jpg");
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap(),
    );

    let file = tokio::fs::File::open(&filename).await.unwrap();
    axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/jpeg"),
        )
        .header(
            header::CONTENT_DISPOSITION,
            header::HeaderValue::from_str(&attachment).unwrap(),
        )
        .body(StreamBody::new(ReaderStream::new(file)))
        .unwrap()
}

#[derive(Deserialize)]
struct Search {
    tags: String,
}

async fn search_image(
    Extension(pool): Extension<sqlx::SqlitePool>,
    Form(form): Form<Search>,
) -> Html<String> {
    let tag = format!("%{}%", form.tags);

    let rows = sqlx::query_as::<_, ImageRecord>(
        "SELECT id, tags FROM images WHERE tags LIKE ? ORDER BY id",
    )
    .bind(tag)
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut results = String::new();
    for row in rows {
        results.push_str(&format!(
            "<a href=\"/image/{}\"><img src='/thumb/{}' /></a><br/>",
            row.id, row.id
        ));
    }

    let path = Path::new("src/search.html");
    let mut content = tokio::fs::read_to_string(path).await.unwrap();
    content = content.replace("{results}", &results);

    Html(content)
}
