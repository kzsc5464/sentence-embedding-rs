use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType, SentenceEmbeddingsModel};
use anyhow::Result;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 모델을 한 번만 로드하는 간단한 싱글톤 패턴
static MODEL: Lazy<Mutex<Option<SentenceEmbeddingsModel>>> = Lazy::new(|| {
    Mutex::new(None)
});

// 기본 encode 함수 - 테스트용
pub fn encode(sentences: Vec<String>) -> Result<()> {
    let model = SentenceEmbeddingsBuilder::remote(
        SentenceEmbeddingsModelType::AllMiniLmL12V2
    ).create_model()?;
    
    let output = model.encode(&sentences)?;
    println!("{:?}", output);
    
    Ok(())
}

// 모델 로드 및 임베딩 생성 - 전체 과정을 동기적으로 처리
pub fn encode_sentences(sentences: &[String]) -> Result<Vec<Vec<f32>>> {
    // 모델 가져오기 (또는 필요하면 생성)
    let mut model_guard = match MODEL.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // 독(poison)된 락에서 값 복구
    };
    
    // 모델이 없으면 생성
    if model_guard.is_none() {
        println!("모델 최초 로딩 중...");
        let new_model = SentenceEmbeddingsBuilder::remote(
            SentenceEmbeddingsModelType::AllMiniLmL12V2
        ).create_model()?;
        
        *model_guard = Some(new_model);
        println!("모델 로딩 완료!");
    }
    
    // 문자열 슬라이스 준비
    let sentence_refs: Vec<&str> = sentences.iter()
        .map(|s| s.as_str())
        .collect();
    
    // 모델 참조 얻기
    let model = model_guard.as_ref().unwrap();
    
    // 임베딩 생성
    let embeddings = model.encode(&sentence_refs)?;
    
    Ok(embeddings)
}