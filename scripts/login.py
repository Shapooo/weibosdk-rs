#!/usr/bin/env python

import requests

SENDCODE_URL = "https://api.weibo.cn/2/account/login"

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
    "lang": "zh_CN",
    # "aid": "01AxyKeuFeNsI8hIVTHNoiSSCebnCoF3ey5BwkA9OGoASc_qU.",
    "getuser": "1",
    "getoauth": "1",
    "getcookie": "1",
    "phone": "13277919960",  # 修改此处手机号
    "smscode": "",
}

try:
    smscode = input("Please input smscode: ")
    payload["smscode"] = smscode
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
