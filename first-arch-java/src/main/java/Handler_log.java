import com.google.gson.Gson;
import com.google.gson.JsonElement;
import io.nats.client.Connection;
import io.nats.client.Nats;
import io.nats.client.Subscription;
import io.nats.client.Message;
import io.nats.client.impl.NatsMessage;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


import java.io.FileWriter;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import java.time.Instant;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

import static java.lang.System.exit;

public class Handler_log {
    private static final Logger logger = LoggerFactory.getLogger(Handler_acl.class);


    private static void handler() throws IOException, InterruptedException, TimeoutException {
        Injector injector = new Injector();
        Service aclService = injector.getServiceById("log");
        String topic = aclService.getServiceTopic();
        Map<String, String> message = new HashMap<>();
        message.put("timestamp", "2024-11-28T16:05:34");
        message.put("message", "ciao");
        message.put("severity", "info");
        String result = invokeFunction(topic, message);
        System.out.println("logging result: " + result);
    }

    private static String invokeFunction(String topic, Map<String, String> message) throws IOException, InterruptedException, TimeoutException {
        Connection nc = Nats.connect();
        long start = System.currentTimeMillis();
        Gson g = new Gson();
        String msg = g.toJson(message);
        Message response = nc.request(NatsMessage.builder()
                        .subject(topic)
                        .data(msg)
                        .build(),
                Duration.ofSeconds(5));
        long end = System.currentTimeMillis();
        logger.info("logging function executed in {} ms", end - start);

        String result = new String(response.getData(), StandardCharsets.UTF_8);
        return result;
    }

    public static void main(String[] args) throws IOException, InterruptedException, TimeoutException {
        handler();
        exit(0);
    }
}

