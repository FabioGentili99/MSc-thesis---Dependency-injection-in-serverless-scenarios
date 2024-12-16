const { MongoClient } = require('mongodb');
const winston = require("winston");

/*
const logger = winston.createLogger({
  level: "info",
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.json()
  ),
  transports: [
    new winston.transports.Console(),
    new winston.transports.File({ filename: "combined.log" }),
  ],
});
*/

const dbUrl = 'mongodb://localhost:27017';
const dbName = 'services';
const collectionName = 'services';

class Injector {
  constructor() {
    this.logger = winston.createLogger({
      level: "info",
      format: winston.format.combine(
        winston.format.timestamp(),
        winston.format.json()
      ),
      transports: [
        new winston.transports.File({ filename: "logs.txt" }),
      ],
    });
    this.dbUrl = dbUrl;
    this.dbName = dbName;
    this.collectionName = collectionName;
    this.client = new MongoClient(dbUrl);
    this.connect();
  }

  connect() {
    if (!this.client.isConnected) {
        this.client.connect();
    }
    this.db = this.client.db(this.dbName);
    this.collection = this.db.collection(this.collectionName);
  }



  /**
   * Registers a new service in the database.
   * @param {string} id - The id of the service.
   * @param {string} name - The name of the service.
   * @param {string} topic - The topic of the service.
   */
  async registerService(id, name, topic) {
    const service = { id: id,
                      ServiceName: name,
                      ServiceTopic: topic };
    await this.collection.insertOne(service, function(err, res) {
        if (err) throw err;
        console.log("1 document inserted");
    });
  }

  /**
   * Retrieves a service by its ID.
   * @param {string} id - The ID of the service.
   * @returns {Promise<Object|null>} The service document or null if not found.
   */
  async getServiceById(id) {
    const start = Date.now()
    const service = await this.collection.findOne({ id: id });
    const end = Date.now()
    this.logger.info(`Read from MongoDB table executed in ${end-start} ms `)
    return service;
  }


  async close() {
    await this.client.close();
  }


}

module.exports = Injector;