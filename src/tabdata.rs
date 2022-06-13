//! Tabular data interfaces.

/// Trait defining the tabular data API.
pub trait TabDataSource {
  /// Get a name for this data source for display.
  fn name(&self) -> Option<&str>;

  
}
