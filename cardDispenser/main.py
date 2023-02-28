from fastapi import FastAPI
import pika
from events import Events
import time, threading


app = FastAPI()
SENDING = False
sending = Events()


def send_message():
    connection = pika.BlockingConnection(
        pika.ConnectionParameters("localhost")
    )  # change when compose to app name
    channel = connection.channel()
    channel.queue_declare(queue="YuGiOh-Card-Queue")  # create queue if !queue
    channel.basic_publish("", "YuGiOh-Card-Queue", '{"card-TEST": {"name": "test"}}')
    # print("[x] Sent message")
    connection.close()


@app.get("/")
def handle_root():
    return {"endpoints": {"/start": "starts pumping", "/stop": "stops pumping"}}


def change_state(state: bool):
    sending.on_change += set_sending_state
    sending.on_change(state)


def set_sending_state(state: bool):
    global SENDING
    SENDING = state
    while SENDING:
        send_message()
        time.sleep(10)


@app.post("/start")
async def handle_post():
    threading.Thread(target=change_state, args=(True,)).start()
    # inform the requester that we are sending
    return "sending"


@app.post("/stop")
async def handle_stop():
    threading.Thread(target=change_state, args=(False,)).start()
    return "stoping"
