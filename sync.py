#!/usr/bin/env python3

import re
import subprocess

import httpx


def get_token():
    result = subprocess.run(['exercism', 'configure', '--show'], capture_output=True)
    return re.findall(r'^Token:.*?([\w-]+)$', result.stderr.decode("utf-8"), re.M)[0]


def make_request(token, page):
    headers = {'Authorization': f'Bearer {token}'}

    return httpx.get(f'https://exercism.org/api/v2/solutions?page={page}', headers=headers).json()


def download_exercise(exercise, track):
    result = subprocess.run(['exercism', 'download', f'--exercise={exercise}', f'--track={track}'], capture_output=True)
    return result.returncode


def main():
    token = get_token()

    current_page = 1
    total_pages = 1
    while current_page <= total_pages:
        data = make_request(token, current_page)

        total_pages = data['meta']['total_pages']
        print(f'Page [{current_page}/{total_pages}]')

        for n, item in enumerate(data['results']):
            exercise = item['exercise']['slug']
            track = item['track']['slug']

            result = download_exercise(exercise, track)
            if result != 0:
                print(f'    [FAILED] {track} - {exercise}')
            else:
                print(f'    [OK] {track} - {exercise}')

        current_page += 1


if __name__ == '__main__':
    main()
