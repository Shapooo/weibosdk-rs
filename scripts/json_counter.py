import json
from typing import List, DefaultDict
from collections import defaultdict
from pprint import pprint

# 常量，请根据需要修改
FILE_PATH = "full_favorites.json"
KEY_TO_COUNT = "page_info"


def is_the_key(key):
    return key == KEY_TO_COUNT


def count_func(value, counts: DefaultDict, trace: List):
    key = "media_info"
    if key in value:
        for k in value[key]:
            counts[k] += 1
    if "cards" in value:
        for c in value["cards"]:
            if key in c:
                for k in c[key]:
                    counts[k] += 1


def count_key_values(data, key_to_count, counts: DefaultDict, trace: List[str]):
    """
    递归地遍历数据结构，统计指定 key 的值的出现频次
    """
    if isinstance(data, dict):
        for key, value in data.items():
            trace.append(value)
            if is_the_key(key):
                count_func(value, counts, trace)
            elif isinstance(value, (dict, list)):
                count_key_values(value, key_to_count, counts, trace)
            trace.pop()
    elif isinstance(data, list):
        for item in data:
            count_key_values(item, key_to_count, counts, trace)


def main():
    """
    主函数
    """
    try:
        with open(FILE_PATH, "r", encoding="utf-8") as f:
            data = json.load(f)
    except FileNotFoundError:
        print(f"错误：找不到文件 {FILE_PATH}")
        return
    except json.JSONDecodeError:
        print(f"错误：{FILE_PATH} 文件不是一个有效的 JSON 文件")
        return

    counts = defaultdict(int)
    trace = []
    count_key_values(data, KEY_TO_COUNT, counts, trace)

    print(f"对 key '{KEY_TO_COUNT}' 的值统计结果如下：")
    pprint(dict(counts))


if __name__ == "__main__":
    main()
