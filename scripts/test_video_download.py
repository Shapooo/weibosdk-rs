#!/usr/bin/env python

from api_test_client import ApiTestClient


def main():
    """
    Runs the test to fetch status details.
    The response will contain video URLs if the post has a video.
    """
    client = ApiTestClient(session_file="../session.json")
    url = input()
    client.run_test(url, None, method="get", output_filename="test.mp4")


if __name__ == "__main__":
    main()
