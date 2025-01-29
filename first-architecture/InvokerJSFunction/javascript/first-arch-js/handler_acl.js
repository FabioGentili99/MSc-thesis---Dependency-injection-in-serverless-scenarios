const Injector = require("./Injector.js")
const { connect } = require('nats');
const winston = require("winston");

natsURL = process.env.NATSSERVER || "nats://192.168.17.118:4222"

const logger = winston.createLogger({
  level: "info",
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.json()
  ),
  transports: [
    new winston.transports.File({ filename: "logs.txt" }),
  ],
});

async function hanlder(){

const injector = new Injector()
const acl_service = await injector.getServiceById("acl")
const topic = acl_service.ServiceTopic
const message = {user: 'Bob', permission: 'read'};
const result = await invokeFunction(topic, message)

console.log("access control result: ", result);
process.exit(0, "access control function executed")
}



async function invokeFunction ( topic, message){
    try {
        // Connect to the NATS server
        const nc = await connect({ servers: natsURL }); // Replace with your NATS server address
        
        //console.log("Connected to NATS");
        const start = Date.now()
        // Publish the message
        const response = await nc.request(topic, JSON.stringify(message));
        const end = Date.now()
        logger.info(`access control function executed in ${end-start} ms `)
        // Close the connection
        nc.drain();
        //console.log("Connection closed");
        return JSON.parse(response.data)
      } catch (err) {
        console.error("Error publishing message:", err);
      }
}


hanlder();

