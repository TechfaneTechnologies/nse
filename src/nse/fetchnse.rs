use chrono::Datelike;
#[allow(dead_code)]
use chrono::{NaiveDate, NaiveDateTime};
// use csv::ReaderBuilder;
// use csv::Trim;
use isahc::{
    config::{DnsCache, RedirectPolicy, VersionNegotiation},
    cookies::{Cookie, CookieJar},
    prelude::*,
    HttpClient, // Request,
};
use rayon::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize};

// use super::cycle_n::{cycle_n, cycle_n_manual, cycle_n_trait};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::{fmt::Display, thread};
// use std::iter::Cycle;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct OhlcvRecord {
    #[serde(deserialize_with = "naive_date_time_from_str")]
    date: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_from_str")]
    open: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    high: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    low: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    close: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    volume: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    cumulative_volume: f64,
}

// You can use this deserializer for any type that implements FromStr
// and the FromStr::Err implements Display
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,      // Required for S::from_str...
    S::Err: Display, // Required for .map_err(de::Error::custom)
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%f").map_err(de::Error::custom)
    NaiveDateTime::parse_from_str(&s, "%d-%m-%Y %H:%M").map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkData {
    #[serde(rename = "Instrument")]
    instrument: &'static str,
    #[serde(rename = "CDSymbol")]
    cdsymbol: String,
    #[serde(rename = "Segment")]
    segment: &'static str,
    #[serde(rename = "Series")]
    series: &'static str,
    #[serde(rename = "CDExpiryMonth")]
    cdexpirymonth: u8,
    #[serde(rename = "FOExpiryMonth")]
    foexpirymonth: u8,
    #[serde(rename = "IRFExpiryMonth")]
    irfexpirymonth: NaiveDate,
    #[serde(rename = "CDIntraExpiryMonth")]
    cdintraexpirymonth: NaiveDate,
    #[serde(rename = "FOIntraExpiryMonth")]
    fointraexpirymonth: NaiveDate,
    #[serde(rename = "IRFIntraExpiryMonth")]
    irfintraexpirymonth: &'static str,
    #[serde(rename = "CDDate1")]
    cddate1: String,
    #[serde(rename = "CDDate2")]
    cddate2: String,
    #[serde(rename = "PeriodType")]
    periodtype: u8,
    #[serde(rename = "Periodicity")]
    periodicity: u8,
    ct0: &'static str,
    ct1: &'static str,
    ctcount: u8,
    time: usize,
}

pub struct FetchNse {
    pub client: HttpClient,
    pub today_folder_name: String,
    pub home_folder_path: &'static Path,
    pub nse_url: &'static str,
    pub nse_cookies: CookieJar,
    pub backoff_time: [Duration; 11],
}

const NSE_URL: &'static str = "https://www.nseindia.com";

impl Default for FetchNse {
    fn default() -> Self {
        Self {
            client: HttpClient::builder()
                    .default_header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.100.1185.50 Safari/537.36 Edg/99.100.1185.50")
                    .max_connections(10)
                    .max_connections_per_host(10)
                    .connection_cache_size(10)
                    .tcp_keepalive(Duration::from_secs(10))
                    .dns_cache(DnsCache::Forever)
                    //.dns_resolve(ResolveMap::new()
                    // Send requests for example.org on port 80 to 127.0.0.1.
                    //.add("www.example.org", 8080, [127, 0, 0, 1]))
                    .version_negotiation(VersionNegotiation::http2())
                    .cookies()
                    .redirect_policy(RedirectPolicy::Follow)
                    .build()
                    .unwrap(),
            today_folder_name: chrono::Local::now().format("%d-%b-%Y").to_string(),
            home_folder_path: Path::new("NSE_Downloads_Data"),
            nse_url: NSE_URL,
            nse_cookies: CookieJar::new(),
            backoff_time: [Duration::from_secs_f64(rand::thread_rng().gen_range(1.229..=1.363)), Duration::from_secs_f64(rand::thread_rng().gen_range(1.541..=1.701)), Duration::from_secs_f64(rand::thread_rng().gen_range(1.731..=1.979)), Duration::from_secs_f64(rand::thread_rng().gen_range(1.143..=1.147)), Duration::from_secs_f64(rand::thread_rng().gen_range(1.571..=1.991)), Duration::from_secs_f64(rand::thread_rng().gen_range(2.011..=3.047)), Duration::from_secs_f64(rand::thread_rng().gen_range(4.051..=10.011)), Duration::from_secs_f64(rand::thread_rng().gen_range(11.111..=27.737)), Duration::from_secs_f64(rand::thread_rng().gen_range(29.123..=33.357)), Duration::from_secs_f64(rand::thread_rng().gen_range(33.979..=57.791)), Duration::from_secs_f64(rand::thread_rng().gen_range(58.853..=59.999))],
        }
    }
}

impl FetchNse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_dir_if_not_exists(&self, dir: &str) {
        if !Path::new(dir).exists() {
            std::fs::create_dir_all(dir).unwrap();
        }
    }

    pub fn check_or_create_expiry_directories(&self) {
        let expiry_dirs = ["Nifty", "BankNifty", "FinNifty", "Misc"];
        let ohlcv_dirs = ["Index", "Equity", "Futures", "Currency", "Commodity"];
        let custom_folders = [
            "GraphsData",
            "Options_Intraday_Snapshots",
            "Futures_Intraday_Snapshots",
        ];
        if !self.home_folder_path.exists() && !self.home_folder_path.is_dir() {
            // self.create_dir_if_not_exists(&self.home_folder_path.to_str().unwrap());
            self.create_dir_if_not_exists(
                &self
                    .home_folder_path
                    .join("1-Minute-OHLCV-Data")
                    .to_str()
                    .unwrap(),
            );
        }
        for dir in &ohlcv_dirs {
            self.create_dir_if_not_exists(
                &self
                    .home_folder_path
                    .join("1-Minute-OHLCV-Data")
                    .join(dir)
                    .join(&self.today_folder_name)
                    .to_str()
                    .unwrap(),
            );
        }
        for dir in &expiry_dirs {
            for folders in &custom_folders {
                self.create_dir_if_not_exists(
                    &self
                        .home_folder_path
                        .join(dir)
                        .join(folders)
                        .join(&self.today_folder_name)
                        .to_str()
                        .unwrap(),
                );
            }
        }
    }

    // pub fn get_expiry_dates(&self) -> Vec<NaiveDate> {
    //     let mut expiry_dates = Vec::new();
    //     let mut expiry_date =
    //         NaiveDate::from_ymd(chrono::Local::now().year(), chrono::Local::now().month(), 1);
    //     while expiry_date <= chrono::Local::now().NaiveDate() {
    //         expiry_dates.push(expiry_date);
    //         expiry_date = expiry_date.succ();
    //     }
    //     expiry_dates
    // }

    fn fetch_nse_home_page(&self) -> Result<(), isahc::Error> {
        let response = self.client.get(self.nse_url)?;
        println!(
            "╭─ Fetched \n╰─{}: With Status {}",
            self.nse_url,
            response.status(),
        );
        Ok(())
    }

    pub fn form_route_url(&self, expiry_date: NaiveDate) -> String {
        let mut route_url = String::from(self.nse_url);
        route_url.push_str("/products/dynaContent/common/productsSymbolMapping.jsp?symbol=NIFTY&instrument=OPTIDX&expiryDate=");
        route_url.push_str(&expiry_date.format("%d%m%Y").to_string());
        route_url
    }
    pub fn fetch_nse(&self, uri: &str, print_response_text: bool) -> Result<String, isahc::Error> {
        let mut retry_no: usize = 0;
        let mut response_texts: String = String::new();
        for retries in (0..11).cycle() {
            if retry_no > 0 {
                println!(
                    "Retry No. {} --> Retrying for {} in {} seconds",
                    &retry_no,
                    &uri,
                    self.backoff_time[retry_no - 1].as_secs()
                );
            }
            let start = Instant::now();
            let mut response = self.client.get(uri)?;
            let end = Instant::now();
            // let response_text = response.text()?;
            if response.status().is_success() && retry_no < 26 {
                println!(
                    "╭─ Fetched \n╰─{}: With Status {} in {:?} and {} no of retries",
                    &uri,
                    response.status(),
                    end.duration_since(start),
                    retry_no
                );
                response_texts = response.text()?;
                if print_response_text {
                    println!(
                        "╭─ And the response texts are as follows \n╰─{}",
                        &response_texts
                    );
                }
                return Ok(response_texts);
            } else if response.status().is_server_error() | response.status().is_client_error()
                && retry_no < 26
            {
                retry_no += 1;
                self.client.cookie_jar().unwrap().clear();
                self.fetch_nse_home_page()?;
                thread::sleep(self.backoff_time[retries]);
                continue;
            } else {
                println!(
                    "╭─ Failed \n╰─{}: With Status {} and {} no of retries",
                    &uri,
                    response.status(),
                    retry_no
                );
                break;
            }
        }
        Ok(response_texts)
    }
    pub fn post_nse(
        &self,
        uri: &str,
        data: NetworkData,
        print_response_text: bool,
    ) -> Result<String, isahc::Error> {
        let start = Instant::now();
        let urlencoded_data = serde_urlencoded::to_string(&data).expect("serialize issue");
        let mut response = self.client.post(uri, urlencoded_data)?;
        let end = Instant::now();
        println!(
            "╭─ Fetched {} \n╰─{}: With Status {} in {:?}",
            &data.cdsymbol,
            &uri,
            response.status(),
            end.duration_since(start)
        );
        // println!("{}", response.text()?);
        let response_text = response.text()?;
        if print_response_text {
            println!(
                "╭─ And the response texts are as follows \n╰─{:#?}",
                &response_text
            );
        }
        // std::io::copy(response.body_mut(), &mut std::io::sink())?;
        Ok(response_text)
    }
    pub fn write_file(&self, filename: &Path, content: &[u8]) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(content)?;
        Ok(())
    }
    pub fn write_file_graphsdata(&self, filename: &str, content: &[u8]) -> std::io::Result<()> {
        let files_path = &self
            .home_folder_path
            .join("Nifty")
            .join("GraphsData")
            .join(&self.today_folder_name)
            .join(format!(
                "{}_{}.json",
                filename,
                chrono::Local::now().format("%d-%m-%Y_%H-%M-%S").to_string()
            ));
        // println!("{}", &files_path.to_str().unwrap());
        let mut file = File::create(files_path.to_str().unwrap())?;
        file.write_all(content)?;
        Ok(())
    }

    pub fn fetch_nse_parallel(
        &self,
        urls_vec: Vec<[&str; 2]>,
        print_response_text: bool,
    ) -> Result<(), isahc::Error> {
        self.fetch_nse_home_page()?;
        let start = Instant::now();
        urls_vec
            .par_iter()
            .try_for_each(|[indices, urls]| {
                let response_text = self.fetch_nse(urls, print_response_text)?;
                self.write_file_graphsdata(indices, response_text.as_bytes())?;
                Ok(())
            })
            .map(|_| {
                let end = Instant::now();
                println!(
                    "╭─ Ran a total of \n╰─{} requests in {:?}",
                    urls_vec.len(),
                    end.duration_since(start)
                );
            })
    }
    pub fn get_indices_ohlcv_data_parallel(
        &self,
        indices_vec: Vec<&str>,
    ) -> Result<(), isahc::Error> {
        let start = Instant::now();
        indices_vec
            .par_iter()
            .try_for_each(|indices| {
                let data = NetworkData {
                    instrument: "FUTSTK",
                    cdsymbol: indices.to_string(),
                    segment: "OI",
                    series: "EQ",
                    cdexpirymonth: 1,
                    foexpirymonth: 1,
                    irfexpirymonth: NaiveDate::parse_from_str("31-03-2023", "%d-%m-%Y").unwrap(),
                    cdintraexpirymonth: NaiveDate::parse_from_str("06-05-2022", "%d-%m-%Y")
                        .unwrap(),
                    fointraexpirymonth: NaiveDate::parse_from_str("26-05-2022", "%d-%m-%Y")
                        .unwrap(),
                    irfintraexpirymonth: "",
                    cddate1: chrono::Local::now()
                        .with_year(chrono::Local::now().year() - 3)
                        .unwrap()
                        .format("%d-%m-%Y")
                        .to_string(),
                    cddate2: chrono::Local::now().format("%d-%m-%Y").to_string(),
                    periodtype: 2,
                    periodicity: 1,
                    ct0: "g1|1|1",
                    ct1: "g2|2|1",
                    ctcount: 2,
                    time: chrono::Local::now().naive_local().timestamp_millis() as usize,
                };
                let response_text = self.post_nse(
                    format!(
                        "{}/ChartApp/install/charts/data/GetHistoricalNew.jsp",
                        self.nse_url
                    )
                    .as_str(),
                    data,
                    false,
                )?;
                let response_text = response_text
                    .replace("|", ",")
                    .replace("~", "\n")
                    .replace("date", "Date")
                    .replace("g1_o", "Open")
                    .replace("g1_h", "High")
                    .replace("g1_l", "Low")
                    .replace("g1_c", "Close")
                    .replace("g2", "Volume")
                    .replace("Volume_CUMVOL", "CumulativeVolume");
                // println!("{indices} ========>");
                // println!("{}", response_text);
                let files_path = &self
                    .home_folder_path
                    .join("1-Minute-OHLCV-Data")
                    .join("Index")
                    .join(&self.today_folder_name);
                self.write_file(
                    files_path
                        .as_path()
                        .join(format!("{}.csv", &indices))
                        .as_path(),
                    response_text.as_bytes(),
                )?;
                Ok(())
            })
            .map(|_| {
                let end = Instant::now();
                println!(
                    "╭─ Ran a total of \n╰─{} requests in {:?}",
                    indices_vec.len(),
                    end.duration_since(start)
                );
            })
    }

    pub fn get_indices_ohlcv_data_serial(
        &self,
        indices_vec: Vec<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for indices in indices_vec {
            let data = NetworkData {
                instrument: "FUTSTK",
                cdsymbol: indices.to_owned(),
                segment: "OI",
                series: "EQ",
                cdexpirymonth: 1,
                foexpirymonth: 1,
                irfexpirymonth: NaiveDate::parse_from_str("31-03-2023", "%d-%m-%Y").unwrap(),
                cdintraexpirymonth: NaiveDate::parse_from_str("29-04-2022", "%d-%m-%Y").unwrap(),
                fointraexpirymonth: NaiveDate::parse_from_str("28-04-2022", "%d-%m-%Y").unwrap(),
                irfintraexpirymonth: "",
                cddate1: chrono::Local::now()
                    .with_year(chrono::Local::now().year() - 3)
                    .unwrap()
                    .format("%d-%m-%Y")
                    .to_string(),
                cddate2: chrono::Local::now().format("%d-%m-%Y").to_string(),
                periodtype: 2,
                periodicity: 1,
                ct0: "g1|1|1",
                ct1: "g2|2|1",
                ctcount: 2,
                time: chrono::Local::now().naive_local().timestamp_millis() as usize,
            };
            let response_text = self.post_nse(
                "https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp",
                data,
                false,
            )?;
            let response_text = response_text
                .replace("|", ",")
                .replace("~", "\n")
                .replace("date", "Date")
                .replace("g1_o", "Open")
                .replace("g1_h", "High")
                .replace("g1_l", "Low")
                .replace("g1_c", "Close")
                .replace("g2", "Volume")
                .replace("Volume_CUMVOL", "CumulativeVolume");
            // println!("{indices} ========>");
            // println!("{}", response_text);
            let files_path = &self
                .home_folder_path
                .join("1-Minute-OHLCV-Data")
                .join("Index")
                .join(&self.today_folder_name);
            self.write_file(
                files_path
                    .as_path()
                    .join(format!("{}.csv", &indices))
                    .as_path(),
                response_text.as_bytes(),
            )?;
        }
        Ok(())
    }

    pub fn get_indices_ohlcv_data(
        &self,
        serial: bool,
        parallel: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.check_or_create_expiry_directories();
        let indices_js_data = self
            .fetch_nse(
                "https://www.nseindia.com/ChartApp/install/charts/scripts/indices.js",
                false,
            )
            .unwrap();
        let raw_indices_data = indices_js_data.as_str();
        // println!("{:#?}", raw_indices_data);
        let raw_indices_data = &*raw_indices_data.split("\n").collect::<Vec<&str>>();
        // println!("Raw Indices ==> \n{:#?}", raw_indices_data);
        let indices = raw_indices_data[1]
            .split("var indices\t\t =")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(";")
            .collect::<Vec<&str>>();
        let intra_indices = raw_indices_data
            .last()
            .unwrap()
            .split("var intraIndices =")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(";")
            .collect::<Vec<&str>>();
        // println!("{:#?}", indices);
        // let indices = *indices.first().unwrap();
        let indices = indices
            .first()
            .unwrap()
            .replace("'", "")
            .replace("[", "")
            .replace("]", "");
        let indices = indices.as_str();
        // let intra_indices = *intra_indices.first().unwrap();
        let intra_indices = intra_indices
            .first()
            .unwrap()
            .replace("'", "")
            .replace("[", "")
            .replace("]", "");
        let intra_indices = intra_indices.as_str();
        let indices_vec = indices.split(",").collect::<Vec<&str>>();
        let intra_indices_vec = intra_indices.split(",").collect::<Vec<&str>>();
        println!("{:#?}", &indices_vec);
        println!("{:#?}", &intra_indices_vec);
        if serial {
            self.get_indices_ohlcv_data_serial(indices_vec)?;
        } else if parallel {
            self.get_indices_ohlcv_data_parallel(indices_vec)?;
        }
        Ok(())
    }
}
