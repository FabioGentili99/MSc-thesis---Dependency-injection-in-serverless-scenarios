const { MongoClient } = require('mongodb');
const { createLogger, format, transports } = require("winston");

const dbUrl = process.env.MONGODB || 'mongodb://192.168.17.118:27017';
const dbName = 'services';
const collectionName = 'services';

// Custom CSV formatter
const csvFormat = format.printf(({ timestamp, level, message }) => {
  return `${timestamp},${level.toUpperCase()},${message}`;
});

class Injector {
  constructor() {
    this.logger = createLogger({
      level:"info",
      format: format.combine(
            format.timestamp({ format: "YYYY-MM-DDTHH:mm:ss.SSS[Z]" }), // ISO 8601 format
            csvFormat
          ),
      transports: [
        new transports.File({ filename: "logs.txt" }),
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
  async registerService(id, name, address) {
    const service = { id: id,
                      ServiceName: name,
                      ServiceAddress: address };
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