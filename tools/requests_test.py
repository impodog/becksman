# Use this file to test requests of the server

import requests

ROOT = "http://127.0.0.1:1145"
token = -1

def init():
    requests.post(ROOT + "/user/create", json = {"name": "test", "pass": "test"})

def login() :
    global token
    response = requests.post(ROOT + "/user/login", json = {"name": "test", "pass": "test"})
    if response.ok:
        token = response.json()["token"]

def tok() -> dict:
    return {"token": token}

