# Rust-Bert Sample Code with warp

## Description
    This is Rust-Bert SentenceEmbedding REST API




## Run
```
cargo run
```

## Test
```
curl -X POST http://127.0.0.1:8080/embed \
  -H "Content-Type: application/json" \
  -d '{"sentences": ["이것은 테스트 문장입니다.", "다른 언어도 지원합니다."]}'
```