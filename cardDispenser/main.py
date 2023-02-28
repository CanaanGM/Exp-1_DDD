from fastapi import FastAPI
import pika, json, time, threading
from events import Events
from card_ops import CARDS

app = FastAPI()
SENDING = False
sending = Events()


def send_message() -> None:
    """Contacts space Rabbit and throws children cards at it"""
    connection = pika.BlockingConnection(
        pika.ConnectionParameters("localhost")
    )  # change when compose to app name
    channel = connection.channel()
    queue = channel.queue_declare(queue="YuGiOh-Card-Queue")  # create queue if !queue
    messages_in_queue_count = queue.method.message_count
    if messages_in_queue_count > 0:
        print(f"Queue already full with { messages_in_queue_count} messages")
        exit()
    print("sending. . .")
    for card in CARDS:
        channel.basic_publish("", "YuGiOh-Card-Queue", json.dumps(card))
    print("sending done~!")
    connection.close()


@app.get("/")
def handle_root():
    return {"endpoints": {"/start": "starts pumping", "/stop": "stops pumping"}}


def change_state(state: bool) -> None:
    """Toggles publishing state ON/OFF"""
    sending.on_change += set_sending_state
    sending.on_change(state)


def set_sending_state(state: bool):
    global SENDING
    SENDING = state
    sent_messages = 0
    while sent_messages <= len(CARDS) and SENDING:
        send_message()
        time.sleep(10)
        sent_messages += 1


@app.post("/start")
async def handle_post():
    threading.Thread(target=send_message).start()
    # threading.Thread(target=change_state, args=(True,)).start()
    # inform the requester that we are sending
    return "sending"


@app.post("/stop")
async def handle_stop():
    threading.Thread(target=change_state, args=(False,)).start()
    return "stoping"
