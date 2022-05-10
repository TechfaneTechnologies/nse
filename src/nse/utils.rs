use chrono::Datelike;
#[allow(dead_code)]
use chrono::{NaiveDate, NaiveDateTime};
use csv::ReaderBuilder;
use csv::Trim;
use isahc::{
    config::{DnsCache, VersionNegotiation},
    prelude::*,
    HttpClient, Request,
};
use rayon::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize};

use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
struct OhlcvRecord {
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

// #[derive(Debug, Deserialize)]
// struct OhlcvRecord {
//     #[serde(deserialize_with = "naive_date_time_from_str")]
//     date: NaiveDateTime,
//     #[serde(deserialize_with = "deserialize_from_str")]
//     g1_o: f64,
//     #[serde(deserialize_with = "deserialize_from_str")]
//     g1_h: f64,
//     #[serde(deserialize_with = "deserialize_from_str")]
//     g1_l: f64,
//     #[serde(deserialize_with = "deserialize_from_str")]
//     g1_c: f64,
//     #[serde(deserialize_with = "deserialize_from_str")]
//     g2: f64,
//     #[serde(deserialize_with = "deserialize_from_str")]
//     g2_CUMVOL: f64,
// }

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

fn write_file(filename: &str, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.csv", filename))?;
    file.write_all(content)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct NetworkData {
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

pub fn run() -> Result<String, Box<dyn std::error::Error>> {
    // let mut response = isahc::get("https://example.org")?;
    // println!("{}", response.text()?);
    let data = NetworkData {
        instrument: "FUTIDX",
        cdsymbol: "NIFTY BANK".to_owned(),
        segment: "OI",
        series: "EQ",
        cdexpirymonth: 1,
        foexpirymonth: 1,
        irfexpirymonth: NaiveDate::parse_from_str("31-03-2023", "%d-%m-%Y").unwrap(),
        cdintraexpirymonth: NaiveDate::parse_from_str("27-04-2022", "%d-%m-%Y").unwrap(),
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
    let data = serde_urlencoded::to_string(&data).expect("serialize issue");
    println!("{}", data);
    // let mut response =
    //     Request::post("https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp")
    //         .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.100.1185.50 Safari/537.36 Edg/99.100.1185.50")
    //         // .header("content-type", "application/x-www-form-urlencoded")
    //         .timeout(Duration::from_secs(35))
    //         .body(
    //             r#"Instrument=FUTSTK&CDSymbol=NIFTY%2050&Segment=OI&Series=EQ&CDExpiryMonth=1&FOExpiryMonth=1&IRFExpiryMonth=31-03-2023&CDIntraExpiryMonth=27-04-2022&FOIntraExpiryMonth=28-04-2022&IRFIntraExpiryMonth=&CDDate1=25-04-2019&CDDate2=25-04-2022&PeriodType=2&Periodicity=1&ct0=g1|1|1&ct1=g2|2|1&ctcount=2&time=1650906185385"#,
    //         )?
    //         .send()?;
    let mut response =
        Request::post("https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.100.1185.50 Safari/537.36 Edg/99.100.1185.50")
            // .header("content-type", "application/x-www-form-urlencoded")
            .timeout(Duration::from_secs(35))
            .body(
                r#"Instrument=FUTSTK&CDSymbol=NIFTY%2050&Segment=OI&Series=EQ&CDExpiryMonth=1&FOExpiryMonth=1&IRFExpiryMonth=31-03-2023&CDIntraExpiryMonth=27-04-2022&FOIntraExpiryMonth=28-04-2022&IRFIntraExpiryMonth=&CDDate1=25-04-2019&CDDate2=25-04-2022&PeriodType=2&Periodicity=1&ct0=g1|1|1&ct1=g2|2|1&ctcount=2&time=1650906185385"#,
            )?
            .send()?;
    // let mut response =
    //     Request::post("https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp")
    //         .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.100.1185.50 Safari/537.36 Edg/99.100.1185.50")
    //         // .header("content-type", "application/x-www-form-urlencoded")
    //         .timeout(Duration::from_secs(35))
    //         .body(data)?
    //         .send()?;
    println!("{}", response.status());
    // println!("{}", response.text()?);
    let response_text = response.text()?;
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
    println!("{}", response_text);
    write_file("nifty50", response_text.as_bytes())?;
    // std::io::copy(response.body_mut(), &mut std::io::sink())?;
    Ok(response_text)
}

// fn modify_response() -> Result<(), Box<dyn std::error::Error>> {
//     let response_text = run()?;
//     let mut reader = ReaderBuilder::new()
//         .trim(Trim::All)
//         .delimiter(b'|')
//         .has_headers(true)
//         .terminator(Terminator::Any(b'~'))
//         .from_reader(response_text.as_bytes());
//     let mut records = reader.deserialize::<OhlcvRecord>();
//     let mut record = records.next().unwrap()?;
//     println!("{:?}", record);
//     Ok(())
// }

pub fn modify_response() -> Result<(), Box<dyn std::error::Error>> {
    let response_text = run()?;
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(true)
        .from_reader(response_text.as_bytes());
    let mut records = reader.deserialize::<OhlcvRecord>();
    let record = records.next().unwrap()?;
    println!("{:?}", record);
    Ok(())
}

pub fn get_index_js() -> Result<String, Box<dyn std::error::Error>> {
    let mut response =
        Request::get("https://www.nseindia.com/ChartApp/install/charts/scripts/indices.js").header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.100.1185.50 Safari/537.36 Edg/99.100.1185.50")
            .timeout(Duration::from_secs(5))
            .body(())?
            .send()?;

    println!("{}", response.status());
    // println!("{}", response.text()?);
    let response_text = response.text()?;
    // std::io::copy(response.body_mut(), &mut std::io::sink())?;
    Ok(response_text)
}

pub fn get_indices_ohlcv_data_parallel(indices_vec: Vec<&str>) -> Result<(), isahc::Error> {
    // let client = HttpClient::new()?;
    let client = HttpClient::builder()
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
        .build()
        .unwrap();
    let start = Instant::now();
    indices_vec
        .par_iter()
        .try_for_each(move |indices| {
            let start = Instant::now();
            let data = NetworkData {
                instrument: "FUTSTK",
                cdsymbol: indices.to_string(),
                segment: "OI",
                series: "EQ",
                cdexpirymonth: 1,
                foexpirymonth: 1,
                irfexpirymonth: NaiveDate::parse_from_str("31-03-2023", "%d-%m-%Y").unwrap(),
                cdintraexpirymonth: NaiveDate::parse_from_str("06-05-2022", "%d-%m-%Y").unwrap(),
                fointraexpirymonth: NaiveDate::parse_from_str("26-05-2022", "%d-%m-%Y").unwrap(),
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
            let data = serde_urlencoded::to_string(&data).expect("serialize issue");
            // println!("{}", data);
            let mut response = client.post(
                "https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp",
                data,
            )?;
            // println!("{}", response.status());
            // println!("{}", response.text()?);
            let response_text = response.text()?;
            let end = Instant::now();
            println!(
                "{}: {} in {:?}",
                &indices,
                response.status(),
                end.duration_since(start)
            );
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
            write_file(indices, response_text.as_bytes())?;
            // std::io::copy(response.body_mut(), &mut std::io::sink())?;
            Ok(())
        })
        .map(|_| {
            let end = Instant::now();
            println!(
                "Ran {} requests in {:?}",
                indices_vec.len(),
                end.duration_since(start)
            );
        })
}

pub fn get_indices_ohlcv_data_serial(
    indices_vec: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::builder()
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
        .build()
        .unwrap();
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
        let data = serde_urlencoded::to_string(&data).expect("serialize issue");
        // println!("{}", data);
        // let mut response =
        // Request::post("https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp")
        //     .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.100.1185.50 Safari/537.36 Edg/99.100.1185.50")
        //     // .header("content-type", "application/x-www-form-urlencoded")
        //     .timeout(Duration::from_secs(35))
        //     .body(data)?
        //     .send()?;
        let mut response = client.post(
            "https://www.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp",
            data,
        )?;
        // println!("{}", response.status());
        // println!("{}", response.text()?);
        let response_text = response.text()?;
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
        println!("{}: {}", &indices, response.status());
        write_file(indices, response_text.as_bytes())?;
        // std::io::copy(response.body_mut(), &mut std::io::sink())?;
    }
    Ok(())
}

pub fn get_indices_ohlcv_data() -> Result<(), Box<dyn std::error::Error>> {
    let indices_js_data = get_index_js().unwrap();
    let raw_indices_data = indices_js_data.as_str();
    println!("{}", raw_indices_data);
    let raw_indices_data = &*raw_indices_data.split("\n").collect::<Vec<&str>>();
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
    println!("{:#?}", indices_vec);
    println!("{:#?}", intra_indices_vec);
    get_indices_ohlcv_data_parallel(indices_vec)?;
    // get_indices_ohlcv_data_serial(indices_vec)?;
    // get_indices_ohlcv_data(intra_indices_vec)?;
    Ok(())
}

// struct NaiveDateTimeVisitor;

// impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
//     type Value = NaiveDateTime;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "a string represents chrono::NaiveDateTime")
//     }

//     fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.%f") {
//             Ok(t) => Ok(t),
//             Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
//         }
//     }
// }

// fn from_timestamp<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
// where
//     D: de::Deserializer<'de>,
// {
//     d.deserialize_str(NaiveDateTimeVisitor)
// }

// #[derive(Deserialize, Debug)]
// struct MyJson {
//     name: String,
//     #[serde(deserialize_with = "from_timestamp")]
//     timestamp: NaiveDateTime,
// }

// fn main() {
//     let result: MyJson =
//         serde_json::from_str(r#"{"name": "asdf", "timestamp": "2019-08-15T17:41:18.106108"}"#)
//             .unwrap();
//     println!("{:?}", result);
// }

// use ::chrono::{DateTime, Utc};
// use serde_with::formats::Flexible;
// use serde_with::TimestampMilliSeconds;

// #[serde_with::serde_as]
// #[derive(serde::Deserialize, serde::Serialize)]
// struct S {
//     #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
//     time: DateTime<Utc>,
// }

// fn main() {
//     serde_json::from_str::<S>(r#"{"time":1526522699918}"#).unwrap(); // millisecond timestamp as a integer
//     serde_json::from_str::<S>(r#"{"time":"1526522699918"}"#).unwrap(); // millisecond timestamp as an string
// }
