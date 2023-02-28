from fastapi import FastAPI

app = FastAPI()


@app.get("/")
def handle_root():
    return {"endpoints": {"/start": "starts pumping", "/stop": "stops pumping"}}


@app.post("/start")
def handle_post():
    return "pumping"


@app.post("/stop")
def handle_stop():
    return "stoping"
