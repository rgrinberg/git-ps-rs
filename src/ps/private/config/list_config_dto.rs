use std::option::Option;
use serde::Deserialize;
use super::super::utils;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ListConfigDto {
  pub add_additional_info: Option<bool>,
  pub additional_info_hook_output_length: Option<usize>
}

impl utils::Mergable for ListConfigDto {
  /// Merge the provided b with self overriding with any present values
  fn merge(&self, b: &Self) -> Self {
    ListConfigDto {
      add_additional_info: b.add_additional_info.or(self.add_additional_info),
      additional_info_hook_output_length: b.additional_info_hook_output_length.or(self.additional_info_hook_output_length),
    }
  }
}
