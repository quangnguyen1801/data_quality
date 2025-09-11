#[derive(Debug, Clone, Default)]
pub struct TestResult {
    pub scope_id: i32,
    pub test_id: i32,
    pub checks: Vec<CheckResult>,
}
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct CheckResult {
    pub passed: bool,
    pub message: Option<String>,
}

impl TestResult {
    pub fn fn_run_com_test_add_check(&mut self, passed: bool, message: Option<String>) {
        self.checks.push(CheckResult { passed, message });
    }
}
