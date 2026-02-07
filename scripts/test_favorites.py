#!/usr/bin/env python

from api_test_client import ApiTestClient

URL = "https://api.weibo.cn/2/favorites"

payload = {
    "c": "weicoabroad",
    "ua": "ROG-ROG%20Phone%207%20Ultimate__weibo__11.6.3__android__android9",
    "from": "12DC195010",
    "s": "6c888888",
    "source": "4215535043",
    "wm": "2468_1001",
    "page": "1",
    "count": 20,
    "lang": "zh_CN",
    "mix_media_enable": 1,
}


def main():
    client = ApiTestClient(session_file="../session.json")
    client.run_test(URL, payload, method="get", output_filename="new_favorites.json")


if __name__ == "__main__":
    main()
