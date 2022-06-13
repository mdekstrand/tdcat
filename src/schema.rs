//! Definitions for tabular data schemas.

/// Tabular schema data types.
#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
  Unknown,
  Logical,
  Integer,
  Numeric,
  String,
  Date,
  DateTime,
}

/// Column specifier
#[derive(Debug, PartialEq, Eq)]
pub struct Column {
  pub name: String,
  pub dtype: DataType,
}
