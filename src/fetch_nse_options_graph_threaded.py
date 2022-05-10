#!/home/n0ak95/anaconda3/envs/mine/bin/python

import requests, shortuuid, base64, aiohttp, asyncio, uvloop, time, dill, pickle, os, pdb, sys #, json
from urllib.parse import urlencode
import pandas as pd
from numpy import random
from time import sleep
from datetime import datetime as dt
import datetime as dtt
import cysimdjson
from pprint import pprint
# from aiohttpratelimiter import RateLimiter
import concurrent.futures
from itertools import cycle
from itertools import islice
import logging
import threading
from concurrent.futures import ThreadPoolExecutor
from requests_futures.sessions import FuturesSession
import hyperjson as json
from signal import signal, SIGINT
from sys import exit
logging.basicConfig(level=logging.DEBUG)
logging.getLogger().setLevel(logging.DEBUG)
asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())
# =========================================================================================================================== #
class nsefetch:
    __NSE = "https://www.nseindia.com"
    __request_headers = {
                        'Host':'www.nseindia.com',
                        'User-Agent':'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:82.0) Gecko/20100101 Firefox/82.0',
                        'Accept':'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
                        'Accept-Language':'en-US,en;q=0.5',
                        'Accept-Encoding':'gzip, deflate, br',
                        'DNT':'1',
                        'Connection':'keep-alive',
                        'Upgrade-Insecure-Requests':'1',
                        'Pragma':'no-cache',
                        'Cache-Control':'no-cache',
                        }

    # Fetch Proxies from http://www.freeproxylists.net/?c=IN&s=u
    __proxies = {
                'http': 'http://116.202.232.246:80',
                'https': 'http://116.202.228.164:3128',
                'https': 'http://116.202.228.162:3128',
                'https': 'http://210.212.128.246:8080',
                'https': 'http://125.99.58.110:3128',
                'https': 'http://182.48.240.2:80',
                'https': 'http://1.186.34.45:80',
                'https': 'http://123.108.201.18:84',
                'https': 'http://1.186.83.4:80',
                'https': 'http://175.101.12.10:84',
                'https': 'http://175.101.14.34:83',
                'https': 'http://27.116.51.119:8080',
                'https': 'http://115.241.225.42:80',
                'https': 'http://103.246.225.34:80',
                'https': 'http://182.71.146.148:8080',
                'https': 'http://117.239.240.202:53281'
                }

    __back_off = (random.uniform(0.229, 0.363), random.uniform(0.541, 0.701), random.uniform(0.731, 0.979), random.uniform(1.143, 1.147), \
                    random.uniform(1.571, 1.991), random.uniform(2.011, 3.047), random.uniform(4.051, 10.011), random.uniform(11.111, 27.737), \
                    random.uniform(29.123, 33.357), random.uniform(33.979, 57.791), random.uniform(58.853, 59.999))

    __today_folder_name = dt.today().date().strftime("%d_%b_%Y")
    __today_folder_path = f'./{dt.today().date().strftime("%d_%b_%Y")}/'
    __max_workers = 32
    __mid_workers = 16
    __avg_workers = 8
    __min_workers = 4
    __thread_local = threading.local()
    __sbull_zip_file = f'../5.Sensibull_Data_Scraping/Zipped_Files/Nifty_Banknifty_Options_Data_With_OI_and_Greeks_10_Seconds_Interval_{dt.today().date().strftime("%d_%b_%Y")}.tar.zst'

    def __init__(self, proxy=None):
        # self.__s = requests.Session()
        self.__s = self.__get_session()
        if proxy:
            self.__s.proxies.update(self.__proxies)
        self.__check_or_create_expiry_directories()
        self.__fetch_nse()

    def __exit_handler(self, signal_received, frame):
        # Handle any cleanup here
        res = input("Ctrl-c was pressed. Do you really want to exit? y/n >>> ")
        if res == 'y':
            print('SIGINT or CTRL-C detected. Exiting gracefully', end="", flush=True)
            exit(0)
        else:
            print("", end="\r", flush=True)
            print(" " * len(msg), end="", flush=True) # clear the printed line
            print("    ", end="\r", flush=True)

    def __get_session(self):
        if not hasattr(self.__thread_local, "session"):
            # self.__thread_local.session = requests.Session()
            self.__thread_local.session = FuturesSession(executor=ThreadPoolExecutor(max_workers=self.__max_workers))
        return self.__thread_local.session

    def __check_or_create_expiry_directories(self):
        ticker_list = ('NIFTY', 'BANKNIFTY', 'FINNIFTY', 'MISC')
        if not (os.path.exists(self.__today_folder_path) and os.path.isdir(self.__today_folder_path)) and not os.path.isfile(f"./Zipped_Files/{self.__today_folder_name}.tar.zst"):
            os.system(f'mkdir -p ./{self.__today_folder_name}')
            for ticker in ticker_list:
                os.system(f'mkdir -p ./{self.__today_folder_name}/{ticker}/GraphsData')
                os.system(f'mkdir -p ./{self.__today_folder_name}/{ticker}/Options_Intraday_Snapshots')
                os.system(f'mkdir -p ./{self.__today_folder_name}/{ticker}/Futures_Intraday_Snapshots')
            print("NBN_Expiries Directory And Subdirectories Has Been Created Successfully")
        else:
            print("NBN_Expiries Directory Exists")

    def __save_symbol_file(self, symbol, url, Options_Intraday_Snapshots=None, Futures_Intraday_Snapshots=None, GraphsData=None):
        if Options_Intraday_Snapshots:
            if symbol[0:4] == 'NIFT':
                file_path = f'./{self.__today_folder_name}/NIFTY/Options_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            elif symbol[0:4] == 'BANK':
                file_path = f'./{self.__today_folder_name}/BANKNIFTY/Options_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            elif symbol[0:4] == 'FINN':
                file_path = f'./{self.__today_folder_name}/FINNIFTY/Options_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            else:
                file_path = f'./{self.__today_folder_name}/MISC/Options_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
        if Futures_Intraday_Snapshots:
            if symbol[0:4] == 'NIFT':
                file_path = f'./{self.__today_folder_name}/NIFTY/Futures_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            elif symbol[0:4] == 'BANK':
                file_path = f'./{self.__today_folder_name}/BANKNIFTY/Futures_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            elif symbol[0:4] == 'FINN':
                file_path = f'./{self.__today_folder_name}/FINNIFTY/Futures_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            else:
                file_path = f'./{self.__today_folder_name}/MISC/Futures_Intraday_Snapshots/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
        if GraphsData:
            if symbol[0:4] == 'NIFT':
                file_path = f'./{self.__today_folder_name}/NIFTY/GraphsData/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            elif symbol[0:4] == 'BANK':
                file_path = f'./{self.__today_folder_name}/BANKNIFTY/GraphsData/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            elif symbol[0:4] == 'FINN':
                file_path = f'./{self.__today_folder_name}/FINNIFTY/GraphsData/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
            else:
                file_path = f'./{self.__today_folder_name}/MISC/GraphsData/{symbol}_{dt.now().strftime("%Y-%b-%d_%H:%M:%S")}.json'
        with open(file_path, 'w') as outfile:
            json.dump(self.fetch(url), outfile, indent=4)

    def __fetch_nse(self):
        retry_no: int = 0
        for no_of_retries in cycle(range(0, 11)):
            if retry_no != 0: print(f'>!>!> Retrying For > {self.__NSE} > Retry No > {retry_no} <!<!<')
            try:
                self.__s.cookies.clear()
                output = self.__s.get(self.__NSE, headers=self.__request_headers, timeout=15)
                if output.result().status_code != 200:
                    output.result().raise_for_status()
                else:
                    return None
            except requests.exceptions.Timeout:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                continue
            except requests.exceptions.TooManyRedirects:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                continue
            except requests.exceptions.RequestException as e:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                continue
            except requests.exceptions.HTTPError as err:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                continue

    def fetch(self, url: str):
        retry_no: int = 0
        for no_of_retries in cycle(range(0, 11)):
            if retry_no != 0: print(f'>!>!> Retrying For > {url} > Retry No > {retry_no} <!<!<')
            try:
                output = self.__s.get(url, headers=self.__request_headers, timeout=30)
                if output.result().status_code != 200:
                    output.result().raise_for_status()
                else:
                    return output.result().json()
            except requests.exceptions.Timeout:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                self.__fetch_nse()
                continue
            except requests.exceptions.TooManyRedirects:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                self.__fetch_nse()
                continue
            except requests.exceptions.RequestException as e:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                self.__fetch_nse()
                continue
            except requests.exceptions.HTTPError as err:
                retry_no += 1
                print(f'>!>!> Retries Left > {11-no_of_retries} <!<!<')
                sleep(self.__back_off[no_of_retries])
                self.__fetch_nse()
                continue

    def fetch_nse_options_json(self, symbols=None, symbols_option=None):
        if (not symbols and not symbols_option):
            symbols = ['NIFTY', 'BANKNIFTY', 'FINNIFTY']
            symbols_option = {'NIFTY': 'nse50_opt', 'BANKNIFTY': 'nifty_bank_opt', 'FINNIFTY': 'finnifty_opt'}
        symbol_option_chain_urls = {f'{symbol}_Option_Chain':f'https://www.nseindia.com/api/option-chain-indices?symbol={symbol}' for symbol in symbols}
        symbol_options_data_urls = {f'{symbol}_Options_Data':f'https://www.nseindia.com/api/liveEquity-derivatives?index={ticker}' for symbol, ticker in symbols_option.items()}
        symbol_options_chains = symbol_option_chain_urls | symbol_options_data_urls
        futures = []
        with concurrent.futures.ThreadPoolExecutor(max_workers=self.__avg_workers) as executor:
            for symbol, symbol_url in symbol_options_chains.items():
                futures.append(executor.submit(self.__save_symbol_file, symbol.upper(), symbol_url, Options_Intraday_Snapshots=True))
        concurrent.futures.wait(futures)

    def fetch_nse_futures_json(self, symbols=None, symbols_future=None):
        if not symbols:
            symbols = ['NIFTY', 'BANKNIFTY', 'FINNIFTY']
            symbols_future = {'NIFTY': 'nse50_fut', 'BANKNIFTY': 'nifty_bank_fut', 'FINNIFTY': 'finnifty_fut'}
        symbol_futures_urls = {f'{symbol}_Futures_Data':f'https://www.nseindia.com/api/liveEquity-derivatives?index={ticker}' for symbol, ticker in symbols_future.items()}
        futures = []
        with concurrent.futures.ThreadPoolExecutor(max_workers=self.__avg_workers) as executor:
            for symbol, symbol_url in symbol_futures_urls.items():
                futures.append(executor.submit(self.__save_symbol_file, symbol.upper(), symbol_url, Futures_Intraday_Snapshots=True))
        concurrent.futures.wait(futures)

    def fetch_nse_options_graphs_json(self, symbols=None):
        if not symbols:
            symbols = ['NIFTY', 'BANKNIFTY', 'FINNIFTY']
        urls = {symbol:f'https://www.nseindia.com/api/option-chain-indices?symbol={symbol}' for symbol in symbols}
        symbol_json_data = { symbol:self.fetch(url) for symbol, url in urls.items() }
        symbol_strike_list_current_expiry = { symbol:[[[f'{item["CE"]["identifier"][6:-3]}', f'https://www.nseindia.com/api/chart-databyindex?index={item["CE"]["identifier"]}'], \
                                            [f'{item["PE"]["identifier"][6:-3]}', f'https://www.nseindia.com/api/chart-databyindex?index={item["PE"]["identifier"]}']] \
                                            for item in json_data['filtered']['data'] if ("CE" in item and "PE" in item)] for symbol, json_data in symbol_json_data.items() }

        symbol_strike_list_current_expiry = { symbol:{item[0]:item[1] for items in strike_n_url for item in items} for symbol, strike_n_url in symbol_strike_list_current_expiry.items() }
        futures = []
        with concurrent.futures.ThreadPoolExecutor(max_workers=self.__max_workers) as executor:
            for strike_list_urls in symbol_strike_list_current_expiry.values():
                for strike, strike_url in strike_list_urls.items():
                    futures.append(executor.submit(self.__save_symbol_file, strike.upper(), strike_url, GraphsData=True))
        concurrent.futures.wait(futures)

    def fetch_nse_all_options_graphs_json(self, symbols=None, returns=None, pprints=None):
        if not symbols:
            symbols = ['NIFTY', 'BANKNIFTY', 'FINNIFTY']
        urls = {symbol:f'https://www.nseindia.com/api/option-chain-indices?symbol={symbol}' for symbol in symbols}
        symbol_json_data = { symbol:self.fetch(url) for symbol, url in urls.items() }
        symbol_strike_list_all_expiry = { symbol:[[[f'{item["CE"]["identifier"][6:-3]}', f'https://www.nseindia.com/api/chart-databyindex?index={item["CE"]["identifier"]}'], \
                                        [f'{item["PE"]["identifier"][6:-3]}', f'https://www.nseindia.com/api/chart-databyindex?index={item["PE"]["identifier"]}']] \
                                        for item in json_data['records']['data'] if ("CE" in item and "PE" in item)] for symbol, json_data in symbol_json_data.items() }

        symbol_strike_list_all_expiry = { symbol:{item[0]:item[1] for items in strike_n_url for item in items} for symbol, strike_n_url in symbol_strike_list_all_expiry.items() }
        if pprints:
            pprint(symbol_strike_list_all_expiry)
        if returns:
            return symbol_strike_list_all_expiry
        else:
            futures = []
            with concurrent.futures.ThreadPoolExecutor(max_workers=self.__max_workers) as executor:
                for strike_list_urls in symbol_strike_list_all_expiry.values():
                    for strike, strike_url in strike_list_urls.items():
                        futures.append(executor.submit(self.__save_symbol_file, strike.upper(), strike_url, GraphsData=True))
            concurrent.futures.wait(futures)

    def fetch_nbf_index_future_json(self):
        index_urls_to_fetch = {
                                'NIFTY50_VS_INDIAVIX': "https://www.nseindia.com/api/getNifty50VsindiaVix",
                                'NIFTY_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true&preopen=true",
                                'NIFTY_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true",
                                'NIFTY_NEXT50_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20NEXT%2050&indices=true&preopen=true",
                                'NIFTY_NEXT50_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20NEXT%2050&indices=true",
                                'NIFTY_MIDCAP50_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20MIDCAP%2050&indices=true&preopen=true",
                                'NIFTY_MIDCAP50_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20MIDCAP%2050&indices=true",
                                'BANKNIFTY_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20BANK&indices=true&preopen=true",
                                'BANKNIFTY_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20BANK&indices=true",
                                'FINNIFTY_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20FINANCIAL%20SERVICES&indices=true&preopen=true",
                                'FINNIFTY_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20FINANCIAL%20SERVICES&indices=true"
                              }
        symbols = {'NIFTY': 'nse50_fut', 'BANKNIFTY': 'nifty_bank_fut', 'FINNIFTY': 'finnifty_fut'}
        symbol_futures_urls = {symbol:f'https://www.nseindia.com/api/liveEquity-derivatives?index={ticker}' for symbol, ticker in symbols.items()}
        symbol_json_data = {symbol:self.fetch(url) for symbol, url in symbol_futures_urls.items()}
        symbol_futureidx_urls = {f"{item['identifier'][6:-6]}FUT":f"https://www.nseindia.com/api/chart-databyindex?index={item['identifier']}" \
                                for symbol, json_data in symbol_json_data.items() for item in json_data['data'] if item['underlying'] == symbol}
        symbol_indexes_futures_urls = index_urls_to_fetch | symbol_futureidx_urls
        futures = []
        with concurrent.futures.ThreadPoolExecutor(max_workers=self.__max_workers) as executor:
            for symbol, future_url in symbol_indexes_futures_urls.items():
                futures.append(executor.submit(self.__save_symbol_file, symbol.upper(), future_url, GraphsData=True))
        concurrent.futures.wait(futures)

    def __upload_to_telegram(self, _file_paths: list, _user_name: str = None, _invite_link: str = None, _caption: str = None):
        _username_groupname_or_invite_link = f"'https://t.me/joinchat/{_invite_link}'" if _invite_link and not _user_name \
                                            else _user_name if _user_name and not _invite_link else "shabhas"
        _send_as_album = '--album ' if len(_file_paths) > 1 else ''
        _files_to_be_sent = (' ').join(_file_paths)
        _caption_to_be_included = f" --caption {json.dumps(_caption)} " if _caption else " "
        _cli_initials = f"telegram-upload --to {_username_groupname_or_invite_link}"
        _cli = f"{_cli_initials}{_caption_to_be_included}{_send_as_album}{_files_to_be_sent}"
        os.system(_cli)

    def __state(self, _state=None, _save=False, _read=False):
        if _save:
            with open('Program_State.json', 'w') as outfile:
                json.dump(_state, outfile, indent=2)
        if _read:
            with open('Program_State.json', 'r') as in_file:
                _state = json.loads(in_file.read())
            return _state

    def __program_state(self):
        if not os.path.isfile(os.path.join(os.getcwd(), 'Program_State.json')):
            _program_state = {  "All_Fetched": False, "Intraday_Fetched": False, "Fetch_Date": dt.today().date().strftime("%d-%b-%Y"), \
                                "Index_Futures_Fetched": False, "Index_Options_Fetched": False, "All_Expiries_Options": False }
            self.__state(_state=_program_state, _save=True)
            return _program_state
        else:
            _program_state = self.__state(_read=True)
            if ((dt.today().date().strftime("%d-%b-%Y") != _program_state["Fetch_Date"]) and _program_state["All_Fetched"]):
                _program_state = {  "All_Fetched": False, "Intraday_Fetched": False, "Fetch_Date": dt.today().date().strftime("%d-%b-%Y"), \
                                    "Index_Futures_Fetched": False, "Index_Options_Fetched": False, "All_Expiries_Options": False }
                self.__state(_state=_program_state, _save=True)
            return _program_state

    def run_daily(self):
        # Tell Python to run the handler() function when SIGINT is recieved
        _program_state = self.__program_state()
        signal(SIGINT, self.__exit_handler)
        print('>>>>>>> Running Program. Press CTRL-C to exit. <<<<<<<', end="\n", flush=True)
        if not _program_state["All_Fetched"]:
            while (dt.combine(dt.now(), dtt.time(0, 0, 1)) < dt.now() < dt.combine(dt.now(), dtt.time(15, 46, 30))) and not _program_state["Intraday_Fetched"]:
                if (dt.combine(dt.now(), dtt.time(9, 14, 0)) < dt.now() < dt.combine(dt.now(), dtt.time(15, 45, 0))):
                    self.fetch_nse_options_json()
                    self.fetch_nse_futures_json()
                elif (dt.combine(dt.now(), dtt.time(15, 45, 0)) < dt.now() < dt.combine(dt.now(), dtt.time(15, 46, 29))):
                    _program_state["Intraday_Fetched"] = True
                    self.__state(_state=_program_state, _save=True)
                else:
                    print(">>>>> THE TIME IS NOT IN THE RANGE TO FETCH <<<<<<", end="\n", flush=True)
                    sleep(random.uniform(60, 75))
                sleep(random.uniform(35.5, 45.5))
            if not _program_state["Index_Futures_Fetched"]:
                self.fetch_nbf_index_future_json()
                _program_state["Index_Futures_Fetched"] = True
                self.__state(_state=_program_state, _save=True)
            if not _program_state["Index_Options_Fetched"]:
                self.fetch_nse_options_graphs_json()
                _program_state["Index_Options_Fetched"] = True
                self.__state(_state=_program_state, _save=True)
            if not _program_state["All_Expiries_Options"]:
                self.fetch_nse_all_options_graphs_json()
                _program_state["All_Expiries_Options"] = True
                self.__state(_state=_program_state, _save=True)
            if _program_state["Intraday_Fetched"] and _program_state["Index_Futures_Fetched"] and _program_state["Index_Options_Fetched"] \
                and _program_state["All_Expiries_Options"]:
                _program_state["All_Fetched"] = True
                self.__state(_state=_program_state, _save=True)
        if _program_state["All_Fetched"]:
            while not os.path.isfile(self.__sbull_zip_file):
                print(">>>>>>>>>> WAITING FOR THE OTHER ZSTD COMPRESSION TO FINISH <<<<<<<<<<")
                sleep(60)
            if not os.path.isfile(f"./{self.__today_folder_name}.tar.zst") and not os.path.isfile(f"./Zipped_Files/{self.__today_folder_name}.tar.zst") \
            and (os.path.exists(self.__today_folder_path) and os.path.isdir(self.__today_folder_path)):
                print(">>>>> FETCH FINISHED, COMPRESSING NOW <<<<<<", end="\n", flush=True)
                os.system(f"tar -I 'zstd --ultra -22' -cvf {self.__today_folder_name}.tar.zst {self.__today_folder_name}/")
                # os.system(f"tar -I 'zstd -T0 --ultra -22' -cf {self.__today_folder_name}.tar.zst {self.__today_folder_name}/")
                # os.system(f"tar -I 'zstd -T0 -19' -cf {self.__today_folder_name}.tar.zst {self.__today_folder_name}/")
                os.system(f"mv ./{self.__today_folder_name}.tar.zst ./Zipped_Files/")
                os.system(f"gio trash ./{self.__today_folder_name}")
            if os.path.isfile(f"./Zipped_Files/{self.__today_folder_name}.tar.zst") and not (os.path.exists(self.__today_folder_path) and os.path.isdir(self.__today_folder_path)):
                self.__upload_to_telegram(_file_paths = [f"./Zipped_Files/{self.__today_folder_name}.tar.zst"], _invite_link = 'j2C9hawf1I5hMDRk', \
                    _caption = "NIFTY, BANKNIFTY & FINNIFTY Futures and Options LTP Data (Expiries: All, Source: NSE)")
                print(">>>>> PROGRAM FINISHED, EXITING NOW <<<<<<", end="\n", flush=True)
                exit(0)

    def custom_command_if_i_need_anything_to_do_cutom(self):
        index_urls_to_fetch = {
                                'NIFTY50_VS_INDIAVIX': "https://www.nseindia.com/api/getNifty50VsindiaVix",
                                'NIFTY_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true&preopen=true",
                                'NIFTY_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true",
                                'NIFTY_NEXT50_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20NEXT%2050&indices=true&preopen=true",
                                'NIFTY_NEXT50_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20NEXT%2050&indices=true",
                                'NIFTY_MIDCAP50_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20MIDCAP%2050&indices=true&preopen=true",
                                'NIFTY_MIDCAP50_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20MIDCAP%2050&indices=true",
                                'BANKNIFTY_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20BANK&indices=true&preopen=true",
                                'BANKNIFTY_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20BANK&indices=true",
                                'FINNIFTY_INDEX_PreMarket': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20FINANCIAL%20SERVICES&indices=true&preopen=true",
                                'FINNIFTY_INDEX': "https://www.nseindia.com/api/chart-databyindex?index=NIFTY%20FINANCIAL%20SERVICES&indices=true"
                              }
        futures = []
        with concurrent.futures.ThreadPoolExecutor(max_workers=self.__mid_workers) as executor:
            for symbols, urls in index_urls_to_fetch.items():
                futures.append(executor.submit(self.__save_symbol_file, symbols.upper(), urls, GraphsData=True))
        concurrent.futures.wait(futures)

# =========================================================================================================================== #

if __name__ == '__main__':
    nse = nsefetch()
    nse.run_daily()
    # nse.custom_command_if_i_need_anything_to_do_cutom()
    # nse.fetch_nse_options_json()
    # nse.fetch_nse_futures_json()
    # nse.fetch_nbf_index_future_json()
    # nse.fetch_nse_options_graphs_json()
    # nse.fetch_nse_all_options_graphs_json()

# =========================================================================================================================== #





















# import requests


# session = requests.Session()
# adapter = requests.adapters.HTTPAdapter(
#     pool_connections=100,
#     pool_maxsize=100)
# session.mount('http://', adapter)
# response = session.get("http://example.org")



# =========================================================================================================================== #
# async def async_fetch_urls(symbol: str, url: str, client_session: aiohttp.ClientSession, rate_limiter: RateLimiter):
#     no_of_retries: int = 11
#     back_off = (random.uniform(58.853, 59.999), random.uniform(33.979, 57.791), random.uniform(29.123, 33.357), random.uniform(11.111, 27.737), 
#                 random.uniform(4.051, 10.011), random.uniform(2.011, 3.047), random.uniform(1.571, 1.991), random.uniform(1.143, 1.147), 
#                 random.uniform(0.731, 0.979), random.uniform(0.541, 0.701), random.uniform(0.229, 0.363))
#     for _ in range(no_of_retries):
#         if no_of_retries != 11: print(f'>!>!> Retrying For > {symbol} > Retries Left > {no_of_retries-1} <!<!<')
#         try:
#             async with rate_limiter.throttle():
#                 # print(url)
#                 r = await client_session.get(url)
#                 if r.status != 200:
#                     r.raise_for_status()
#             response = await r.text()
#             if "502 Bad Gateway" in response:
#                 no_of_retries -= 1
#                 print(f'502 Bad Gateway > {symbol} > Retries Left > {no_of_retries}')
#                 print(f'Going to sleep for > {back_off[no_of_retries]} Seconds')
#                 await asyncio.sleep(back_off[no_of_retries])
#                 # client_session.cookies.clear()  # Reset cookies
#                 r = await client_session.get('https://www.nseindia.com')
#                 if r.status != 200:
#                     r.raise_for_status()
#                 await r.text()
#                 continue
#             # print(response)
#             else:
#                 try:
#                     return [symbol, json.loads(response)]
#                 except Exception as e:
#                     r.release()
#                     raise Exception(f'Unexpected Error Occured > {response}, Exception is {e}')
#         except aiohttp.client_exceptions.ClientResponseError:
#             no_of_retries -= 1
#             print(f'ClientResponseError > {symbol} > Retries Left > {no_of_retries}')
#             print(f'Going to sleep for > {back_off[no_of_retries]} Seconds')
#             await asyncio.sleep(back_off[no_of_retries])
#             # client_session.cookies.clear()  # Reset cookies
#             # client_session.headers.clear()  # Reset headers
#             # client_session.close()  # End HTTP connection
#             r = await client_session.get('https://www.nseindia.com')
#             if r.status != 200:
#                 r.raise_for_status()
#             await r.text()
#             continue
#         except aiohttp.ClientConnectionError:
#             no_of_retries -= 1
#             print(f'ClientResponseError > {symbol} > Retries Left > {no_of_retries}')
#             print(f'Going to sleep for > {back_off[no_of_retries]} Seconds')
#             await asyncio.sleep(back_off[no_of_retries])
#             # client_session.cookies.clear()  # Reset cookies
#             r = await client_session.get('https://www.nseindia.com')
#             if r.status != 200:
#                 r.raise_for_status()
#             await r.text()
#             continue
#         except aiohttp.ClientError:
#             no_of_retries -= 1
#             print(f'ClientResponseError > {symbol} > Retries Left > {no_of_retries}')
#             print(f'Going to sleep for > {back_off[no_of_retries]} Seconds')
#             await asyncio.sleep(back_off[no_of_retries])
#             # client_session.cookies.clear()  # Reset cookies
#             r = await client_session.get('https://www.nseindia.com')
#             if r.status != 200:
#                 r.raise_for_status()
#             await r.text()
#             continue
#         except aiohttp.client_exceptions.ClientOSError:
#             no_of_retries -= 1
#             print(f'ClientResponseError > {symbol} > Retries Left > {no_of_retries}')
#             print(f'Going to sleep for > {back_off[no_of_retries]} Seconds')
#             await asyncio.sleep(back_off[no_of_retries])
#             # client_session.cookies.clear()  # Reset cookies
#             r = await client_session.get('https://www.nseindia.com')
#             if r.status != 200:
#                 r.raise_for_status()
#             await r.text()
#             continue
# =========================================================================================================================== #
# async def async_fetch_nse_all_options_graphs_json(strike_list_urls):
#     headers = {
#                         'Host':'www.nseindia.com',
#                         'User-Agent':'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:82.0) Gecko/20100101 Firefox/82.0',
#                         'Accept':'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
#                         'Accept-Language':'en-US,en;q=0.5',
#                         'Accept-Encoding':'gzip, deflate, br',
#                         'DNT':'1',
#                         'Connection':'keep-alive',
#                         'Upgrade-Insecure-Requests':'1',
#                         'Pragma':'no-cache',
#                         'Cache-Control':'no-cache',
#                         }
#     async with RateLimiter(rate_limit=7, concurrency_limit=7) as rate_limiter:
#         async with aiohttp.ClientSession(headers=headers) as session:
#             return await asyncio.gather(*[asyncio.ensure_future(async_fetch_urls(strike, strike_url, session, rate_limiter)) for strike, strike_url in strike_list_urls.items()])
# =========================================================================================================================== #
# async def run_async_fetch_nse_all_options_graphs_json(strike_list_urls):
#     return await async_fetch_nse_all_options_graphs_json(strike_list_urls)
# =========================================================================================================================== #
# def run_save_all_options_graphs_json(strike_list_urls):
#     start_timer0 = time.time()
#     uvloop.install()
#     json_data = asyncio.run(async_fetch_nse_all_options_graphs_json(strike_list_urls))
#     strike_and_json_data = { item[0]:item[1] for item in json_data if item }
#     for strike, json_data in strike_and_json_data.items():
#         with open(f'{strike}_{dt.now().strftime("%Y-%m-%d_%H:%M:%S")}.json', 'w') as outfile:
#             json.dump(json_data, outfile, indent=4)
# =========================================================================================================================== #

    # elif (dt.combine(dt.now(), dtt.time(15, 46, 0)) < dt.now() < dt.combine(dt.now(), dtt.time(23, 59, 59))):
    #     # symbol_strike_list_all_expiry = fetch_nse_all_options_graphs_json(returns=True)
    #     # for strike_list_urls in symbol_strike_list_all_expiry.values():
    #     #     run_save_all_options_graphs_json(strike_list_urls)
    #     # fetch_nse_index_future_json()
    #     # sleep(random.uniform(57.51, 59.97))
    #     # fetch_nse_options_graphs_json()
    #     # sleep(random.uniform(57.51, 59.97))

# =========================================================================================================================== #
