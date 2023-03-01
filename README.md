
# IN PROGRESS
>
> card info gotten from [Ygoprodeck.com](https://db.ygoprodeck.com/api/v7/cardinfo.php), i got all the cards as to not annoy their API

## Run

1. get RabbitMQ started:

```powershell
docker run --hostname YuGiOh-rabbit  --name yugioh-bus -p 5672:5672 -p 15672:15672 rabbitmq:3-management
```

> 5672: to access the node
>15672: to access the web interface

2. start pubing the cards

```powershell
cd cardDispenser
python -m venv venv
.\venv\Scripts\activate # source venv/bin/activate
pip install -r reqs.txt

uvicorn main:app --reload -p 4444 # or whatever u want
```

3. run the c# code

Starring:

1. FastAPI as pythonAPI that will keep pumping RAW card info as JSON into rabbitMQ `queueName=YuGiOh-Card-Queue`, it has 2 endpoints:
    1. Start. /start. ; will start populating the que
    2. Stop. /stop; will cease hostilites and declare peace. . . .

> the way the inital plan was for the API to get either a `start` or `stop` commands via REST and ivoke an event depending, at least that was the main idea, toggle switching, but as the data we have is finite, i didnt want duplication hence the way i did it.

2. RabbitMQ as the bus that will distribute the info between the apps

3. C# 6 as the cards proccessor, it takes the card from pyPi and removes some props and throws it again at the space rabbit this time onto a different queue `queueName=YuGiOh-Processed-Queue`

# Resources

1. [turn Json to language](https://json2csharp.com/)
1. [RabbitMq c# docs](https://www.rabbitmq.com/tutorials/tutorial-one-dotnet.html)
1. [RabbitMq python docs](https://www.rabbitmq.com/tutorials/tutorial-three-python.html)
1. [python events package](https://pypi.org/project/Events/)
1. [this](https://stackoverflow.com/questions/47290108/how-to-open-rabbitmq-in-browser-using-docker-container) to read queue size
