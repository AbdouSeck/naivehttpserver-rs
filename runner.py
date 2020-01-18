#!/usr/bin/env python3
"""
Hit the target endpoints with GET requests
"""
import sys
from argparse import ArgumentParser, RawDescriptionHelpFormatter as RDHF
from urllib.request import urlopen
from threading import Thread


def read_url(url):
    """
    Get request targeting the provided endpoint

    :param url: A URL to submit a GET request to
    :return: None
    """
    try:
        urlopen(url)
    except:
        excp = sys.exc_info()[1]
        print("Failed to open {u}: {e}".format(u=url, e=excp))


def get_it(base_url, n=100):
    """
    Send n GET requests to the provided url.
    This involves ranging between 0 and 100 (exclusive)
    and hitting /sleep if i is even.

    :param base_url: The base url of the web server (landing page)
    :param n: The number of requests to send
    :return: List of Thread objects to be joined
    """
    threads = []
    for i in range(n):
        if i % 2 == 0:
            t = Thread(target=read_url, args=("{}/sleep".format(base_url),))
        else:
            t = Thread(target=read_url, args=(base_url,))
        t.start()
        threads.append(t)
    return threads


if __name__ == '__main__':
    parser = ArgumentParser(description=__doc__, formatter_class=RDHF)
    parser.add_argument(
        '--url', '-u',
        help='The base of the target url',
        default='http://127.0.0.1:7878'
    )
    parser.add_argument(
        '--number', '-n',
        help='Number of GET requests to send',
        default=100,
        type=int
    )
    args = parser.parse_args()
    [tr.join() for tr in get_it(args.url, args.number)]

