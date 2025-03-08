const Injector = require("./Injector.js")
const { createLogger, format, transports } = require("winston");

// Custom CSV formatter
const csvFormat = format.printf(({ timestamp, level, message }) => {
  return `${timestamp},${level.toUpperCase()},${message}`;
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
const hello_service = await injector.getServiceById("hello")
const address = hello_service.ServiceAddress
const msg = {message: "world"};
const result = await invokeFunction(address, msg)

console.log("hello function result: ", result);
process.exit(0, "hello function result: ", result)
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
        logger.info(`hello function executed in ${end-start} ms `)

        return response.text()
      } catch (err) {
        console.error("Error publishing message:", err);
      }
}


hanlder();

