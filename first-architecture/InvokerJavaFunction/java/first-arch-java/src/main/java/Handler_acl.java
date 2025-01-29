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
import java.util.Optional;
import java.util.concurrent.TimeoutException;

import static java.lang.System.exit;

public class Handler_acl {
    private static final Logger logger = LoggerFactory.getLogger(Handler_acl.class);
    private static final String natsURL = Optional.ofNullable(System.getenv("NATSSERVER")).orElse("nats://192.168.17.118:4222");



    private static void handler() throws IOException, InterruptedException, TimeoutException {
        Injector injector = new Injector();
        Service aclService = injector.getServiceById("acl");
        String topic = aclService.getServiceTopic();
        Map<String, String> message = new HashMap<>();
        message.put("user", "Bob");
        message.put("permission", "read");
        String result = invokeFunction(topic, message);
        System.out.println("access control result: " + result);
    }

    private static String invokeFunction(String topic, Map<String, String> message) throws IOException, InterruptedException, TimeoutException {
        Connection nc = Nats.connect(natsURL);
        long start = System.currentTimeMillis();
        Gson g = new Gson();
        String msg = g.toJson(message);
        Message response = nc.request(NatsMessage.builder()
                        .subject(topic)
                        .data(msg)
                        .build(),
                Duration.ofSeconds(5));
        long end = System.currentTimeMillis();
        logger.info("access control function executed in {} ms", end - start);

        String result = new String(response.getData(), StandardCharsets.UTF_8);
        return result.equals("true") ? "access granted" : "access denied";
    }

    public static void main(String[] args) throws IOException, InterruptedException, TimeoutException {
        handler();
        exit(0);
    }
}

