#[allow(dead_code)]
mod nse;
// use nse::utils::get_indices_ohlcv_data;

use nse::fetchnse::FetchNse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // run().unwrap();
    // modify_response().unwrap();
    // modify_response()?;
    // get_indices_ohlcv_data()?;
    let nse_client = FetchNse::new();
    // println!("{}",nse_client.nse_url);
    // nse_client.check_or_create_expiry_directories();
    nse_client.get_indices_ohlcv_data(false, true)?;
    // nse_client.get_indices_ohlcv_data(true, false)?;
    // nse_client.fetch_nse("https://www.nseindia.com/api/getNifty50VsindiaVix", true)?;
    let urls = [["NIFTY50_VS_INDIAVIX","https://www.nseindia.com/api/getNifty50VsindiaVix"],
                                ["NIFTY_INDEX_PreMarket","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true&preopen=true"],
                                ["NIFTY_INDEX","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true"],
                                ["NIFTY_NEXT50_INDEX_PreMarket","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20NEXT%2050&indices=true&preopen=true"],
                                ["NIFTY_NEXT50_INDEX","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20NEXT%2050&indices=true"],
                                ["NIFTY_MIDCAP50_INDEX_PreMarket","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20MIDCAP%2050&indices=true&preopen=true"],
                                ["NIFTY_MIDCAP50_INDEX","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20MIDCAP%2050&indices=true"],
                                ["BANKNIFTY_INDEX_PreMarket","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20BANK&indices=true&preopen=true"],
                                ["BANKNIFTY_INDEX","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20BANK&indices=true"],
                                ["FINNIFTY_INDEX_PreMarket","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20FINANCIAL%20SERVICES&indices=true&preopen=true"],
                                ["FINNIFTY_INDEX","https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20FINANCIAL%20SERVICES&indices=true"]];
    nse_client.fetch_nse_parallel(urls.to_vec(), false)?;
    Ok(())
}
