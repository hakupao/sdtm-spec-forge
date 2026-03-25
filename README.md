# SDTM Spec Forge

A config-driven Python pipeline that transforms raw clinical trial data into [CDISC SDTM](https://www.cdisc.org/standards/foundational/sdtm) compliant datasets and M5 submission packages.

## Overview

SDTM Spec Forge (codename **VAPORCONE**) automates the end-to-end conversion of raw clinical data into regulatory-ready SDTM tabulation datasets. It supports 80+ SDTM domains and generates complete M5 data packages (ZIP) suitable for regulatory submission.

### Pipeline

```
Raw Data (CSV)
  → VC_OP01  Cleaning        — field filtering, row validation
  → VC_OP02  CodeList         — terminology code list insertion
  → VC_OP03  Metadata         — metadata enrichment via MySQL
  → VC_OP04  Format           — data formatting with indexes
  → VC_OP05  Mapping          — SDTM field mapping (multiprocessing)
  → VC_PS01  Input CSV        — generate submission-ready CSVs
  → VC_PS02  JSON Package     — CSV → JSON → M5 ZIP archive
```

## Requirements

- Python 3.11+
- MySQL 8.0+

### Python Dependencies

```
mysql-connector-python >= 9.4.0
pandas >= 2.3.1
numpy >= 2.2.6
openpyxl >= 3.1.5
python-dateutil >= 2.9.0
```

## Quick Start

```bash
# 1. Clone and install dependencies
git clone https://github.com/hakupao/sdtm-spec-forge.git
cd sdtm-spec-forge/pipeline
pip install -r requirements.txt

# 2. Create local config from template
cp project.local.json.example project.local.json
# Edit project.local.json with your study ID and paths

# 3. (Optional) Configure database via environment variables
export VC_DB_HOST=127.0.0.1
export VC_DB_USER=root
export VC_DB_PASSWORD=yourpassword
export VC_DB_DATABASE=VC-DataMigration_2.0

# 4. Prepare your study data
#    - Create studySpecific/<YOUR_STUDY_ID>/VC_BC05_studyFunctions.py
#    - Place raw CSVs in studySpecific/<YOUR_STUDY_ID>/01_RawData/
#    - Place your OperationConf.xlsx in the data directory
#    - Place SDTMIG and Terminology XLSX in the master directory

# 5. Run the pipeline
python VC_OP01_cleaning.py
python VC_OP02_insertCodeList.py
python VC_OP03_insertMetadata.py
python VC_OP04_format.py
python VC_OP05_mapping.py
python VC_PS01_makeInputCSV.py
python VC_PS02_csv2json.py
```

## Project Structure

```
sdtm-spec-forge/
├── pipeline/                       # Core application
│   ├── VC_BC01_constant.py         # Constants & configuration
│   ├── VC_BC02_baseUtils.py        # Logging, DB, filesystem utilities
│   ├── VC_BC03_fetchConfig.py      # Excel config parser
│   ├── VC_BC04_operateType.py      # Data transformation dispatcher
│   ├── VC_BC06_operateTypeFunctions.py  # Operation type implementations
│   ├── VC_OP01–OP05*.py            # Pipeline stage scripts
│   ├── VC_PS01–PS02*.py            # Post-processing & packaging
│   ├── studySpecific/              # Per-study functions
│   │   └── example_study/          # Example study template
│   └── experiment/                 # Experimental features
├── config.toml                     # Root configuration template
└── docs/                           # Requirements documentation
```

## Configuration

| File | Purpose |
|------|---------|
| `config.toml` | Root-level study ID and path configuration |
| `pipeline/project.local.json` | Machine-specific paths and DB table names (gitignored) |
| `<STUDY_ID>_OperationConf.xlsx` | Study-specific field mappings, code lists, and processing rules |

## Multi-Study Support

Each clinical study has its own directory under `pipeline/studySpecific/` with:
- `VC_BC05_studyFunctions.py` — study-specific data joins and transformations
- Timestamped output folders for each pipeline run

See `pipeline/studySpecific/example_study/` for the expected pattern.

## License

MIT License. See [LICENSE](LICENSE) for details.
