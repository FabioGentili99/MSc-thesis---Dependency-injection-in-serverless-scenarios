const Injector = require("./Injector.js")
const { connect } = require('nats');
const winston = require("winston");
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
console.log ("topic: ", topic)
const message = {timestamp: "2024-11-28T16:05:34", message: "ciao", severity: "info"}
const result = await invokeFunction(topic, message)

console.log("logging result:", result)

}



async function invokeFunction ( topic, message){
    try {
        // Connect to the NATS server
        const nc = await connect({ servers: "nats://localhost:4222" }); // Replace with your NATS server address
        
        console.log("Connected to NATS");
        const start = Date.now()

        // Publish the message
        const response = await nc.request(topic, JSON.stringify(message));
        const end = Date.now()
        logger.info(`logging function executed in ${end-start} ms `)

        
        // Close the connection
        nc.drain();
        console.log("Connection closed");
        return (response.data.toString())
      } catch (err) {
        console.error("Error publishing message:", err);
      }
}

hanlder()