classdef MatChatClient < handle
    properties
        Name
        Location
        LastReceived

        Server
        Timer
    end

    methods
        function client = MatChatClient(name, location)
            client.Name = name;
            client.Location = location;
            client.LastReceived = 0;

            client.Server = tcpserver("localhost", 30006);

            system("~/.local/bin/client localhost:30006 http://" + location + "/get/ &");

            client.Timer = timer();
            client.Timer.TimerFcn = @(~, ~)checkSocket();
            client.Timer.ExecutionMode = "fixedDelay";
            client.Timer.Period = .5;
            client.Timer.BusyMode = "drop";
            client.Timer.start();

            function checkSocket()
                if client.Server.NumBytesAvailable > 0
                    messages_raw = client.Server.read(client.Server.NumBytesAvailable, "char");
                    messages = jsondecode(messages_raw);

                    for message = messages(:)'
                        disp(message.name + ":" + message.content);
                    end
                end
            end
        end

        function send(client, content)
            import matlab.net.http.RequestMessage;
            import matlab.net.http.HeaderField;
            import matlab.net.http.RequestLine;
            import matlab.net.http.MessageBody;

            message = RequestMessage(matlab.net.http.RequestMethod.POST);
            message.Header = HeaderField("Content-Type", "application/json");

            obj.id = 0;
            obj.name = client.Name;
            obj.timestamp = "0";
            obj.content = content;

            message.Body = MessageBody(obj);
            response = send(message, client.Location + "/send");
        end

        function lr = get(client)
            import matlab.net.http.RequestMessage;
            import matlab.net.http.HeaderField;
            import matlab.net.http.RequestLine;
            import matlab.net.http.MessageBody;

            lr = client.LastReceived;

            message = RequestMessage;
            response = send(message, client.Location + "/get/" + client.LastReceived);

            if strcmp(response.StatusCode, "OK")
                messages = response.Body.Data;

                lr = max([messages.id]);
                client.LastReceived = lr;

                for message = messages(:)'
                    if ~strcmp(message.name, client.Name)
                        disp(message.name + ":" + message.content);
                    end
                end
            end
        end
    end
end