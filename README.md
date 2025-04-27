# Rust-Bert Sample Code with warp

## Spec
Rust : "1.82"
rust-bert= "0.23.0"
warp = "0.3"

## Description
이 프로젝트는 Rust-Bert 라이브러리를 사용하여 텍스트 문장의 임베딩 벡터를 생성하는 REST API를 제공합니다. 다국어 문장 임베딩을 지원하며, 텍스트 유사도 검색, 클러스터링, 분류 등 다양한 NLP 작업에 활용할 수 있습니다.



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
