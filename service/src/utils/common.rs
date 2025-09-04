use std::collections::HashMap;

use anyhow::Ok;
use async_std::path::Path;
use model::{
    models::data_set,
    modelviews::{
        configtestparameter_view::ConfigTestParameterView, dataset_view::DatasetView,
        matrixexecute_view::MatrixExecuteView, metric_view::MetricView,
        notification_view::NotificationView, scope_view::ScopeView, selector_view::SelectorView,
        test_view::TestView, testparameter_view::TestParameterView,
    },
};

use crate::{
    iservices::{idataset_service::IDatasetService, iselector_service::ISelectorService},
    services::{dataset_service::DatasetService, selector_service::SelectorService},
    utils::helpers::Helper,
};

pub struct Common {}
impl Common {
    pub async fn fn_com_load_configs_by_file(
        path: String,
    ) -> anyhow::Result<(
        Vec<DatasetView>,
        Vec<SelectorView>,
        Vec<MetricView>,
        Vec<ScopeView>,
        Vec<TestView>,
        Vec<TestParameterView>,
        Vec<ConfigTestParameterView>,
        Vec<MatrixExecuteView>,
        Vec<NotificationView>,
    )> {
        //Dataset
        let dataset =
            Helper::fn_help_read_excel_file::<DatasetView>(path.clone(), "Dataset".to_string())
                .await?;
        //Selector
        let selector =
            Helper::fn_help_read_excel_file::<SelectorView>(path.clone(), "Selector".to_string())
                .await?;
        //Metric
        let metric =
            Helper::fn_help_read_excel_file::<MetricView>(path.clone(), "Metric".to_string())
                .await?;
        //Scope
        let scope =
            Helper::fn_help_read_excel_file::<ScopeView>(path.clone(), "Scope".to_string()).await?;
        //test
        let test =
            Helper::fn_help_read_excel_file::<TestView>(path.clone(), "Test".to_string()).await?;
        //TestParameter
        let testparameter = Helper::fn_help_read_excel_file::<TestParameterView>(
            path.clone(),
            "TestParameter".to_string(),
        )
        .await?;
        //TestParameterSet
        let testparameterset = Helper::fn_help_read_excel_file::<ConfigTestParameterView>(
            path.clone(),
            "TestParameterSet".to_string(),
        )
        .await?;
        //Matrix
        let matrix = Helper::fn_help_read_excel_file::<MatrixExecuteView>(
            path.clone(),
            "Matrix".to_string(),
        )
        .await?;
        //Notification
        let notification =
            Helper::fn_help_read_excel_file::<NotificationView>(path, "Notification".to_string())
                .await?;
        Ok((
            dataset,
            selector,
            metric,
            scope,
            test,
            testparameter,
            testparameterset,
            matrix,
            notification,
        ))
    }

    pub async fn fn_com_load_dataset(
        datasets: Vec<DatasetView>,
        selectors: Vec<SelectorView>,
        metrics: Vec<MetricView>,
        scopes: Vec<ScopeView>,
    ) -> anyhow::Result<HashMap<String, Vec<HashMap<String, String>>>> {
        let mut result = HashMap::new();
        for scope in scopes {
            let data_set_id = scope.dataset_id;
            let selector_id = scope.selector_id;
            let metric_id = scope.metric_id;
            if DatasetService::fn_ser_is_exists(data_set_id, datasets.clone()).await? == false {
                continue;
            }

            let data_set = datasets
                .iter()
                .cloned()
                .find(|f| f.id == data_set_id)
                .unwrap();
            let file_name = format!("{}{}", data_set.name.clone(), ".csv");
            let data = Helper::fn_help_read_csv_file(data_set.path.clone(), file_name).await?;
            let mut data_resut = Vec::new();
            if SelectorService::fn_ser_is_exists(selector_id, selectors.clone()).await? == true {
                let selector: Vec<SelectorView> = selectors
                    .iter()
                    .cloned()
                    .filter(|f| f.id == selector_id)
                    .collect();
                for chunk in data.chunks(1000) {
                    let data_chunk =
                        Helper::fn_help_apply_selector_to_chunk(chunk, &selector).await;
                    data_resut.extend_from_slice(&data_chunk);
                }
            }
            result.insert(scope.id.to_string(), data_resut);
        }

        Ok(result)
    }
}
