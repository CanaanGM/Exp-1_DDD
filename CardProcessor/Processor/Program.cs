using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;

using Processor;

using RabbitMQ.Client;
using RabbitMQ.Client.Events;

namespace YuGiOh_CMD_server
{
    internal class Program
    {
        public static bool Processing { get; set; } = false;

        public static void Main(string[] args)
        {
            Run();
        }

        public async static void Run()
        {
            while (true)
            {
                switch (Console.ReadLine())
                {
                    case "Start":
                        Task.Run(async () => await HandleStart());
                        break;
                    case "Stop":
                        HandleStop();
                        break;
                    default:
                        Console.WriteLine("awww");
                        break;
                }
            }
        }
        private static void SendMessage(YuGiOhCard recievedCard)
        {
            var factory = new ConnectionFactory { HostName = "localhost" };
            using var connection = factory.CreateConnection();
            using var channel = connection.CreateModel();

            channel.QueueDeclare(queue: "YuGiOh-Processed-Queue", false, false, false, null);
            string message = JsonSerializer.Serialize(recievedCard);
             var body = Encoding.UTF8.GetBytes(message);

          channel.BasicPublish(
              exchange: string.Empty, 
              routingKey: "YuGiOh-Processed-Queue", 
              basicProperties: null, 
              body
              );
        }

        private static void ConsumeMessage(IModel channel)
        {
            channel.QueueDeclare(
                    queue: "YuGiOh-Card-Queue",false,false,false,null
                );
            var consumer = new EventingBasicConsumer(channel);
            consumer.Received += Consumer_Received;

            channel.BasicConsume(queue: "YuGiOh-Card-Queue",
                     autoAck: true,
                     consumer: consumer);
        }

        private static void Consumer_Received(object? sender, BasicDeliverEventArgs e)
        {
            var body = e.Body.ToArray();
            var message = Encoding.UTF8.GetString(body);

            SendMessage(JsonSerializer.Deserialize<YuGiOhCard>(message));
        }

        async public static Task HandleStart()
        {
            Processing = true;
            var factory = new ConnectionFactory { HostName = "localhost" };
            using var connection = factory.CreateConnection();
            using var channel = connection.CreateModel();

            while (Processing)
            {
                try
                {
                    ConsumeMessage(channel);
                    
                    if (!Processing) break;
                }
                catch (Exception ex)
                {
                    Console.Write(ex);
                }
            }
            Console.WriteLine("Sending");
        }
        public static void HandleStop()
        {
            Processing = false;
            Console.WriteLine("Stopping");
        }
    }
}