# VAPORCONE Pipeline

Core processing pipeline for the SDTM Spec Forge system.

## Module Architecture

```
pipeline/
├── Base Components (BC)
│   ├── VC_BC01_constant.py              # Constants & configuration loading
│   ├── VC_BC02_baseUtils.py             # Logging, DB, filesystem utilities
│   ├── VC_BC03_fetchConfig.py           # Excel config parsing
│   ├── VC_BC04_operateType.py           # Data transformation dispatcher
│   ├── VC_BC05_studyFunctions.py        # Study-specific functions (per study)
│   └── VC_BC06_operateTypeFunctions.py  # Operation type implementations
├── Operations (OP)
│   ├── VC_OP01_cleaning.py              # Data cleaning
│   ├── VC_OP02_insertCodeList.py        # CodeList insertion
│   ├── VC_OP03_insertMetadata.py        # Metadata insertion
│   ├── VC_OP04_format.py                # Data formatting
│   └── VC_OP05_mapping.py               # SDTM field mapping
├── Post-Processing (PS)
│   ├── VC_PS01_makeInputCSV.py          # Generate submission CSVs
│   └── VC_PS02_csv2json.py              # CSV → JSON → M5 ZIP package
├── studySpecific/
│   └── <STUDY_ID>/VC_BC05_studyFunctions.py  # Per-study logic
└── experiment/                           # Experimental features
```

## Dependency Flow

```
VC_BC01 (Constants)
  → VC_BC02 (Utils) + VC_BC03 (Config Parser)
    → VC_BC04 (Dispatcher) + VC_BC06 (Op Functions)
      → VC_BC05 (Study-Specific, loaded dynamically)
        → VC_OP01–05 (Processing Stages)
          → VC_PS01–02 (Output Generation)
```

## Operation Types

The mapping engine supports 8 operation types:

| Type | Description |
|------|-------------|
| `DEF` | Default / fixed value assignment |
| `FIX` | Direct field copy |
| `FLG` | Flag field (conditional Y/blank) |
| `IIF` | Inline-if conditional mapping |
| `COB` | Coalesce (first non-blank) |
| `CDL` | CodeList lookup |
| `PRF` | Prefix concatenation |
| `SEL` | Selective field mapping |

Custom operations can be added via `specialType()` in study-specific modules.

## Adding a New Study

1. Create `studySpecific/<YOUR_STUDY_ID>/VC_BC05_studyFunctions.py`
   - See `studySpecific/example_study/` for the expected pattern
2. Create `project.local.json` from the template:
   ```json
   {
     "STUDY_ID": "YOUR_STUDY_ID",
     "ROOT_PATH": "/path/to/pipeline",
     "RAW_DATA_ROOT_PATH": "/path/to/pipeline/studySpecific/YOUR_STUDY_ID/01_RawData",
     ...
   }
   ```
3. Prepare your `<STUDY_ID>_OperationConf.xlsx` with field mappings
4. Place raw data CSVs in `studySpecific/<YOUR_STUDY_ID>/01_RawData/`

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VC_DB_HOST` | `127.0.0.1` | MySQL host |
| `VC_DB_USER` | `root` | MySQL user |
| `VC_DB_PASSWORD` | `root` | MySQL password |
| `VC_DB_DATABASE` | `VC-DataMigration_2.0` | MySQL database name |
| `PROJECT_CONFIG_PATH` | `./project.local.json` | Config file override |

### project.local.json

Machine-specific paths and study configuration. See `project.local.json.example`.
