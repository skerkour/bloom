use crate::Error;
use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};
use stdx::{crypto, dotenv, encoding::base64, mail, url::Url, vat};

const ENV_APP_ENV: &str = "APP_ENV";
const ENV_APP_BASE_URL: &str = "APP_BASE_URL";
const ENV_APP_MASTER_KEY: &str = "APP_MASTER_KEY";
const ENV_APP_OLD_MASTER_KEY: &str = "APP_OLD_MASTER_KEY";
const ENV_APP_SELF_HOSTED: &str = "APP_SELF_HOSTED";
const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_POOL_SIZE: &str = "DATABASE_POOL_SIZE";
const ENV_HTTP_PORT: &str = "PORT";
const ENV_HTTP_ACCESS_LOGS: &str = "HTTP_ACCESS_LOGS";
const ENV_HTTP_PUBLIC_DIRECTORY: &str = "HTTP_PUBLIC_DIRECTORY";
const ENV_MAIL_DRIVER: &str = "MAIL_DRIVER";
const ENV_MAIL_NOTIFY_ADDRESS: &str = "MAIL_NOTIFY_ADDRESS";
const ENV_MAIL_NEWSLETTER_ADDRESS: &str = "MAIL_NEWSLETTER_ADDRESS";
const ENV_MAIL_BLOCKLIST: &str = "MAIL_BLOCKLIST";
const ENV_STORAGE_DRIVER: &str = "STORAGE_DRIVER";
const ENV_STORAGE_BASE_DIRECTORY: &str = "STORAGE_BASE_DIRECTORY";
const ENV_STRIPE_SECRET_KEY: &str = "STRIPE_SECRET_KEY";
const ENV_STRIPE_PUBLIC_KEY: &str = "STRIPE_PUBLIC_KEY";
const ENV_STRIPE_WEBHOOK_SECRET: &str = "STRIPE_WEBHOOK_SECRET";
const ENV_STRIPE_DATA: &str = "STRIPE_DATA";
const ENV_AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
const ENV_AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const ENV_AWS_DEFAULT_REGION: &str = "AWS_DEFAULT_REGION";
const ENV_SMTP_PORT: &str = "SMTP_PORT";
const ENV_SMTP_HOST: &str = "SMTP_HOST";
const ENV_SMTP_USERNAME: &str = "SMTP_USERNAME";
const ENV_SMTP_PASSWORD: &str = "SMTP_PASSWORD";
const ENV_S3_REGION: &str = "S3_REGION";
const ENV_S3_BUCKET: &str = "S3_BUCKET";
const ENV_SES_REGION: &str = "SES_REGION";
const ENV_WORKER_CONCURRENCY: &str = "WORKER_CONCURRENCY";
const ENV_SENTRY_SECURITY_REPORT_URI: &str = "SENTRY_SECURITY_REPORT_URI";
const ENV_SENTRY_INGEST_DOMAIN: &str = "SENTRY_INGEST_DOMAIN";
const ENV_SENTRY_DSN: &str = "SENTRY_DSN";

const POSTGRES_SCHEME: &str = "postgres";
const STRIPE_PRODUCT_PREFIX: &str = "prod_";
const STRIPE_PRICE_PREFIX: &str = "price_";
const STRIPE_TAX_PREFIX: &str = "txr_";
const STRIPE_PUBLIC_KEY_PREFIX: &str = "pk_";
const STRIPE_SECRET_KEY_PREFIX: &str = "sk_";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub env: Env,
    pub base_url: String,
    pub master_key: Vec<u8>,
    pub old_master_key: Option<Vec<u8>>, // used for key rotation
    pub self_hosted: bool,
    pub countries: HashMap<String, String>,
    pub http: Http,
    pub database: Database,
    pub smtp: Smtp,
    pub mail: Mail,
    pub storage: Storage,
    pub stripe: Option<Stripe>,
    pub aws: Aws,
    pub ses: Ses,
    pub s3: S3,
    pub worker: Worker,
    pub sentry: Sentry,
}
const DEFAULT_APP_SELF_HOSTED: bool = false;
const APP_ENV_DEV: &str = "dev";
const APP_ENV_STAGING: &str = "staging";
const APP_ENV_PRODUCTION: &str = "production";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Env {
    Dev,
    Staging,
    Production,
}

impl FromStr for Env {
    type Err = Error;

    fn from_str(s: &str) -> Result<Env, Error> {
        match s {
            APP_ENV_DEV => Ok(Env::Dev),
            APP_ENV_STAGING => Ok(Env::Staging),
            APP_ENV_PRODUCTION => Ok(Env::Production),
            _ => Err(Error::InvalidArgument(format!(
                "config: {} is not a valid env. Valid values are [{}, {}, {}]",
                s,
                Env::Dev,
                Env::Staging,
                Env::Production,
            ))),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Dev => write!(f, "{}", APP_ENV_DEV),
            Env::Staging => write!(f, "{}", APP_ENV_STAGING),
            Env::Production => write!(f, "{}", APP_ENV_PRODUCTION),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub code: String,
}

/// Database contains the data necessary to connect to a database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}
const DEFAULT_DATABASE_POOL_SIZE: u32 = 100;

/// Http contains the data specific to the HTTP(s) server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Http {
    pub port: u16,
    pub access_logs: bool,
    pub public_directory: String,
    // pub https_certs_directory: String,
    // pub https_certs_email: String,
    // pub https_domain: String,
    // pub https_port: u16,
}
const DEFAULT_HTTP_PORT: u16 = 8080;
const DEFAULT_ACCESS_LOGS: bool = false;
const DEFAULT_HTTP_PUBLIC_DIRECTORY: &str = "public";
// const ENV_HTTPS_CERTS_DIRECTORY: &str = "ENV_HTTPS_CERTS_DIRECTORY";
// const ENV_HTTPS_CERTS_EMAIL: &str = "HTTPS_CERTS_EMAIL";
// const ENV_HTTPS_DOMAIN: &str = "HTTPS_DOMAIN";
// const ENV_HTTPS_PORT: &str = "HTTPS_PORT";
// const DEFAULT_HTTPS_CERT_DIRECTORY: &str = "certs";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    pub driver: MailDriver,
    pub notify_address: mail::Address,
    pub newsletter_address: mail::Address,
    pub domains_blocklist_file: String,
    pub domains_blocklist: HashSet<String>,
    // 	OutboundAddress  mail.Address `env:"MAIL_OUTBOUND_ADDRESS"`
}
const DEFAULT_MAIL_BLOCKLIST_FILE: &str = "email_domains_blocklist.txt";
const DEFAULT_MAIL_DRIVER: MailDriver = MailDriver::Ses;
const MAIL_DRIVER_SES: &str = "ses";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MailDriver {
    Ses,
}

impl FromStr for MailDriver {
    type Err = Error;

    fn from_str(s: &str) -> Result<MailDriver, Error> {
        match s {
            MAIL_DRIVER_SES => Ok(MailDriver::Ses),
            _ => Err(Error::InvalidArgument(format!(
                "config: {} is not a valid mail driver. Valid values are [{}]",
                s,
                MailDriver::Ses,
            ))),
        }
    }
}

impl fmt::Display for MailDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MailDriver::Ses => write!(f, "{}", MAIL_DRIVER_SES),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub driver: StorageDriver,
    pub base_directory: String,
}
const DEFAULT_STORAGE_DRIVER: StorageDriver = StorageDriver::S3;
const DEFAULT_STORAGE_BASE_DIRECTORY: &str = "";
const STORAGE_DRIVER_S3: &str = "s3";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageDriver {
    S3,
}

impl FromStr for StorageDriver {
    type Err = Error;

    fn from_str(s: &str) -> Result<StorageDriver, Error> {
        match s {
            STORAGE_DRIVER_S3 => Ok(StorageDriver::S3),
            _ => Err(Error::InvalidArgument(format!(
                "config: {} is not a valid storage driver. Valid values are [{}]",
                s,
                StorageDriver::S3,
            ))),
        }
    }
}

impl fmt::Display for StorageDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageDriver::S3 => write!(f, "{}", STORAGE_DRIVER_S3),
        }
    }
}

/// Stripe contains the data to connect to Stripe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stripe {
    pub secret_key: String,
    pub public_key: String,
    pub webhook_secret: String,
    pub data: StripeData,
    json_data: String,
    // StarterPlanID string `env:"STRIPE_STARTER_PLAN"`
    // ProPlanID     string `env:"STRIPE_PRO_PLAN"`
    // UltraPlanID   string `env:"STRIPE_ULTRA_PLAN"`
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeData {
    pub taxes: HashMap<String, String>,
    pub products: StripeProducts,
    pub prices: StripePrices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeProducts {
    pub starter: String,
    pub pro: String,
    // pub ultra: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripePrices {
    pub starter: String,
    pub pro: String,
    // pub ultra: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aws {
    pub secret_access_key: Option<String>,
    pub access_key_id: Option<String>,
    pub default_region: String,
    pub default_region_rusoto: Region,
}
const DEFAULT_AWS_REGION: &str = "eu-west-1"; // Ireland

/// Smtp contains the data necessary to send emails using the SMTP protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smtp {
    pub port: Option<u16>,
    pub host: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ses {
    pub region: String,
    pub region_rusoto: Region,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3 {
    pub region: String,
    pub bucket: String,
    pub region_rusoto: Region,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub concurrency: usize,
}
const DEFAULT_WORKER_CONCURRENCY: usize = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentry {
    pub security_report_uri: Option<String>,
    pub ingest_domain: Option<String>,
    pub dsn: Option<String>,
}

impl Config {
    /// Load and validate the configuration from the environment.
    /// If an error is found while parsing the values, or validating the data, an error is returned.
    pub fn load() -> Result<Config, Error> {
        dotenv::dotenv().ok();

        // app
        let env = std::env::var(ENV_APP_ENV)
            .map_err(|_| env_not_found(ENV_APP_ENV))?
            .parse::<Env>()?;
        let base_url = std::env::var(ENV_APP_BASE_URL).map_err(|_| env_not_found(ENV_APP_BASE_URL))?;
        let master_key = std::env::var(ENV_APP_MASTER_KEY)
            .map_err(|_| env_not_found(ENV_APP_MASTER_KEY))
            .map(base64::decode)??;
        let old_master_key = std::env::var(ENV_APP_OLD_MASTER_KEY)
            .ok()
            .map_or(Ok(None), |env_val| base64::decode(env_val).map(Some))?;
        let self_hosted = std::env::var(ENV_APP_SELF_HOSTED)
            .ok()
            .map_or(Ok(DEFAULT_APP_SELF_HOSTED), |env_val| env_val.parse::<bool>())?;
        let countries_data_json = include_str!("../../countries.json");
        let countries_vec: Vec<Country> = serde_json::from_str(countries_data_json)?;
        let countries: HashMap<String, String> = countries_vec
            .into_iter()
            .map(|country| (country.code, country.name))
            .collect();

        // http
        let http_port = std::env::var(ENV_HTTP_PORT)
            .ok()
            .map_or(Ok(DEFAULT_HTTP_PORT), |env_val| env_val.parse::<u16>())?;
        let http_access_logs = std::env::var(ENV_HTTP_ACCESS_LOGS)
            .ok()
            .map_or(Ok(DEFAULT_ACCESS_LOGS), |env_val| env_val.parse::<bool>())?;
        let http_public_directory =
            std::env::var(ENV_HTTP_PUBLIC_DIRECTORY).unwrap_or(String::from(DEFAULT_HTTP_PUBLIC_DIRECTORY));

        let http = Http {
            port: http_port,
            access_logs: http_access_logs,
            public_directory: http_public_directory,
        };

        // database
        let database_url = std::env::var(ENV_DATABASE_URL).map_err(|_| env_not_found(ENV_DATABASE_URL))?;
        let database_pool_size = std::env::var(ENV_DATABASE_POOL_SIZE)
            .ok()
            .map_or(Ok(DEFAULT_DATABASE_POOL_SIZE), |pool_size_str| {
                pool_size_str.parse::<u32>()
            })?;

        let database = Database {
            url: database_url,
            pool_size: database_pool_size,
        };

        // smtp
        let smtp_port = std::env::var(ENV_SMTP_PORT)
            .ok()
            .map_or(Ok(None), |smtp_port_str| smtp_port_str.parse::<u16>().map(Some))?;
        let smtp_host = std::env::var(ENV_SMTP_HOST).ok();
        let smtp_username = std::env::var(ENV_SMTP_USERNAME).ok();
        let smtp_password = std::env::var(ENV_SMTP_PASSWORD).ok();

        let smtp = Smtp {
            port: smtp_port,
            host: smtp_host,
            username: smtp_username,
            password: smtp_password,
        };

        // mail
        let mail_driver = std::env::var(ENV_MAIL_DRIVER)
            .ok()
            .map_or(Ok(DEFAULT_MAIL_DRIVER), |env_val| env_val.parse::<MailDriver>())?;
        let mail_notify_address = std::env::var(ENV_MAIL_NOTIFY_ADDRESS)
            .map_err(|_| env_not_found(ENV_MAIL_NOTIFY_ADDRESS))?
            .parse::<mail::Address>()?;
        let mail_newsletter_address = std::env::var(ENV_MAIL_NEWSLETTER_ADDRESS)
            .map_err(|_| env_not_found(ENV_MAIL_NEWSLETTER_ADDRESS))?
            .parse::<mail::Address>()?;
        let mail_domains_blocklist_file =
            std::env::var(ENV_MAIL_BLOCKLIST).unwrap_or(String::from(DEFAULT_MAIL_BLOCKLIST_FILE));

        let mail_domains_blocklist: HashSet<String> = {
            let blocklist_file = OpenOptions::new()
                .read(true)
                .open(&mail_domains_blocklist_file)
                .map_err(|err| Error::Internal(format!("config: Error reading email blocklist: {}", err)))?;
            let reader = BufReader::new(&blocklist_file);
            let mut blocklist = HashSet::new();
            for line in reader.lines() {
                let line = line?.trim().to_string();
                blocklist.insert(line);
            }
            blocklist
        };

        let mail = Mail {
            driver: mail_driver,
            notify_address: mail_notify_address,
            newsletter_address: mail_newsletter_address,
            domains_blocklist_file: mail_domains_blocklist_file,
            domains_blocklist: mail_domains_blocklist,
        };

        // storage
        let storage_driver = std::env::var(ENV_STORAGE_DRIVER)
            .ok()
            .map_or(Ok(DEFAULT_STORAGE_DRIVER), |env_val| env_val.parse::<StorageDriver>())?;
        let storage_base_directory =
            std::env::var(ENV_STORAGE_BASE_DIRECTORY).unwrap_or(String::from(DEFAULT_STORAGE_BASE_DIRECTORY));

        let storage = Storage {
            driver: storage_driver,
            base_directory: storage_base_directory,
        };

        // Stripe
        let stripe = if self_hosted {
            None
        } else {
            let stripe_private_key =
                std::env::var(ENV_STRIPE_SECRET_KEY).map_err(|_| env_not_found(ENV_STRIPE_SECRET_KEY))?;
            let stripe_public_key =
                std::env::var(ENV_STRIPE_PUBLIC_KEY).map_err(|_| env_not_found(ENV_STRIPE_PUBLIC_KEY))?;
            let stripe_webhook_secret =
                std::env::var(ENV_STRIPE_WEBHOOK_SECRET).map_err(|_| env_not_found(ENV_STRIPE_WEBHOOK_SECRET))?;
            let stripe_data_json = std::env::var(ENV_STRIPE_DATA).map_err(|_| env_not_found(ENV_STRIPE_DATA))?;
            let mut stripe_data: StripeData = serde_json::from_str(&stripe_data_json)?;
            stripe_data.taxes = stripe_data
                .taxes
                .into_iter()
                .map(|tax| (tax.0.trim().to_string(), tax.1.trim().to_string()))
                .collect();

            Some(Stripe {
                secret_key: stripe_private_key,
                public_key: stripe_public_key,
                webhook_secret: stripe_webhook_secret,
                data: stripe_data,
                json_data: stripe_data_json,
            })
        };

        // aws
        let aws_secret_access_key = std::env::var(ENV_AWS_SECRET_ACCESS_KEY).ok();
        let aws_access_key_id = std::env::var(ENV_AWS_ACCESS_KEY_ID).ok();
        let aws_default_region = std::env::var(ENV_AWS_DEFAULT_REGION).unwrap_or(String::from(DEFAULT_AWS_REGION));
        let aws_default_region_rusoto = Region::from_str(&aws_default_region)?;

        let aws = Aws {
            secret_access_key: aws_secret_access_key,
            access_key_id: aws_access_key_id,
            default_region: aws_default_region,
            default_region_rusoto: aws_default_region_rusoto,
        };

        // ses
        let ses_region = std::env::var(ENV_SES_REGION).unwrap_or(aws.default_region.clone());
        let ses_region_rusoto = Region::from_str(&ses_region)?;

        let ses = Ses {
            region: ses_region,
            region_rusoto: ses_region_rusoto,
        };

        // s3
        let s3_region = std::env::var(ENV_S3_REGION).unwrap_or(aws.default_region.clone());
        let s3_bucket = std::env::var(ENV_S3_BUCKET).map_err(|_| env_not_found(ENV_S3_BUCKET))?;
        let s3_region_rusoto = Region::from_str(&s3_region)?;

        let s3 = S3 {
            region: s3_region,
            bucket: s3_bucket,
            region_rusoto: s3_region_rusoto,
        };

        // worker
        let worker_concurrency = std::env::var(ENV_WORKER_CONCURRENCY)
            .ok()
            .map_or(Ok(DEFAULT_WORKER_CONCURRENCY), |env_val| env_val.parse::<usize>())?;

        let worker = Worker {
            concurrency: worker_concurrency,
        };

        // sentry
        let sentry_security_report_uri = std::env::var(ENV_SENTRY_SECURITY_REPORT_URI).ok();
        let sentry_ingest_domain = std::env::var(ENV_SENTRY_INGEST_DOMAIN).ok();
        let sentry_dsn = std::env::var(ENV_SENTRY_DSN).ok();

        let sentry = Sentry {
            security_report_uri: sentry_security_report_uri,
            ingest_domain: sentry_ingest_domain,
            dsn: sentry_dsn,
        };

        let mut config = Config {
            env,
            base_url,
            master_key,
            old_master_key,
            self_hosted,
            countries,
            http,
            database,
            smtp,
            mail,
            storage,
            stripe,
            aws,
            ses,
            s3,
            worker,
            sentry,
        };

        config.clean_and_validate()?;

        Ok(config)
    }

    fn clean_and_validate(&mut self) -> Result<(), Error> {
        // app
        if self.master_key.len() != crypto::AEAD_KEY_SIZE {
            return Err(Error::InvalidArgument(format!(
                "config: master_key is not valid. Required size is: {} bytes",
                crypto::AEAD_KEY_SIZE
            )));
        }

        if let Some(ref old_master_key) = self.old_master_key {
            if old_master_key.len() != crypto::AEAD_KEY_SIZE {
                return Err(Error::InvalidArgument(format!(
                    "config: old_master_key is not valid. Required size is: {} bytes",
                    crypto::AEAD_KEY_SIZE
                )));
            }
        }

        // Database
        let database_url = Url::parse(&self.database.url)?;
        if database_url.scheme() != POSTGRES_SCHEME {
            return Err(Error::InvalidArgument(String::from(
                "config: database_url is not a valid postgres URL",
            )));
        }
        // force ssl if not explicitely disabled
        // databaseURLQuery := databaseURL.Query()
        // if len(databaseURLQuery["sslmode"]) == 0 {
        //     databaseURLQuery.Set("sslmode", "require")
        // }
        // databaseURL.RawQuery = databaseURLQuery.Encode()
        // config.Database.URL = databaseURL.String()

        // Stripe
        if !self.self_hosted {
            let stripe = self.stripe.as_ref().unwrap();
            if !stripe.public_key.starts_with(STRIPE_PUBLIC_KEY_PREFIX) {
                return Err(Error::InvalidArgument(String::from(
                    "config: STRIPE_PUBLIC_KEY is not valid",
                )));
            }

            if !stripe.secret_key.starts_with(STRIPE_SECRET_KEY_PREFIX) {
                return Err(Error::InvalidArgument(String::from(
                    "config: STRIPE_SECRET_KEY is not valid",
                )));
            }

            if !stripe.data.prices.starter.starts_with(STRIPE_PRICE_PREFIX)
                || !stripe.data.prices.pro.starts_with(STRIPE_PRICE_PREFIX)
            // || !self.stripe.data.prices.ultra.starts_with(STRIPE_PRICE_PREFIX)
            {
                return Err(Error::InvalidArgument(String::from("config: invalid price")));
            }

            if !stripe.data.products.starter.starts_with(STRIPE_PRODUCT_PREFIX)
                || !stripe.data.products.pro.starts_with(STRIPE_PRODUCT_PREFIX)
            // || !self.stripe.data.products.ultra.starts_with(STRIPE_PRODUCT_PREFIX)
            {
                return Err(Error::InvalidArgument(String::from("config: invalid product")));
            }

            if stripe.data.taxes.len() != vat::RATES_NUMBER {
                return Err(Error::InvalidArgument(String::from(
                    "config: invalid number of stripe taxes",
                )));
            }

            for tax in &stripe.data.taxes {
                if !tax.1.starts_with(STRIPE_TAX_PREFIX) {
                    return Err(Error::InvalidArgument(String::from("config: stripe tax not valid")));
                }

                if !self.countries.contains_key(tax.0) {
                    return Err(Error::InvalidArgument(format!(
                        "config: country code not found for stripe tax: {}",
                        tax.0
                    )));
                }
            }
        }

        Ok(())
    }
}

fn env_not_found(var: &str) -> Error {
    Error::NotFound(format!("config: {} env var not found", var))
}

#[cfg(test)]
pub mod test {
    use super::Config;
    use std::env;

    pub fn load_test_config() -> Config {
        // by default cargo run the tests in the directory of the package's manifest
        // in ou case, we need to run the tests in the directory of the workspace, in
        // order to load assets
        let current_dir = env::current_dir().unwrap();
        env::set_current_dir(current_dir.join("..")).unwrap();
        Config::load().unwrap()
    }
}
