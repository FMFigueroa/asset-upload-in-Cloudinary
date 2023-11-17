use std::path::{Path, PathBuf};

use cloudinary::{upload::UploadOptions, Cloudinary, Source};
use dotenvy::dotenv;

pub async fn upload_asset_to_cloudinary(
    file_path: PathBuf,
    author: String,
    file_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
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

    // Parameters
    let public_id = format!("{}/{}", &author, public_name);
    let image_path = &file_path;

    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
        .set_folder(String::from("storage"))
        .set_overwrite(true);

    cloudinary
        .upload_image(Source::Path(image_path.try_into().unwrap()), &options)
        .await
        .expect("Upload failed");

    Ok(())
}
