
# IN PROGRESS

## Run

1. get RabbitMQ started:

```powershell
docker run --hostname YuGiOh-rabbit  --name yugioh-bus -p 5672:5672 -p 15672:15672 rabbitmq:3-management
```

> 5672: to access the node
>15672: to access the web interface

2. start pubing the cards

```powershell
cd cardDispenser;
python -m venv venv;
.\venv\Scripts\activate # source venv/bin/activate
pip install -r reqs.txt

uvicorn main:app --reload -p 4444 # or whatever u want
```

Starring:

1. FastAPI as pythonAPI that will keep pumping RAW card info as JSON into rabbitMQ, it has 2 endpoints:
    1. Start. /start. ; will start populating the que
    2. Stop. /stop; will cease hostilites and declare peace. . . .

2. RabbitMQ as the bus that will distribute the info between the apps
