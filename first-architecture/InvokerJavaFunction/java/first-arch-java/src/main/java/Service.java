import org.bson.types.ObjectId;


public class Service {
    private ObjectId _id;
    private String id;
    private String ServiceName;
    private String ServiceTopic;

    public Service(String ServiceName, String ServiceTopic, ObjectId _id, String id) {
        this.ServiceName = ServiceName;
        this.ServiceTopic = ServiceTopic;
        this._id = _id;
        this.id = id;
    }

    public ObjectId getOid() {
        return _id;
    }

    public void setId(ObjectId _id) {
        this._id = _id;
    }

    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public String getServiceName() {
        return ServiceName;
    }

    public void setServiceName(String ServiceName) {
        this.ServiceName = ServiceName;
    }

    public String getServiceTopic() {
        return ServiceTopic;
    }

    public void setServiceTopic(String ServiceTopic) {
        this.ServiceTopic = ServiceTopic;
    }


    
}
