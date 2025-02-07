# Use this file to test requests of the server

import requests
import time

ROOT = "http://127.0.0.1:1145"
token = -1

def format_route(route: str) -> str:
    if route.startswith('/'):
        return ROOT + route
    else:
        return ROOT + '/' + route

def init():
    requests.post(ROOT + "/user/create", json = {"name": "test", "pass": "test"})

def login() :
    global token
    response = requests.post(ROOT + "/user/login", json = {"name": "test", "pass": "test"})
    if response.ok:
        token = response.json()["token"]

def tok() -> dict:
    return {"token": token}

def timestamp() -> int:
    timestamp = time.time()
    return int(round(timestamp))

def post(route: str, **json) -> requests.Response:
    global token
    return requests.post(format_route(route), json = tok() | json)

def get(route: str, **json) -> requests.Response:
    global token
    return requests.get(format_route(route), json = tok() | json)
