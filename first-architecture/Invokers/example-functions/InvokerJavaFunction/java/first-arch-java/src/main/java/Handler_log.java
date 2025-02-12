import com.google.gson.Gson;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

import static java.lang.System.exit;

public class Handler_log {
    private static final Logger logger = LoggerFactory.getLogger(Handler_log.class);


    private static void handler() throws IOException, InterruptedException, TimeoutException {
        Injector injector = new Injector();
        Service logService = injector.getServiceById("log");
        String address = logService.getServiceAddress();
        Map<String, String> message = new HashMap<>();
        message.put("message", "{\"timestamp\":\"2024-11-28T16:05:34\",\"message\":\"ciao\",\"severity\":\"info\"}");
        String result = invokeFunction(address, message);
        System.out.println("logging result: " + result);
    }

    private static String invokeFunction(String address, Map<String, String> message) throws IOException, InterruptedException, TimeoutException {
        Gson g = new Gson();
        String msg = g.toJson(message);
        HttpClient client = HttpClient.newHttpClient();
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create(address))
                .POST(HttpRequest.BodyPublishers.ofByteArray(msg.getBytes()))
                .setHeader("Content-type", "application/json")
                .build();
        long start = System.currentTimeMillis();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());
        long end = System.currentTimeMillis();
        logger.info("logging function executed in {} ms", end - start);


        return response.body();
    }

    public static void main(String[] args) throws IOException, InterruptedException, TimeoutException {
        handler();
        exit(0);
    }
}

