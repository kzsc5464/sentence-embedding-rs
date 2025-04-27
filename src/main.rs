mod embedding;

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;

#[derive(Deserialize)]
struct EmbeddingRequest {
    sentences: Vec<String>,
}

#[derive(Serialize)]
struct EmbeddingResponse {
    success: bool,
    message: String,
    embeddings: Option<Vec<Vec<f32>>>,
}

// 임베딩 처리 핸들러
async fn handle_embedding(req: EmbeddingRequest) -> Result<impl Reply, Rejection> {
    // 문장 데이터의 길이를 미리 저장해 둡니다
    let sentence_count = req.sentences.len();
    
    // 블로킹 작업을 별도 스레드로 이동
    let result = tokio::task::spawn_blocking(move || {
        embedding::encode_sentences(&req.sentences)
    }).await.unwrap(); // spawn_blocking 결과 unwrap
    
    match result {
        Ok(embeddings) => {
            let response = EmbeddingResponse {
                success: true,
                message: format!("Successfully embedded {} sentences", sentence_count),
                embeddings: Some(embeddings),
            };
            Ok(warp::reply::json(&response))
        },
        Err(e) => {
            let response = EmbeddingResponse {
                success: false,
                message: format!("Error: {}", e),
                embeddings: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// 헬스 체크 핸들러
async fn handle_health() -> Result<impl Reply, Rejection> {
    Ok("Embedding API is running!")
}

// CORS 설정 추가
fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
}

#[tokio::main]
async fn main() {
    println!("Starting Embedding API server at http://127.0.0.1:8080");
    
    // 라우트 정의
    let health_route = warp::path("health")
        .and(warp::get())
        .and_then(handle_health);
    
    let embed_route = warp::path("embed")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_embedding);
    
    // 모든 라우트 결합
    let routes = health_route
        .or(embed_route)
        .with(cors());
    
    // 서버 시작
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}