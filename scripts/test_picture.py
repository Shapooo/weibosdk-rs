#!/usr/bin/env python

import requests
import os
from urllib.parse import urlparse
from api_test_client import ApiTestClient

# Replace with the actual picture URL you want to test
URL = "https://wx1.sinaimg.cn/thumbnail/53899d01ly1i3ihymdqvyj20l50t6dmw.jpg"
URL = "http://vip.storage.weibo.com/feed_cover/star_1554_mobile_new.png?version=2025032601"


def download_picture(client: ApiTestClient, url: str):
    """
    Downloads a picture, following redirects, and saves it with the filename from the final URL.
    """
    if not client.gsid:
        print("Could not proceed without a gsid.")
        return

    try:
        # Make the request, allowing redirects (which is the default)
        # No payload is sent for a simple GET request
        response = requests.get(url, headers=client.headers, allow_redirects=True)
        response.raise_for_status()

        # Get the final URL after any redirects
        final_url = response.url

        # Extract the filename from the path of the final URL
        parsed_url = urlparse(final_url)
        filename = os.path.basename(parsed_url.path)

        if not filename:
            print("Could not determine filename from the final URL.")
            filename = "downloaded_picture.jpg"

        # Save the image content to a file
        with open(filename, "wb") as f:
            f.write(response.content)

        print(f"Successfully downloaded picture and saved as {filename}")

    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")


def main():
    """
    Main function to initialize the client and start the download.
    """
    client = ApiTestClient(session_file="../session.json")
    download_picture(client, URL)


if __name__ == "__main__":
    main()
