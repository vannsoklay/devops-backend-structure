use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::stream::StreamExt;
use std::{collections::HashMap, fs, path::Path};
use tokio::{
    self,
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

// Route configuration
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(upload_chunk);
}

#[post("/upload_chunk")]
async fn upload_chunk(
    mut payload: Multipart,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> impl Responder {
    let chunk_index: usize = params.get("chunkIndex").unwrap().parse().unwrap();
    let total_chunks: usize = params.get("totalChunks").unwrap().parse().unwrap();
    let upload_id = params.get("uploadId").unwrap();
    let mut original_extension = String::new(); // Default extension if not detected
    let mut filename = String::new();

    let chunk_path = format!("temp/{}_chunk_{}", upload_id, chunk_index);
    let file_exists = Path::new(&chunk_path).exists();

    if file_exists {
        // Return a success response immediately if the chunk already exists
        return HttpResponse::Ok().body("Chunk already uploaded.");
    }

    let mut file = match File::create(&chunk_path).await {
        Ok(f) => f,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Unable to create file: {}", e))
        }
    };

    // Extract the file extension from the first field of the multipart data
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                filename = field
                    .content_disposition()
                    .unwrap()
                    .get_filename()
                    .unwrap()
                    .to_string();
                // Separate scope to handle writing data to avoid repeated mutable borrow of `file`
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            // Write data to the file
                            if let Err(e) = file.write_all(&data).await {
                                return HttpResponse::InternalServerError()
                                    .body(format!("Unable to write data: {}", e));
                            }
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError()
                                .body(format!("Error reading chunk: {}", e))
                        }
                    }
                }
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error processing file field: {}", e))
            }
        }
    }

    if let Some(ext) = Path::new(&filename).extension() {
        original_extension = ext.to_str().unwrap_or("bin").to_string();
    }
    // Check if this is the last chunk and reassemble the file
    if chunk_index == total_chunks - 1 {
        let final_file_path = format!("uploads/{}.{}", upload_id, original_extension);
        let mut final_file = match File::create(&final_file_path).await {
            Ok(f) => f,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Unable to create final file: {}", e))
            }
        };

        for i in 0..total_chunks {
            let chunk_path = format!("temp/{}_chunk_{}", upload_id, i);
            if Path::new(&chunk_path).exists() {
                let mut chunk_file = match File::open(&chunk_path).await {
                    Ok(f) => f,
                    Err(e) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("Unable to open chunk file {}: {}", chunk_path, e))
                    }
                };
                let mut buffer = Vec::new();
                if let Err(e) = chunk_file.read_to_end(&mut buffer).await {
                    return HttpResponse::InternalServerError()
                        .body(format!("Unable to read chunk file {}: {}", chunk_path, e));
                }
                if let Err(e) = final_file.write_all(&buffer).await {
                    return HttpResponse::InternalServerError()
                        .body(format!("Unable to write to final file: {}", e));
                }
            }
        }

        cleanup_temp_files(upload_id, total_chunks);

        HttpResponse::Ok().body("All chunks uploaded and reassembled successfully")
    } else {
        HttpResponse::Ok().body("Chunk uploaded successfully")
    }
}

fn cleanup_temp_files(upload_id: &str, total_chunks: usize) {
    for i in 0..total_chunks {
        let chunk_path = format!("temp/{}_chunk_{}", upload_id, i);
        if Path::new(&chunk_path).exists() {
            fs::remove_file(chunk_path).expect("Unable to delete temporary chunk file");
        }
    }
}
