# atcoder_rs
AtCoderのコンテスト参加のためのRustのテンプレートや、ライブラリをまとめたリポジトリです。
コンテストの実装結果は、`_result`以下にあります。

## 設定
- atcoderのコンテストをやりやすくするため、[cargo-compete](https://github.com/qryxip/cargo-compete)を使っています。
- compete.toml: コンテストの情報を記述するファイルです。
  - compete.toml内の`[template]`セクションには、`cargo compete new`で生成されるファイルのテンプレートが記述されています。
- Rustのバージョンを指定するため、`rust-toolchain`ファイルを使用しています。
  - Rustバージョンは1.70.0を指定しています。
 
