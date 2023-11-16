use axum::{http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromField, TryFromMultipart, TypedMultipart};
use cloudinary::{upload::UploadOptions, Cloudinary, Source};
use dotenvy::dotenv;
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
    dotenv().ok();

    let file_name = image.metadata.file_name.unwrap_or(String::from("data.bin"));
    let user_path = Path::new("./tmp").join(&author);

    //Create directory
    fs::create_dir_all(&user_path).unwrap();
    let file_path = user_path.join(&file_name);

    // Upload asset local
    match image.contents.persist(&file_path) {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Cloudinary Credencials Setup
    let api_key = dotenvy::var("CLOUDINARY_API_KEY").expect("enviroment variables not set");
    let cloud_name = dotenvy::var("CLOUDINARY_CLOUD_NAME").expect("enviroment variables not set");
    let api_secret = dotenvy::var("CLOUDINARY_API_SECRET").expect("enviroment variables not set");
    let cloudinary = Cloudinary::new(api_key, cloud_name, api_secret);

    // Remove the extension from the asset
    let file_name_alone = Path::new(&file_name);
    let public_name = file_name_alone
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default();

    //
    let public_id = format!("{}/{}", &author, public_name);
    let image_path = &file_path;

    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
        .set_folder(String::from("storage"))
        .set_overwrite(true);

    let res = cloudinary
        .upload_image(Source::Path(image_path.try_into().unwrap()), &options)
        .await;

    fs::remove_dir_all(&user_path).unwrap_or_else(|err| {
        eprintln!("Error al eliminar contenido de la carpeta local: {:?}", err);
    });

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            eprintln!("Error uploading to Cloudinary: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}
