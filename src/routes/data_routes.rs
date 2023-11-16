use axum::{http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromField, TryFromMultipart, TypedMultipart};
use serde::Serialize;
use std::{fs, path::Path};
use tempfile::NamedTempFile;

#[derive(TryFromField, Serialize, Clone)]
pub enum Level {
    Beginner,
    Pro,
}

#[derive(TryFromMultipart, Serialize)]
pub struct FormData {
    first_name: String,
    last_name: String,
    github: Option<String>,
    level: Option<Level>,
}

pub async fn form_data(data: TypedMultipart<FormData>) -> Json<FormData> {
    let first_name = &data.first_name;
    let last_name = &data.last_name;
    let github = &data.github;
    let level = data.level.clone();

    Json(FormData {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        github: if github.to_owned() == None {
            Some("https://github.com/FMFigueroa".to_owned())
        } else {
            github.to_owned()
        },
        level,
    })
}

#[derive(TryFromMultipart)]
pub struct UploadAssetRequest {
    // The `unlimited arguments` means that this field will be limited to the
    // total size of the request body. If you want to limit the size of this
    // field to a specific value you can also specify a limit in bytes, like
    // '5MiB' or '1GiB'.
    #[form_data(limit = "unlimited")]
    image: FieldData<NamedTempFile>,

    // This field will be limited to the default size of 1MiB.
    author: String,
}

pub async fn upload_asset(
    TypedMultipart(UploadAssetRequest { image, author }): TypedMultipart<UploadAssetRequest>,
) -> StatusCode {
    let file_name = image.metadata.file_name.unwrap_or(String::from("data.bin"));
    let user_path = Path::new("./tmp").join(author);

    //Create directory
    fs::create_dir_all(&user_path).unwrap();
    let file_path = user_path.join(&file_name);

    println!("{:?}", file_path);

    // upload asset local
    match image.contents.persist(file_path) {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
