package injector

import (
	"context"
	"log"
	"os"
	"time"

	"github.com/sirupsen/logrus"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Service struct {
	id           string
	ServiceName  string
	ServiceTopic string
}

type Injector struct {
	logger     *logrus.Logger
	client     *mongo.Client
	db         *mongo.Database
	collection *mongo.Collection
}

const (
	dbUrl          = "mongodb://localhost:27017"
	dbName         = "services"
	collectionName = "services"
)

func NewInjector() *Injector {
	clientOptions := options.Client().ApplyURI(dbUrl)
	client, err := mongo.Connect(context.TODO(), clientOptions)
	if err != nil {
		log.Fatal(err)
	}
	logger := logrus.New()
	logFile, _ := os.OpenFile("logs.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0666)
	defer logFile.Close()
	logger.Out = logFile

	injector := &Injector{
		logger:     logger,
		client:     client,
		db:         client.Database(dbName),
		collection: client.Database(dbName).Collection(collectionName),
	}
	return injector
}

func (i *Injector) RegisterService(id, name, topic string) {
	service := bson.D{
		{Key: "id", Value: id},
		{Key: "ServiceName", Value: name},
		{Key: "ServiceTopic", Value: topic},
	}
	_, err := i.collection.InsertOne(context.TODO(), service)
	if err != nil {
		log.Fatal(err)
	}
}

func (i *Injector) GetServiceById(id string) Service {
	start := time.Now()
	var service Service
	err := i.collection.FindOne(context.TODO(), bson.D{{Key: "id", Value: id}}).Decode(&service)
	if err != nil {
		log.Fatal(err)
	}
	end := time.Now()
	i.logger.Infof("Read from MongoDB table executed in %v ms", end.Sub(start).Milliseconds())
	return service
}

func (i *Injector) Close() {
	if err := i.client.Disconnect(context.TODO()); err != nil {
		log.Fatal(err)
	}
}
