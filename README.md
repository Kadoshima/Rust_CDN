# mini-cdn-rs（学習用ミニCDN / キャッシュ付きリバースプロキシ）

学習目的で作る **「キャッシュ付きリバースプロキシ」**（= ミニCDN）のRust実装です。
HTTPサーバ部分は **hyper / axum など既存クレート**を使い、主に以下を理解・検証することに集中します。

* HTTPキャッシュの基本（TTL / fresh・stale / 再検証の考え方）
* キャッシュキー設計（URL・クエリ・一部ヘッダ）
* `req/s（Hz）` と `ヒット率（hit/miss）` の計測・比較
* キャッシュ導入が **オリジン負荷**と **レイテンシ**に与える影響

> ⚠️ 本プロジェクトは学習用です。プロダクション用途の安全性・網羅性（攻撃耐性/完全なRFC準拠/TLS等）は対象外です。

---

## できること（スコープ）

### 目標（最低限）

* オリジンに対する **リバースプロキシ**（GET中心）
* インメモリキャッシュ（LRU or TTL付き）
* `Cache-Control: max-age` を基準にしたTTLキャッシュ
* メトリクス出力（hit/miss、req/s、オリジンへの転送量など）
* ベンチで **キャッシュOFF vs ON** を比較できる

### 可能ならやる（拡張）

* 再検証（ETag / If-None-Match、Last-Modified / If-Modified-Since）
* `stale-while-revalidate` 的な挙動（バックグラウンド更新）
* `Vary` を考慮したキャッシュキー（最低限でOK）

### やらない（非目標）

* TLS終端（HTTPS）
* 完全なHTTP仕様網羅（chunked, range, compression, pipeline等の厳密対応）
* 認証・認可やWAF的防御
* 分散キャッシュ（単一プロセス・単一ノード）

---

## アーキテクチャ概要

```
Client ──HTTP──> mini-cdn-rs ──HTTP──> Origin
                  │
                  ├─ Cache lookup (key, ttl)
                  ├─ HIT: cached response
                  └─ MISS: fetch from origin → store → response
```

* **HTTP基盤**: hyper / axum（どちらか採用）
* **オリジン取得**: reqwest（or hyper client）
* **キャッシュ**: `moka`（TTL付きで便利）or `lru` + 自前TTL
* **計測**: カウンタ（hit/miss/bytes）＋簡易レート計算

---

## キャッシュ仕様（最小版）

### キャッシュ対象

* デフォルトは **GETのみ**
* `Cache-Control: no-store` はキャッシュしない（予定）
* TTLは基本 `Cache-Control: max-age` を優先
* `max-age` が無い場合は `DEFAULT_TTL_SECS` を使う（学習用割り切り）

### キャッシュキー（最小版）

* `scheme + host(origin固定なら不要) + path + query`
* 将来：`Vary` 対応で一部ヘッダをキーに含める

---

## メトリクス（例）

最低限以下をログ or `/metrics` で出します。

* total_requests
* cache_hits / cache_misses（hit率）
* origin_requests（=miss数の目安）
* bytes_from_origin / bytes_to_client
* req/s（簡易：直近N秒の移動平均でもOK）
* レイテンシ（平均/ p95 どちらか）

---

## ベンチマーク方法（例）

### 目的

* キャッシュOFF vs ON で

  * req/s（Hz）
  * ヒット率
  * オリジンへのリクエスト数
  * レイテンシ
    がどう変わるかを見る

### 手順例

1. キャッシュOFFで `wrk` / `hey` / `oha` を実行
2. キャッシュONで同条件を実行
3. 同じURLを叩く（ヒット率が上がる条件）と、ランダムURLを叩く（ヒット率が上がらない条件）を両方試す

---

## ディレクトリ構成（案）

```
src/
  main.rs
  config.rs
  proxy.rs          # リクエスト転送
  cache.rs          # キャッシュ層（get/set/ttl/key）
  metrics.rs        # hit/miss/bytes/reqs など
  headers.rs        # Cache-Control 解析など（必要なら）
README.md
```

---

## ライセンス

学習用なのでお好みで（MIT/Apache-2.0 など）

---