import com.google.gson.Gson;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;


public class LambdaStressTest {

    private static final String exampleFunction = "http://192.168.17.118:8081/asyncfunction/example-fn";





    private static final String LAMBDA_URL = exampleFunction; // Replace with your Lambda URL
    private static final int INITIAL_REQUESTS_PER_SECOND = 1;
    private static final int PEAK_REQUESTS_PER_SECOND = 500;
    private static final int INCREMENT = 1;
    private static final int DECREMENT = 1000;
    private static final int PRELIMINARY_PHASE_DURATION_SECONDS = 0;
    private static final int STEP_DURATION_SECONDS = 1;

    public static void main(String[] args) {
        try {
            stressTestLambda();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }

    private static void stressTestLambda() throws InterruptedException {
        ScheduledExecutorService scheduler = Executors.newScheduledThreadPool(1);
        // Preliminary phase with constant load
        //runPreliminaryPhase(scheduler, INITIAL_REQUESTS_PER_SECOND, PRELIMINARY_PHASE_DURATION_SECONDS);
        // Incremental phase to peak
        for (int currentRequestsPerSecond = INITIAL_REQUESTS_PER_SECOND;
             currentRequestsPerSecond <= PEAK_REQUESTS_PER_SECOND;
             currentRequestsPerSecond += INCREMENT) {
            runLoadPhase(scheduler, currentRequestsPerSecond, STEP_DURATION_SECONDS);
        }

        /*
        // Decremental phase back to initial
        for (int currentRequestsPerSecond = PEAK_REQUESTS_PER_SECOND;
             currentRequestsPerSecond > INITIAL_REQUESTS_PER_SECOND;
             currentRequestsPerSecond -= DECREMENT) {
            runLoadPhase(scheduler, currentRequestsPerSecond, STEP_DURATION_SECONDS);
        }
        // final phase with constant load
        runPreliminaryPhase(scheduler, INITIAL_REQUESTS_PER_SECOND, PRELIMINARY_PHASE_DURATION_SECONDS/2);
        */

        scheduler.shutdown();
        scheduler.awaitTermination(1, TimeUnit.MINUTES);
    }

    private static void runPreliminaryPhase(ScheduledExecutorService scheduler, int requestsPerSecond, int durationSeconds) throws InterruptedException {
        System.out.println("Starting preliminary phase...");
        runLoadPhase(scheduler, requestsPerSecond, durationSeconds);
    }

    private static void runLoadPhase(ScheduledExecutorService scheduler, int requestsPerSecond, int durationSeconds) throws InterruptedException {
        System.out.println("Running load phase: " + requestsPerSecond + " requests per second for " + durationSeconds + " seconds");
        for (int i = 0; i < durationSeconds; i++) {
            scheduler.schedule(() -> sendRequests(requestsPerSecond), i, TimeUnit.SECONDS);
        }
        Thread.sleep(durationSeconds * 1000L);
    }

    private static void sendRequests(int numberOfRequests) {
        ExecutorService executor = Executors.newFixedThreadPool(numberOfRequests);
        for (int i = 0; i < numberOfRequests; i++) {
            executor.submit(() -> {
                /*
                try (CloseableHttpClient httpClient = HttpClients.createDefault()) {

                    HttpRequest request = HttpRequest.newBuilder()
                            .uri(URI.create(LAMBDA_URL))
                            .POST(HttpRequest.BodyPublishers.ofByteArray(message.getBytes()))
                            .setHeader("Content-type", "application/json")
                            .build();



                    HttpPost httpPost = new HttpPost(LAMBDA_URL);
                    httpPost.setHeader(CONTENT_TYPE, "application/Json");
                    StringEntity entity = new StringEntity(message, StandardCharsets.UTF_8);
                    httpPost.setEntity(entity);
                    HttpResponse response = httpClient.execute(httpPost);
                    System.out.println("Response Code: " + response.getStatusLine().getStatusCode() + response.getEntity().getContent());
                } catch (Exception e) {
                    e.printStackTrace();
                }
                */
                HttpClient client = HttpClient.newHttpClient();
                Gson g = new Gson();
                Map msg = new HashMap<String, String>();
                msg.put("message", Long.toString(System.currentTimeMillis()));
                String message = g.toJson(msg);
                HttpRequest request = HttpRequest.newBuilder()
                        .uri(URI.create(LAMBDA_URL))
                        .POST(HttpRequest.BodyPublishers.ofByteArray(message.getBytes()))
                        .setHeader("Content-type", "application/json")
                        .build();

                HttpResponse<String> response = null;
                try {
                    response = client.send(request, HttpResponse.BodyHandlers.ofString());
                } catch (IOException e) {
                    throw new RuntimeException(e);
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
                System.out.println("Response Code: " + response.statusCode() + response.body());
            });
        }
        executor.shutdown();
        try {
            executor.awaitTermination(1, TimeUnit.MINUTES);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }
}