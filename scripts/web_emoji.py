#!/usr/bin/env python

import pprint
import requests
import json

URL = "https://weibo.com/ajax/statuses/config"

headers = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:141.0) Gecko/20100101 Firefox/141.0",
    "Accept": "application/json, text/plain, */*",
    "Accept-Language": "en-US,en;q=0.5",
    # "Accept-Encoding": "gzip, deflate, br, zstd",
    "Accept-Encoding": "gzip",
    # "X-Requested-With": "XMLHttpRequest",
    # "client-version": "v2.47.101",
    # "server-version": "v2025.08.14.2",
    # "X-XSRF-TOKEN": "ddgoznhVpJwwOUNqOdyCbWJq",
    # "Connection": "keep-alive",
    "Referer": "https://weibo.com/",
    "Cookie": "SCF=AjWQVVy1IILN8wgFvxsxCI0AGvNNdhQQc2WL-RfMfjS60QVqwjB2LUVnMTKtpAbRd1H1ezXGiQKpMA4EM_vFjig.; SUB=_2A25FpF51DeRhGedJ41QR9SvIyTuIHXVm2N-9rDV8PUNbmtAbLUaskW9NUbyV1WXQxWaAMPCCPiBE4nto-wQ2lixP; SUBP=0033WrSXqPxfM725Ws9jqgMF55529P9D9WWFuq8C.odx30xuj5LZdzp55NHD95QpS0nceh-fShzNWs4DqcjB-cyyMsvadcva9J9DqcLyd5tt; XSRF-TOKEN=ddgoznhVpJwwOUNqOdyCbWJq; WBPSESS=2A7r_bMt1vcByBlnXOFfFKtn-SPBUqhUqM40QOYoPb761OdOqp336s63BZzb8T_V5-ARWbFtw5opEIY6Q9EjuuaQe3X_1raUvJMWvuuJZu2nn53C7wIgHiovGVYbDEaq; ALF=02_1757920037",
    # "Sec-Fetch-Dest": "empty",
    # "Sec-Fetch-Mode": "cors",
    # "Sec-Fetch-Site": "same-origin",
    # "TE": "trailers",
}


def main():
    try:
        response = requests.get(URL, headers=headers)
        pprint.pprint(response)
        response.raise_for_status()
        result = response.json()
        pprint.pprint(result)
        json_txt = json.dumps(result, indent=2)
        with open("web_emoji.json", "w") as f:
            f.write(json_txt)

    except requests.exceptions.RequestException as e:
        print(e)


if __name__ == "__main__":
    main()
