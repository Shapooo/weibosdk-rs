#!/usr/bin/env python3

import requests

SENDCODE_URL = "https://api.weibo.cn/2/account/login_sendcode"

# 定义请求头字典 (可修改)
headers = {
    # "X-Sessionid": "d1d292e9-e657-4036-b9a6-65769cb657d8",
    "User-Agent": "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710",
    "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
    "Host": "api.weibo.cn",
    "Connection": "Keep-Alive",
    "Accept-Encoding": "gzip"
}

# 定义请求体参数字典 (可方便修改)
payload = {
    "c": "weicoabroad",
    "from": "12DC195010",
    "source": "4215535043",
    "lang": "zh_CN",
    "locale": "zh_CN",
    "wm": "2468_1001",
    "ua": "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710",
    # "aid": "01AxyKeuFeNsI8hIVTHNoiSSCebnCoF3ey5BwkA9OGoASc_qU.",
    "phone": "13277919960"  # 修改此处手机号
}

try:
    # 发送POST请求
    response = requests.post(
        SENDCODE_URL,
        headers=headers,
        data=payload  # 自动编码为x-www-form-urlencoded
    )

    # 检查请求是否成功
    response.raise_for_status()

    # 解析JSON响应
    result = response.json()
    print("请求成功，响应数据:")
    print(result)

except requests.exceptions.RequestException as e:
    print(f"请求失败: {e}")
except ValueError as e:
    print(f"JSON解析失败: {e}")
