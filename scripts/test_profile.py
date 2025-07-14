#!/usr/bin/env python

from api_test_client import ApiTestClient

URL = "https://api.weibo.cn/2/profile/statuses"

payload = {
    "c": "weicoabroad",
    "v_p": "89",
    "skin": "default",
    "count": "20",
    "networktype": "wifi",
    "ua": "ROG-ROG%20Phone%207%20Ultimate__weibo__11.6.3__android__android9",
    "uid": "1401527553",
    "from": "12DC195010",
    "oldwm": "3333_1001",
    "wm": "2468_1001",
    "page": "1",
    "lang": "zh_CN",
    "android_id": "eebbb67889d77bb4",
    "v_f": "2",
    "containerid": "1076031401527553_-_WEIBO_SECOND_PROFILE_WEIBO_ARTICAL",
    "wb_version": "5005",
    "aid": "01AyPIx-jMVwTWJAaNX5bykc3nSzTYoGJw5HrCTRv69ulYJDg.",
    "cum": "D65802CA",
}

def main():
    client = ApiTestClient(session_file="../session.json")
    client.run_test(URL, payload, method="get", output_filename="profile.json")

if __name__ == "__main__":
    main()
