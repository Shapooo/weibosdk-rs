#!/usr/bin/env python

from api_test_client import ApiTestClient

URL = "https://api.weibo.cn/2/2statuses/show"

payload = {
    "c": "weicoabroad",
    "networktype": "wifi",
    "ua": "ROG-ROG%20Phone%207%20Ultimate__weibo__11.6.3__android__android9",
    "uid": "1401527553",
    "from": "12DC195010",
    # "oldwm": "3333_1001",
    "wm": "2468_1001",
    "lang": "zh_CN",
    # "aid": "01AyPIx-jMVwTWJAaNX5bykc3nSzTYoGJw5HrCTRv69ulYJDg.",
    # "cum": "D65802CA",
    "id": 5179586393932632,
    "isGetLongText": 1,
}


def main():
    client = ApiTestClient(session_file="../session.json")
    client.run_test(URL, payload, method="get", output_filename="statuses_show.json")


if __name__ == "__main__":
    main()
