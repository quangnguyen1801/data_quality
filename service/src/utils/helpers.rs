use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Ok};
use async_std::{fs::File, io::BufReader};
use calamine::{RangeDeserializerBuilder, Reader, open_workbook_auto};
use chrono::{DateTime, NaiveDateTime, Utc};
use csv_async::AsyncReaderBuilder;
use futures::{StreamExt, future::ok};
use model::{modelviews::selector_view::SelectorView, shared::ultihelper::CONFIGS};
use serde::de::DeserializeOwned;

use crate::{
    iservices::isettingversion_service::ISettingVersionService,
    services::settingversion_service::SettingVersionService,
};

pub struct Helper {}
impl Helper {
    pub async fn fn_ser_help_read_excel_file<T>(
        path: String,
        sheet_name: String,
    ) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut wb =
            open_workbook_auto(&path).with_context(|| format!("Can not file excel: {}", path))?;
        let range = wb
            .worksheet_range(&sheet_name)
            .with_context(|| format!("Sheet '{}' not match!", sheet_name))?;
        let iter = RangeDeserializerBuilder::new().from_range(&range)?;
        let mut result = Vec::new();
        for row in iter {
            let record: T = row?;
            result.push(record);
        }
        Ok(result)
    }

    pub async fn fn_ser_help_read_csv_file(
        path: String,
        file_name: String,
    ) -> anyhow::Result<Vec<HashMap<String, String>>> {
        let full_path = Path::new(&path).join(&file_name);
        let file = File::open(full_path).await?;
        let buf_reader = BufReader::new(file);
        let mut csv_reader = AsyncReaderBuilder::new()
            .has_headers(true)
            .create_deserializer(buf_reader);

        let mut records = Vec::new();
        let mut stream = csv_reader.deserialize::<HashMap<String, String>>();

        while let Some(row) = stream.next().await {
            let record = row?;
            records.push(record);
        }
        Ok(records)
    }

    pub async fn fn_ser_help_apply_selector_to_chunk(
        chunk: &[HashMap<String, String>],
        selectors: &[SelectorView],
    ) -> Vec<HashMap<String, String>> {
        let mut result = Vec::new();
        for row in chunk {
            let mut pass = true;
            for selector in selectors {
                if selector.inc_excl == "E" {
                    continue;
                }
                let value = row.get(&selector.field_name).unwrap();
                let ok = Self::fn_ser_help_check_selector_condition(value, selector)
                    .await
                    .unwrap_or(false);
                if !ok {
                    pass = false;
                    break;
                }
            }
            if pass {
                result.push(row.clone());
            }
        }
        return result;
    }

    pub async fn fn_ser_help_check_selector_condition(
        value: &str,
        selector: &SelectorView,
    ) -> anyhow::Result<bool> {
        match selector.operator.as_str() {
            "EQ" => Ok(value == selector.vlow.as_str()),
            "NE" => Ok(value != selector.vlow.as_str()),
            "BT" => {
                let format = "%Y-%m-%d";
                if Self::fn_ser_help_is_datetime(value) {
                    let v = Self::fn_ser_help_parse_datetime(value, format).unwrap();
                    let low =
                        Self::fn_ser_help_parse_datetime(selector.vlow.as_str(), format).unwrap();
                    let high =
                        Self::fn_ser_help_parse_datetime(selector.vhigh.as_str(), format).unwrap();
                    match (v, low, high) {
                        (v, l, h) => Ok(v >= l && v <= h),
                        _ => Ok(false),
                    }
                } else {
                    let v = value.parse::<f64>().unwrap_or(-9999.0);
                    let low = selector.vlow.parse::<f64>().unwrap_or(0.0);
                    let high = selector.vhigh.parse::<f64>().unwrap_or(9999.0);
                    match (v, low, high) {
                        (v, l, h) => Ok(v >= low && v <= high),
                        _ => Ok(false),
                    }
                }
            }
            "GE" => {
                let format = "%Y-%m-%d";
                if Self::fn_ser_help_is_datetime(value) {
                    let v = Self::fn_ser_help_parse_datetime(value, format).unwrap();
                    let low =
                        Self::fn_ser_help_parse_datetime(selector.vlow.as_str(), format).unwrap();
                    match (v, low) {
                        (v, l) => Ok(v >= l),
                        _ => Ok(false),
                    }
                } else {
                    let v = value.parse::<f64>().unwrap_or(-9999.0);
                    let low = selector.vlow.parse::<f64>().unwrap_or(0.0);
                    match (v, low) {
                        (v, l) => Ok(v >= l),
                        _ => Ok(false),
                    }
                }
            }
            "GT" => {
                let format = "%Y-%m-%d";
                if Self::fn_ser_help_is_datetime(value) {
                    let v = Self::fn_ser_help_parse_datetime(value, format).unwrap();
                    let low =
                        Self::fn_ser_help_parse_datetime(selector.vlow.as_str(), format).unwrap();
                    match (v, low) {
                        (v, l) => Ok(v > l),
                        _ => Ok(false),
                    }
                } else {
                    let v = value.parse::<f64>().unwrap_or(-9999.0);
                    let low = selector.vlow.parse::<f64>().unwrap_or(0.0);
                    match (v, low) {
                        (v, l) => Ok(v > l),
                        _ => Ok(false),
                    }
                }
            }
            "LE" => {
                let format = "%Y-%m-%d";
                if Self::fn_ser_help_is_datetime(value) {
                    let v = Self::fn_ser_help_parse_datetime(value, format).unwrap();
                    let high =
                        Self::fn_ser_help_parse_datetime(selector.vhigh.as_str(), format).unwrap();
                    match (v, high) {
                        (v, h) => Ok(v <= h),
                        _ => Ok(false),
                    }
                } else {
                    let v = value.parse::<f64>().unwrap_or(-9999.0);
                    let high = selector.vhigh.parse::<f64>().unwrap_or(0.0);
                    match (v, high) {
                        (v, h) => Ok(v <= h),
                        _ => Ok(false),
                    }
                }
            }
            "LT" => {
                let format = "%Y-%m-%d";
                if Self::fn_ser_help_is_datetime(value) {
                    let v = Self::fn_ser_help_parse_datetime(value, format).unwrap();
                    let high =
                        Self::fn_ser_help_parse_datetime(selector.vhigh.as_str(), format).unwrap();
                    match (v, high) {
                        (v, h) => Ok(v < h),
                        _ => Ok(false),
                    }
                } else {
                    let v = value.parse::<f64>().unwrap_or(-9999.0);
                    let high = selector.vhigh.parse::<f64>().unwrap_or(0.0);
                    match (v, high) {
                        (v, h) => Ok(v < h),
                        _ => Ok(false),
                    }
                }
            }
            "CP" => {
                let pattern = selector.vlow.as_str();
                Ok(value.contains(pattern))
            }
            "NCP" => {
                let pattern = selector.vlow.as_str();
                Ok(!value.contains(pattern))
            }
            _ => Ok(true),
        }
    }

    pub fn fn_ser_help_is_datetime(s: &str) -> bool {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d").is_ok()
    }

    pub fn fn_ser_help_parse_datetime(s: &str, format: &str) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(s, format).ok()
    }

    pub async fn fn_ser_help_is_setting_from_excel(path: String) -> anyhow::Result<(bool, i32)> {
        let lastest_version = SettingVersionService::fn_ser_get_current_version().await?;
        let file_info = fs::metadata(Path::new(&path)).unwrap();
        let modified = file_info.modified()?;
        let file_time: DateTime<Utc> = modified.into();
        if file_time > lastest_version.upddate {
            Ok((true, 0))
        } else {
            Ok((false, lastest_version.id))
        }
    }
}
