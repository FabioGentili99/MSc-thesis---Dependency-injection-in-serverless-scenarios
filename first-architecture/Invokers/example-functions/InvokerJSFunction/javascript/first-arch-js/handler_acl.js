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
const acl_service = await injector.getServiceById("acl")
const address = acl_service.ServiceAddress
const msg = {message: "{\"user\": \"Bob\", \"permission\": \"read\"}"};
const result = await invokeFunction(address, msg)

console.log("access control result: ", result);
process.exit(0, "access control result: ", result)
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


hanlder();

