Ingrediants:

1. pythonAPI that will keep pumping RAW card info as JSON into rabbitMQ, it has 2 endpoints:
    1. Start. /start. ; will start populating the que
    2. Stop. /stop; will cease hostilites and declare peace. . . .

2. rabbitMQ as the bus that will distribute the info between the apps
