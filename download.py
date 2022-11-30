from dotenv import load_dotenv
import requests
from os import getenv
from sys import argv
from time import sleep

BACKOFF_SCALE=2
BACKOFF_TIME=10

if __name__ == "__main__":
    if len(argv) < 3:
        print("Missing arguments")
        exit(1)
    year = int(argv[1])
    day = int(argv[2])
    load_dotenv()
    cookie = getenv("AOC_COOKIE")

    if cookie is None:
        print("Missing env AOC_COOKIE")
        exit(1)

    counter = 0
    while counter < 10:
        counter += 1
        r = requests.get(f"https://adventofcode.com/{year}/day/{day}/input",cookies={"session":cookie})

        if r.status_code != 200:
            print(f"Status: {r.status_code}\nWaiting {BACKOFF_TIME}s")
            sleep(BACKOFF_TIME)
            if counter <= 5:
                BACKOFF_TIME *= BACKOFF_SCALE
        else:
            print("SUCCESS")
            with open(f"inputs/day{day:02}.txt", "w+") as f:
                f.write(r.text)
            exit(0)
    print("Could not download input")
    exit(1)