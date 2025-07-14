#[derive(Debug)]
pub struct UpdateOptions {
    pub force: bool,
    pub db_only: bool,
    pub chart_only: bool,
    pub keep_raw: bool,
    pub analyze: bool,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        Self {
            force: false,
            db_only: false,
            chart_only: false,
            keep_raw: false,
            analyze: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UpdateResult {
    Updated,
    NoUpdate,
    AnalysisComplete,
}