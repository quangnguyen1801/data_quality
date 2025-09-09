pub struct TestResult {
    pub scope_id: i32,
    pub test_id: i32,
    pub checks: Vec<CheckResult>,
}
pub struct CheckResult {
    pub metric: String,
    pub passed: bool,
    pub message: Option<String>,
}

impl TestResult {
    pub fn fn_com_test_add_check(&mut self, metric: String, passed: bool, message: Option<String>) {
        self.checks.push(CheckResult {
            metric,
            passed,
            message,
        });
    }
}
