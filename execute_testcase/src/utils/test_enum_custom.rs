use strum_macros::EnumIter;

use crate::{
    testcases::{
        test_case_forex_daily_is_in_check::TestCaseForexDailyIsInCheck,
        test_case_forex_daily_outlier_tolerance::TestCaseForexDailyOutlierTolerance,
    },
    utils::test_logic::TestLogic,
};

#[derive(Debug, EnumIter)]
pub enum EnumTestType {
    TestCaseForexDailyIsInCheck,
    TestCaseForexDailyOutlierTolerance,
}

impl EnumTestType {
    pub fn fn_enum_from_name(test_name: String) -> Option<Self> {
        match test_name.as_str() {
            "Test1" => Some(Self::TestCaseForexDailyIsInCheck),
            "Test2" => Some(Self::TestCaseForexDailyOutlierTolerance),
            _ => None,
        }
    }

    pub fn fn_enum_get_name(&self) -> &'static str {
        match self {
            EnumTestType::TestCaseForexDailyIsInCheck => "test1",
            EnumTestType::TestCaseForexDailyOutlierTolerance => "test2",
        }
    }

    pub fn fn_enum_get_logic(&self) -> Box<dyn TestLogic> {
        match self {
            EnumTestType::TestCaseForexDailyIsInCheck => Box::new(TestCaseForexDailyIsInCheck),
            EnumTestType::TestCaseForexDailyOutlierTolerance => {
                Box::new(TestCaseForexDailyOutlierTolerance)
            }
        }
    }
}
