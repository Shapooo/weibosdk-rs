#!/usr/bin/env python3
import requests
import json
from generate_s import generate_s

uid = ""


class ApiTestClient:
    def __init__(self, session_file="../session.json"):
        self.gsid = self._get_gsid(session_file)
        self.headers = {
            "User-Agent": "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710",
            "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
            "Connection": "Keep-Alive",
            "Accept-Encoding": "gzip",
        }

    def _get_gsid(self, path: str):
        """Reads gsid from a json file."""
        try:
            with open(path, "r", encoding="utf-8") as f:
                data = json.load(f)
                gsid = data.get("gsid")
                global uid
                uid = data.get("uid")
                return gsid
        except FileNotFoundError:
            print(f"Error: File not found at {path}")
            return None
        except json.JSONDecodeError:
            print(f"Error: Could not decode JSON from {path}")
            return None

    def run_test(
        self, url, payload, method="get", output_filename="apitest_result.json"
    ):
        if not self.gsid:
            print("Could not proceed without a gsid.")
            return

        payload["gsid"] = self.gsid
        payload["s"] = generate_s(uid=uid, from_=payload["from"])

        try:
            if method.lower() == "get":
                response = requests.get(url, headers=self.headers, params=payload)
            elif method.lower() == "post":
                response = requests.post(url, headers=self.headers, data=payload)
            else:
                print(f"Unsupported method: {method}")
                return

            response.raise_for_status()
            if "errmsg" in response.json():
                print(response.json())
                exit(1)

            result = response.json()

            with open(output_filename, "w", encoding="utf-8") as f:
                json.dump(result, f, indent=4, ensure_ascii=False)

            print(f"Successfully fetched data and saved to {output_filename}")

        except requests.exceptions.RequestException as e:
            print(f"Request failed: {e}")
        except ValueError as e:
            print(f"Failed to parse JSON response: {e}")
