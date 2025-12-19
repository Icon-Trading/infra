import requests
import os

bots = {
    "icontrading": {
        "token": os.getenv("TELEGRAM_TOKEN_ICONTRADING"),
        "chat_id": [000000],
    }
}

bot = "icontrading"

TOKEN = bots[bot]["token"]
CHAT_ID = bots[bot]["chat_id"]

def send_telegram(msg: str):
    url = f"https://api.telegram.org/bot{TOKEN}/sendMessage"
    for chat_id in CHAT_ID:
        payload = {
            "chat_id": chat_id,
            "text": msg,
            "parse_mode": "Markdown"
        }
        requests.post(url, json=payload, timeout=1)

# Example signal
send_telegram("⚚ *ICON TRADING*\n Sharpe ratio over 3000!")
# send_telegram("Hi!")
