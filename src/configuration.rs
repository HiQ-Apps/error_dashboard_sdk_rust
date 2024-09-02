#[derive(Debug, Clone)]
pub struct Configuration {
    verbose: bool,
    sampling_rate: usize,
    // milliseconds
    max_age: u64,
    // milliseconds
    retry_delay: u64,
    retry_attempts: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            verbose: false,
            sampling_rate: 2,
            max_age: 20000,
            retry_delay: 3000,
            retry_attempts: 3,
        }
    }
}

pub enum ConfigKey {
    Verbose,
    SamplingRate,
    MaxAge,
    RetryDelay,
    RetryAttempts,
}

pub enum ConfigValue {
    Bool(bool),
    Usize(usize),
    U64(u64),
}

// Struct for partial configurations
#[derive(Debug, Clone)]
pub struct PartialConfigs {
    pub verbose: Option<bool>,
    pub sampling_rate: Option<usize>,
    pub max_age: Option<u64>,
    pub retry_delay: Option<u64>,
    pub retry_attempts: Option<usize>,
}


impl Configuration {
    pub fn new(configs: Option<PartialConfigs>) -> Self {
        let mut config = Configuration::default();

        if let Some(overrides) = configs {
            if let Some(verbose) = overrides.verbose {
                config.verbose = verbose;
            }
            if let Some(sampling_rate) = overrides.sampling_rate {
                if sampling_rate <= 0 {
                    panic!("sampling_rate must be a positive number");
                }
                config.sampling_rate = sampling_rate;
            }
            if let Some(max_age) = overrides.max_age {
                if max_age <= 0 {
                    panic!("max_age must be a positive number");
                }
                config.max_age = max_age;
            }
            if let Some(retry_delay) = overrides.retry_delay {
                if retry_delay <= 0 {
                    panic!("retry_delay must be a positive number");
                }
                config.retry_delay = retry_delay;
            }
            if let Some(retry_attempts) = overrides.retry_attempts {
                if retry_attempts <= 0 {
                    panic!("retry_attempts must be a positive number");
                }
                config.retry_attempts = retry_attempts;
            }
        }

        config
    }

    pub fn get_config(&self, key: ConfigKey) -> ConfigValue {
        match key {
            ConfigKey::Verbose => ConfigValue::Bool(self.verbose),
            ConfigKey::SamplingRate => ConfigValue::Usize(self.sampling_rate),
            ConfigKey::MaxAge => ConfigValue::U64(self.max_age),
            ConfigKey::RetryDelay => ConfigValue::U64(self.retry_delay),
            ConfigKey::RetryAttempts => ConfigValue::Usize(self.retry_attempts),
        }
    }

    pub fn set_config(&mut self, key: ConfigKey, value: ConfigValue) {
        match (key, value) {
            (ConfigKey::Verbose, ConfigValue::Bool(val)) => self.verbose = val,
            (ConfigKey::SamplingRate, ConfigValue::Usize(val)) => {
                if val <= 0 {
                    panic!("sampling_rate must be a positive number");
                }
                self.sampling_rate = val
            }
            (ConfigKey::MaxAge, ConfigValue::U64(val)) => {
                if val <= 0 {
                    panic!("max_age must be a positive number");
                }
                self.max_age = val
            }
            (ConfigKey::RetryDelay, ConfigValue::U64(val)) => {
                if val <= 0 {
                    panic!("retry_delay must be a positive number");
                }
                self.retry_delay = val
            }
            (ConfigKey::RetryAttempts, ConfigValue::Usize(val)) => {
                if val <= 0 {
                    panic!("retry_attempts must be a positive number");
                }
                self.retry_attempts = val
            }
            _ => panic!("Invalid type for the given key"),
        }
    }
}
