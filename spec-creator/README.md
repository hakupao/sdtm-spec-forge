# SDTM Dataset Specification Creator

SDTM データセット仕様書を自動生成する CLI ツール。
CSV 形式の SDTM データセットと CDISC 公式メタデータを読み取り、Excel 形式の仕様書を出力します。

## 必要な入力ファイル

```
プロジェクト/
├── data/                                # SDTM データセット
│   ├── DM.csv                           #   各ドメインの CSV（UTF-8 BOM）
│   ├── CM.csv
│   ├── DS.csv
│   ├── ...
│   └── <STUDY_ID>_OperationConf.xlsx    #   EDC マッピング定義ファイル
├── master/                              # CDISC 公式メタデータ
│   ├── SDTMIG_v3.4.xlsx                 #   SDTMIG 変数・データセット定義
│   └── SDTM Terminology.xlsx            #   CDISC Controlled Terminology
└── output/                              # 出力先（自動作成）
```

### 各ファイルの役割

| ファイル | 必須 | 用途 |
|---------|------|------|
| `data/*.csv` | ○ | 分析対象の SDTM データセット |
| `SDTMIG_v3.4.xlsx` | ○ | 変数の Label / Core / Role / CDISC Notes を取得 |
| `SDTM Terminology.xlsx` | ○ | コードリスト（CDISC CT）との照合 |
| `<STUDY_ID>_OperationConf.xlsx` | ○ | EDC→SDTM マッピング定義、Sort Key、DEFINITION 取得 |

## ビルド

```bash
# Rust ツールチェーンが必要（rustup でインストール）
cd spec-creator
cargo build --release
```

ビルド後の実行ファイル:
```
target/release/sdtm-spec-creator.exe
```

## 使い方

### 基本コマンド（PowerShell）

```powershell
sdtm-spec-creator.exe `
  --data-dir "../data" `
  --sdtmig "../master/SDTMIG_v3.4.xlsx" `
  --terminology "../master/SDTM Terminology.xlsx" `
  --mapping "../data/MY_STUDY_OperationConf.xlsx" `
  --output "../output" `
  --study-id MY_STUDY
```

### TOML 設定ファイルを使う場合

```powershell
sdtm-spec-creator.exe --config ../config.toml
```

### パラメータ一覧

| パラメータ | 必須 | デフォルト | 説明 |
|-----------|------|-----------|------|
| `--config` | - | - | TOML 設定ファイルのパス（指定時は他の引数は不要） |
| `--data-dir` | ○ | - | SDTM CSV ファイルが格納されたディレクトリ |
| `--sdtmig` | ○ | - | SDTMIG v3.4 Excel ファイルのパス |
| `--terminology` | ○ | - | SDTM Terminology Excel ファイルのパス |
| `--mapping` | ○ | - | OperationConf.xlsx のパス |
| `--output` | - | `./output` | 出力先ディレクトリ（存在しなければ自動作成） |
| `--study-id` | ○ | - | Study ID（出力ファイル名とCoverシートに使用） |

### 出力

```
output/
└── {STUDY_ID}_SDTM_Dataset_Specification_{YYYYMMDD}.xlsx
```

## 出力 Excel の構成

| No. | Sheet Name | 内容 |
|-----|-----------|------|
| 1 | Cover | 表紙（Study ID、SDTM版本、作成日） |
| 2 | Overview | 全シートの一覧と説明 |
| 3 | ChangeHistory | 更新履歴（手動記入用） |
| 4 | DatasetList | 全ドメインの概要一覧 |
| 5 | DomainTree | ドメイン内分類のツリー表示（CAT→SCAT→TRT/DECOD/TESTCD） |
| 6 | DomainTable | ドメイン内分類のテーブル表示 |
| 7 | DataTrace | EDC→SDTM マッピング追跡 |
| 8~N | CM, DM, DS... | 各ドメインの変数一覧 |
| N+1 | Codelist | コードリスト一覧（CDISC CT 照合結果） |
| N+2 | Notes | 備考（手動記入用） |

## ヘルプ

```bash
sdtm-spec-creator.exe --help
```
