const Injector = require("./Injector.js")
const { createLogger, format, transports } = require("winston");

// Custom CSV formatter
const csvFormat = format.printf(({ timestamp, level, label, message }) => {
  return `${timestamp},${level.toUpperCase()},${label},${message}`;
});


const logger = createLogger({
      level:"info",
      format: format.combine(
            format.timestamp({ format: "YYYY-MM-DDTHH:mm:ss.SSS[Z]" }), // ISO 8601 format
            csvFormat
          ),
      transports: [
        new transports.File({ filename: "logs.txt" }),
      ],
    });

async function hanlder(){
const injector = new Injector()
const log_service = await injector.getServiceById("log")
const address = log_service.ServiceAddress
const msg = {message: "{\"timestamp\": \"2024-11-28T16:05:34\", \"message\": \"ciao\", \"severity\": \"info\"}"}
const result = await invokeFunction(address, msg)
console.log("logging result:", result)
process.exit(0, "logging result:", result)
}



async function invokeFunction ( address, message){
  try {
      
     
      const start = Date.now()
      
      const response = await fetch(address, {
        method: "POST",
        body: JSON.stringify(message),
        headers: {
          "Content-type": "application/json; charset=UTF-8"
        }
      });
      const end = Date.now()
      logger.info(`access control function executed in ${end-start} ms `)

      return response.text()
    } catch (err) {
      console.error("Error publishing message:", err);
    }
}

hanlder()