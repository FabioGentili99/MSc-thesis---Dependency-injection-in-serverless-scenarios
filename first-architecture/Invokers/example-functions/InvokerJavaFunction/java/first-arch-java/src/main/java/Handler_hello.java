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

public class Handler_hello {
    private static final Logger logger = LoggerFactory.getLogger(Handler_hello.class);



    private static void handler() throws IOException, InterruptedException, TimeoutException {
        Injector injector = new Injector();
        Service aclService = injector.getServiceById("hello");
        String address = aclService.getServiceAddress();
        Map<String, String> message = new HashMap<>();
        message.put("message", "world");
        String result = invokeFunction(address, message);
        System.out.println("hello function result: " + result);
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

        logger.info("hello function executed in {} ms", end - start);


        return response.body();
    }

    public static void main(String[] args) throws IOException, InterruptedException, TimeoutException {
        handler();
        exit(0);
    }
}

