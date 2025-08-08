
#[derive(Debug, Clone)]
pub struct Config {
    pub max_workers: usize,
    pub min_workers: usize,
    pub rate_limit: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_workers: 20,
            min_workers: 5,
            rate_limit: 10,
        }
    }
}