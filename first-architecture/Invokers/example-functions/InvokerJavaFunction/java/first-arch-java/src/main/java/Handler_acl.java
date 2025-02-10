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

public class Handler_acl {
    private static final Logger logger = LoggerFactory.getLogger(Handler_acl.class);



    private static void handler() throws IOException, InterruptedException, TimeoutException {
        Injector injector = new Injector();
        Service aclService = injector.getServiceById("acl");
        String address = aclService.getServiceAddress();
        Map<String, String> message = new HashMap<>();
        message.put("message", "{\"user\": \"Bob\",\"permission\": \"read\"}");
        //String message = "{\"message\": {\"user\": \"Bob\",\"permission\": \"read\"}";
        String result = invokeFunction(address, message);
        System.out.println("access control result: " + result);
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

        logger.info("access control function executed in {} ms", end - start);


        return response.body();
    }

    public static void main(String[] args) throws IOException, InterruptedException, TimeoutException {
        handler();
        exit(0);
    }
}

