# TODO（簡易）

## Phase 0: 土台

* [-] `cargo new mini-cdn-rs` 作成
* [-] 設定（env or clap）: `ORIGIN_BASE`, `LISTEN_ADDR`, `CACHE_ENABLED`, `DEFAULT_TTL_SECS`
* [-] ログ基盤（tracing）を入れる

## Phase 1: リバースプロキシ（キャッシュ無し）

* [ ] `GET /{path..}` をオリジンへ転送してレスポンスを返す
* [ ] エラー設計（origin unreachable / timeout / 5xx をどう返すか）
* [ ] 最低限のヘッダ取り回し（Hostなどは整理）

## Phase 2: キャッシュ導入（最小版）

* [ ] キャッシュキー: `path + query`（まずはこれだけ）
* [ ] TTL決定: `Cache-Control: max-age` > `DEFAULT_TTL_SECS`
* [ ] HITならキャッシュから返す / MISSならオリジンから取って保存

## Phase 3: メトリクス

* [ ] hit/miss/total/origin_requests をカウンタで保持
* [ ] bytes_from_origin / bytes_to_client を計測
* [ ] `/metrics` かログに定期出力
* [ ] hit率を表示（hits / total）

## Phase 4: ベンチ

* [ ] ベンチ手順をREADMEに記載（使用ツール、コマンド例）
* [ ] キャッシュOFF vs ON の結果を表にする（req/s、hit率、origin req）
* [ ] 「ヒットするケース」「ヒットしないケース」を両方測る

## Phase 5: 余裕があれば（拡張）

* [ ] ETag対応（If-None-Matchで再検証、304ならTTL更新）
* [ ] stale-while-revalidate っぽい更新（古いの返して裏で更新）
* [ ] `Vary` を最小限で（例：`Accept-Encoding` だけ対応など）