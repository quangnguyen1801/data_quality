use std::collections::HashMap;

use anyhow::Ok;
use async_std::path::Path;
use futures::future::ok;
use model::{
    models::data_set,
    modelviews::{
        configtestparameter_view::ConfigTestParameterView, dataset_view::DatasetView,
        matrixexecute_view::MatrixExecuteView, metric_view::MetricView,
        notification_view::NotificationView, scope_view::ScopeView, selector_view::SelectorView,
        test_view::TestView, testparameter_view::TestParameterView,
    },
    shared::ultihelper::{ParameterSet, TestExecution},
};

use crate::{
    iservices::{
        idataset_service::IDatasetService, imetric_service::IMetricService,
        iselector_service::ISelectorService,
    },
    services::{
        dataset_service::DatasetService, metric_service::MetricService,
        selector_service::SelectorService,
    },
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

    pub async fn fn_com_load_data_by_scope(
        datasets: Vec<DatasetView>,
        selectors: Vec<SelectorView>,
        metrics: Vec<MetricView>,
        scopes: Vec<ScopeView>,
    ) -> anyhow::Result<(
        HashMap<String, Vec<HashMap<String, String>>>,
        HashMap<String, Vec<String>>,
    )> {
        let mut data_result = HashMap::new();
        let mut metric_result = HashMap::new();
        for scope in scopes {
            let data_set_id = scope.dataset_id;
            let selector_id = scope.selector_id;
            let metric_ids = scope.metric_id;
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
            data_result.insert(scope.id.to_string(), data_resut);
            if metric_ids.len() > 0 {
                let mut columns = Vec::new();
                for id in metric_ids.iter() {
                    if MetricService::fn_ser_is_exists(*id, metrics.clone()).await? == true {
                        let metric = metrics.iter().cloned().find(|f| f.id == *id).unwrap();
                        columns.push(metric.name);
                    }
                }
                metric_result.insert(scope.id.to_string(), columns);
            }
        }
        Ok((data_result, metric_result))
    }

    pub async fn fn_com_build_execution_plans() -> anyhow::Result<Vec<TestExecution>> {
        let configs: (
            Vec<DatasetView>,
            Vec<SelectorView>,
            Vec<MetricView>,
            Vec<ScopeView>,
            Vec<TestView>,
            Vec<TestParameterView>,
            Vec<ConfigTestParameterView>,
            Vec<MatrixExecuteView>,
            Vec<NotificationView>,
        ) = Self::fn_com_load_configs_by_file("".to_string())
            .await
            .unwrap();
        let data = Self::fn_com_load_data_by_scope(configs.0, configs.1, configs.2, configs.3)
            .await
            .unwrap();
        let parameters = Self::fn_com_parameter_execution_set(configs.5, configs.6)
            .await
            .unwrap();
        let list_testcase_executions =
            Self::fn_com_test_execution_set(configs.4, parameters, data.0, data.1)
                .await
                .unwrap();
        Ok(list_testcase_executions)
    }

    pub async fn fn_com_parameter_execution_set(
        parameters: Vec<TestParameterView>,
        config_parameters: Vec<ConfigTestParameterView>,
    ) -> anyhow::Result<HashMap<String, HashMap<String, Vec<ParameterSet>>>> {
        let mut list_params: HashMap<String, HashMap<String, Vec<ParameterSet>>> = HashMap::new();
        for cg_param in config_parameters.iter().cloned() {
            let test_id = cg_param.test_id;
            let group_id = cg_param.id;
            let pr = parameters
                .iter()
                .cloned()
                .find(|f| f.id == cg_param.test_parameter_id)
                .unwrap();
            let group_params = list_params
                .entry(test_id.to_string())
                .or_insert_with(|| HashMap::new());
            let prl = group_params
                .entry(group_id.to_string())
                .or_insert_with(|| Vec::new());
            prl.push(ParameterSet {
                param_group: cg_param.id,
                param_id: cg_param.test_parameter_id,
                param_name: pr.name,
                param_value: vec![cg_param.vlow, cg_param.vhigh],
                operator: cg_param.operator,
                incl_excl: cg_param.inc_excl,
            });
        }
        Ok(list_params)
    }

    pub async fn fn_com_test_execution_set(
        testcases: Vec<TestView>,
        mut parameters: HashMap<String, HashMap<String, Vec<ParameterSet>>>,
        mut datasets: HashMap<String, Vec<HashMap<String, String>>>,
        mut columns_check: HashMap<String, Vec<String>>,
    ) -> anyhow::Result<Vec<TestExecution>> {
        let mut config_test_executions: Vec<TestExecution> = Vec::new();
        for t in testcases.iter().cloned() {
            let prg = parameters.entry(t.id.to_string()).or_default().clone();
            for p in prg.values().cloned() {
                config_test_executions.push(TestExecution {
                    test_id: t.id,
                    test_name: t.description.clone(),
                    test_parameter: p,
                    test_data: datasets.entry(t.id.to_string()).or_default().clone(),
                    test_columns: columns_check.entry(t.id.to_string()).or_default().clone(),
                });
            }
        }
        Ok(config_test_executions)
    }
}
