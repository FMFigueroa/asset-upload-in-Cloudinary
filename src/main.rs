use std::{io::Write, net::SocketAddr};

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Response,
    routing::{on, MethodFilter},
    Router,
};
use cloudinary::{upload::UploadOptions, Cloudinary, Source};
use std::fs::File;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", on(MethodFilter::GET, hello_world))
        .route("/upload", on(MethodFilter::POST, upload)) // Cambia MethodFilter a POST
        .route("/download", on(MethodFilter::GET, download));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server listening on http://{addr}\n");

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("server failed");

    Ok(())
}

pub async fn hello_world() -> String {
    "Hello world".to_owned()
}

pub async fn upload(mut multipart: Multipart) -> Result<Response, StatusCode> {
    // Configura el cliente Cloudinary
    let options = UploadOptions::new().set_public_id("file.jpg".to_string());
    let cloudinary = Cloudinary::new(
        "api_key".to_string(),
        "cloud_name".to_string(),
        "api_secret".to_string(),
    );

    // Crea una carpeta temporal
    let temp_dir = TempDir::new().unwrap();

    while let Some(field) = multipart.next_field().await.unwrap() {
        // Añade `mut` a `field`
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        // Crea un archivo temporal en la carpeta temporal
        let file_path = temp_dir.path().join(&filename);
        let mut file = File::create(&file_path).unwrap();

        // Copia el contenido del archivo temporal al campo multipart
        let chunk = std::fs::read(&file_path).unwrap();
        while !chunk.is_empty() {
            let chunk = std::fs::read(&file_path).unwrap();
            file.write(&chunk).unwrap();
            //chunk = std::fs::read(&file_path).unwrap();
        }

        // Carga el archivo en Cloudinary
        let result = cloudinary.upload_image(Source::Path("./image.jpg".into()), &options);

        // Elimina el archivo temporal
        std::fs::remove_file(file_path).unwrap();

        match result.await.map_err(|err| {
            eprintln!("Error al cargar el archivo en Cloudinary: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }) {
            Ok(response) => {
                // La carga fue exitosa, puedes manejar la respuesta de Cloudinary aquí
                println!("Archivo cargado en Cloudinary: {:?}", response);
                return Ok(Response::builder()
                    .status(StatusCode::CREATED)
                    .body(axum::body::boxed("OK".to_string()))
                    .unwrap());
            }
            Err(status) => return Err(status),
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn download() -> String {
    "Hello download".to_owned()
}
