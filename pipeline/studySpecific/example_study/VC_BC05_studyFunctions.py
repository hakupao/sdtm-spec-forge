#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Example study-specific functions module.

Each clinical study has its own VC_BC05_studyFunctions.py under
studySpecific/<STUDY_ID>/. This file demonstrates the expected patterns:

- filter_df_by_field(): Generic record filtering utility
- DM(): Example domain-building function (Demographics)

These functions are dynamically imported by the pipeline via
VC_BC04_operateType.py based on the STUDY_ID in project.local.json.
"""

from VC_BC03_fetchConfig import *
from VC_BC04_operateType import *
import numpy as np
import pandas as pd


def filter_df_by_field(source, **filters):
    """
    Filter a data table by a single field-value condition.

    Usage:
        filter_df_by_field('TABLE_NAME', FieldName='value')
        filter_df_by_field(existing_df, FieldName='value')

    Args:
        source: Table name (str) or DataFrame
        **filters: Exactly one field_name=value pair

    Returns:
        pd.DataFrame: Filtered DataFrame with all-blank columns removed
    """
    if source is None:
        raise ValueError("source must not be None.")

    if isinstance(source, str):
        format_dataset = getFormatDataset(source)
        if source not in format_dataset:
            raise KeyError(f"Table '{source}' not found in format dataset.")
        df = format_dataset[source].copy()
    elif isinstance(source, pandas.DataFrame):
        df = source.copy()
    else:
        raise TypeError("source must be a DataFrame or table name string.")

    if len(filters) != 1:
        raise ValueError("filters must contain exactly one condition.")

    field_name, value = next(iter(filters.items()))

    if field_name not in df.columns:
        raise KeyError(f"Column '{field_name}' not found in data.")

    target_value = '' if value is None else str(value)
    series = df[field_name].fillna('').astype(str).str.strip()
    filtered_df = df.loc[series == target_value].copy()

    # Drop all-blank columns
    if not filtered_df.empty:
        non_blank_cols = (
            filtered_df.fillna('')
            .astype(str)
            .apply(lambda s: s.str.strip().ne(''))
            .any(axis=0)
        )
        filtered_df = filtered_df.loc[:, non_blank_cols]

    return filtered_df.fillna('').astype(str)


def DM():
    """
    Example: Build the Demographics (DM) domain.

    This function demonstrates the typical pattern for study-specific
    domain construction:
    1. Fetch source tables via getFormatDataset()
    2. Merge/join tables on SUBJID
    3. Derive computed fields
    4. Return a string-typed DataFrame

    Customize this function for your study's specific data sources
    and derivation rules.
    """
    # 1. Fetch source tables
    format_dataset = getFormatDataset('REGISTRATION', 'FOLLOWUP')

    reg_df = format_dataset['REGISTRATION'].copy()
    followup_df = format_dataset['FOLLOWUP'][['SUBJID', 'LAST_VISIT_DATE']].copy()

    # 2. Merge tables
    dm_df = pd.merge(reg_df, followup_df, on='SUBJID', how='left')
    dm_df = dm_df.fillna('')

    # 3. Derive fields (example: RFENDTC from last visit date)
    dm_df['RFENDTC'] = dm_df['LAST_VISIT_DATE']

    return dm_df.astype(str)
