import com.google.gson.Gson;
import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoDatabase;
import com.mongodb.client.MongoCollection;
import com.mongodb.client.model.Filters;
import org.bson.Document;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Injector {
    private static final Logger logger = LoggerFactory.getLogger(Injector.class);
    private static final String dbUrl = "mongodb://localhost:27017";
    private static final String dbName = "services";
    private static final String collectionName = "services";

    private MongoClient client;
    private MongoDatabase db;
    private MongoCollection<Document> collection;

    public Injector() {
        this.client = MongoClients.create(dbUrl);
        this.connect();
    }

    private void connect() {
        this.db = client.getDatabase(dbName);
        this.collection = db.getCollection(collectionName);
    }

    /**
     * Registers a new service in the database.
     * @param id - The id of the service.
     * @param name - The name of the service.
     * @param topic - The topic of the service.
     */
    public void registerService(String id, String name, String topic) {
        Document service = new Document("id", id)
                .append("ServiceName", name)
                .append("ServiceTopic", topic);
        collection.insertOne(service);
        System.out.println("1 document inserted");
    }

    /**
     * Retrieves a service by its ID.
     * @param id - The ID of the service.
     * @return The service document or null if not found.
     */
    public Service getServiceById(String id) {
        long start = System.currentTimeMillis();
        Document doc = collection.find(Filters.eq("id", id)).first();
        long end = System.currentTimeMillis();
        logger.info("Read from MongoDB table executed in {} ms", (end - start));
        Gson gson = new Gson();
        Service service = gson.fromJson(doc.toJson(), Service.class);  
        return service;
    }

    public void close() {
        client.close();
    }
}

