import io.nats.client.Connection;
import io.nats.client.Message;
import io.nats.client.Nats;
import io.nats.client.impl.NatsMessage;
import org.apache.http.HttpResponse;
import org.apache.http.client.methods.HttpGet;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClients;

import java.io.IOException;
import java.time.Duration;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;


public class LambdaStressTest {

    private static final String handler_topic = "handler";
    private static final String natsURL = "nats://192.168.17.118:4222";
    private static final Connection nc;

    static {
        try {
            nc = Nats.connect(natsURL);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
    }

    private static final String LAMBDA_TOPIC = handler_topic; // Replace with your Lambda URL
    private static final int INITIAL_REQUESTS_PER_SECOND = 2;
    private static final int PEAK_REQUESTS_PER_SECOND = 20;
    private static final int INCREMENT = 1;
    private static final int DECREMENT = 2;
    private static final int PRELIMINARY_PHASE_DURATION_SECONDS = 120;
    private static final int STEP_DURATION_SECONDS = 2;

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
        runPreliminaryPhase(scheduler, INITIAL_REQUESTS_PER_SECOND, PRELIMINARY_PHASE_DURATION_SECONDS);
        // Incremental phase to peak
        for (int currentRequestsPerSecond = INITIAL_REQUESTS_PER_SECOND;
             currentRequestsPerSecond <= PEAK_REQUESTS_PER_SECOND;
             currentRequestsPerSecond += INCREMENT) {
            runLoadPhase(scheduler, currentRequestsPerSecond, STEP_DURATION_SECONDS);
        }
        // Decremental phase back to initial
        for (int currentRequestsPerSecond = PEAK_REQUESTS_PER_SECOND;
             currentRequestsPerSecond > INITIAL_REQUESTS_PER_SECOND;
             currentRequestsPerSecond -= DECREMENT) {
            runLoadPhase(scheduler, currentRequestsPerSecond, STEP_DURATION_SECONDS);
        }
        // final phase with constant load
        runPreliminaryPhase(scheduler, INITIAL_REQUESTS_PER_SECOND, PRELIMINARY_PHASE_DURATION_SECONDS/2);
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
                try {
                    Message response = nc.request(NatsMessage.builder()
                                    .subject(handler_topic)
                                    .data("ciao")
                                    .build(),
                            Duration.ofSeconds(5));
                    System.out.println("Response: " + response.getData());
                } catch (Exception e) {
                    e.printStackTrace();
                }
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