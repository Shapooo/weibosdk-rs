#!/usr/bin/env python
import pprint
import requests
import json
import time

URL = "https://weibointl.api.weibo.cn/portal.php"

payload = {
    "ct": "util",
    "a": "expression_all",
    "user_id": "0",
    "time": "",
    "ua": "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710_miui",
    "lang": "zh_CN",
    "version": "6710",
}

headers = {
    "User-Agent": "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710",
    "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
    "Connection": "Keep-Alive",
    "Accept-Encoding": "gzip",
}


def main():
    payload["time"] = str(int(time.time() * 1000))
    pprint.pprint(payload)
    output_filename = "emoji.json"
    try:
        response = requests.get(URL, headers=headers, params=payload)
        response.raise_for_status()
        result = response.json()
        if "errmsg" in result:
            print(result)
            exit(1)

        with open(output_filename, "w", encoding="utf-8") as f:
            json.dump(result, f, indent=4, ensure_ascii=False)

        print(f"Successfully fetched data and saved to {output_filename}")

    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")
    except json.JSONDecodeError as e:
        print(f"Failed to parse JSON response: {e}")


if __name__ == "__main__":
    main()
