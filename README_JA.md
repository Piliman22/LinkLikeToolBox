# LinkLike ToolBox

Link! Like! ラブライブ！蓮ノ空女学院スクールアイドルクラブのゲームデータ操作・抽出を行う総合ツールキットです。

## 機能

### アセット管理
- **アセットバンドル暗号化/復号化**: LinkLike形式のアセットバンドルの暗号化・復号化
- **バッチ処理**: 複数ファイル・フォルダの一括処理
- **フォーマットサポート**: 各種LinkLikeアセット形式に対応

### 譜面データ処理
- **譜面展開**: 圧縮された譜面ファイル（.bytes）からJSONデータを抽出
- **譜面圧縮**: JSON譜面データを.bytes形式に圧縮
- **譜面解析**: リズムゲーム譜面データの詳細統計分析

### マスターデータ抽出
- **自動更新**: ゲームマスターデータの自動ダウンロード・更新
- **カタログ処理**: ゲームアセットカタログの処理・フィルタリング
- **データベース変換**: 暗号化マスターデータを読み取り可能な形式に変換

### マニフェスト処理
- **マニフェストダウンロード**: ゲームマニフェストのダウンロード・処理
- **暗号化処理**: 暗号化マニフェストデータの処理
- **アセット追跡**: ゲームアセットの追跡・管理

## インストール

### 前提条件
- Rust 1.70以上 (ソースからのビルド時)
- Docker (コンテナ使用時)

### ソースからビルド
```bash
git clone <repository-url>
cd LinkLikeToolBox
cargo build --release
```

### Docker使用
```bash
docker-compose up linklike-toolbox
```

## 使用方法

### CLIコマンド

#### アセットバンドル操作
```bash
# アセットバンドル復号化
./linklike decrypt ab <file_path>

# アセットバンドル暗号化
./linklike crypt ab <file_path>
```

#### 譜面データ操作
```bash
# 譜面ファイル展開 (.bytes → .json)
./linklike decrypt chart <file_path>

# 譜面ファイル圧縮 (.json → .bytes)
./linklike crypt chart <file_path> [compression_level]
```

#### マスターデータ操作
```bash
# マスターデータ自動更新
./linklike download auto [options]

# 特定マニフェストダウンロード
./linklike download manifest <real_name> <save_directory>

# カタログからすべてのアセットをダウンロード
./linklike download assets <catalog_path> <download_directory>
```

#### 自動更新オプション
```bash
# バージョンが同じでも強制更新
./linklike download auto --force

# データベースファイル(.tsv)のみダウンロード・変換
./linklike download auto --db-only

# 譜面ファイル(.bytes → .json)のみダウンロード・変換
./linklike download auto --chart-only

# 生のダウンロードファイルを保持
./linklike download auto --keep-raw

# オプション組み合わせ例
./linklike download auto --chart-only --keep-raw
./linklike download auto --db-only --force
```

### Python譜面解析
```bash
python chart_analyzer.py
```

## プロジェクト構造

```
LinkLikeToolBox/
├── crates/
│   ├── LinkLike_Core/     # コア機能
│   │   ├── src/
│   │   │   ├── crypt/     # 暗号化/復号化モジュール
│   │   │   ├── fetch/     # データフェッチ・処理
│   │   │   ├── master/    # マスターデータ処理
│   │   │   └── manifest.rs # マニフェスト処理
│   │   └── Cargo.toml
│   └── LinkLike_CLI/      # コマンドラインインターフェース
│       ├── src/
│       └── Cargo.toml
├── example/               # サンプル譜面ファイル
├── chart_analyzer.py      # 譜面解析ツール
├── docker-compose.yml
├── Dockerfile
└── Makefile
```

## コアコンポーネント

### LinkLike_Core
- [`AssetBundle`](crates/LinkLike_Core/src/crypt/asset_bundle.rs): LinkLikeアセットバンドル形式の処理
- [`Chart`](crates/LinkLike_Core/src/crypt/chart.rs): リズムゲーム譜面データの処理
- [`AssetDecoder`](crates/LinkLike_Core/src/crypt/asset_decoder.rs): ゲームアセットの復号化
- [`AutoUpdater`](crates/LinkLike_Core/src/fetch/auto_update.rs): 自動データ更新
- [`Catalog`](crates/LinkLike_Core/src/manifest.rs): ゲームアセットカタログ管理

### LinkLike_CLI
- 全操作のコマンドラインインターフェース
- プログレス表示とカラー出力
- バッチ処理機能

## 設定

### 更新オプション
- `--force`: 全アセットを強制更新
- `--db-only`: データベースファイルのみ更新
- `--chart-only`: 譜面ファイルのみ更新
- `--keep-raw`: 生のダウンロードファイルを保持

### 圧縮レベル
譜面圧縮は0-9のレベルをサポート：
- 0: 無圧縮（最高速）
- 6: デフォルト圧縮（バランス）
- 9: 最大圧縮（最小サイズ）

## 開発

### ビルド
```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# 特定プラットフォーム向けビルド
make build-linux
make build-windows
```

### テスト
```bash
cargo test
```

### Docker開発
```bash
# 開発コンテナ
docker-compose -f docker-compose.yml up dev

# プロダクションビルド
make docker-build
```

## 譜面解析

付属のPythonスクリプトはリズムゲーム譜面の詳細分析を提供：
- ノートタイミングとタイプ分析
- BPMとビート情報
- 統計サマリー
- CSV出力機能

### 譜面解析機能
- **ノート分類**: 異なるノートタイプの分類
- **タイミング分析**: ノートタイミングパターンの分析
- **フラグ解読**: ノートフラグ情報の解読
- **出力オプション**: 解析結果のCSV出力

## ファイル形式

### サポート形式
- `.assetbundle`: LinkLikeアセットバンドル
- `.bytes`: 圧縮譜面データ
- `.json`: 展開譜面データ
- `.tsv`: マスターデータベースファイル

### 譜面データ構造
```json
{
  "Bpms": [...],
  "Offset": 0.0,
  "Beats": [...],
  "Notes": [
    {
      "just": 1000.0,
      "Flags": 123456,
      "Uid": 1,
      "holds": [...]
    }
  ]
}
```

## コントリビュート

1. リポジトリをフォーク
2. フィーチャーブランチを作成
3. 変更を加える
4. 必要に応じてテストを追加
5. プルリクエストを送信

## ライセンス

このプロジェクトは教育・研究目的で作成されています。元ゲームの利用規約を尊重してください。

## 免責事項

このツールは教育目的のみで提供されています。ユーザーはゲームの利用規約および適用法を遵守する責任があります。