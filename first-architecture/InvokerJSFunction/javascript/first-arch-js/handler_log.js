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
const log_service = await injector.getServiceById("log")
const topic = log_service.ServiceTopic
const message = {timestamp: "2024-11-28T16:05:34", message: "ciao", severity: "info"}
const result = await invokeFunction(topic, message)
console.log("logging result:", result)
process.exit(0, "logging function executed")
}



async function invokeFunction (topic, message){
    try {
        // Connect to the NATS server
        const nc = await connect({ servers: natsURL }); 
        const start = Date.now()
        // Publish the message
        const response = await nc.request(topic, JSON.stringify(message));
        const end = Date.now()
        logger.info(`logging function executed in ${end-start} ms `)
        // Close the connection
        nc.drain();
        return (response.data.toString())
      } catch (err) {
        console.error("Error publishing message:", err);
      }
}

hanlder()