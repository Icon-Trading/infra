import requests
import os

WEBHOOK = os.getenv("SLACK_WEBHOOK")

def slack(msg: str):
    requests.post(WEBHOOK, json={"text": msg}, timeout=1)

slack("⚚ *ICON TRADING*\n Sharpe ratio over 3000!")