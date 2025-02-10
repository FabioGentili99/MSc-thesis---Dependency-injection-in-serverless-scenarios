const fs = require("fs");

/**
 * Logs a message with a given timestamp and severity level to a file.
 * @param {string} timestamp - The timestamp of the log (e.g., "2024-11-27T12:00:00Z").
 * @param {string} message - The log message.
 * @param {string} severity - The severity level (e.g., "info", "warn", "error").
 * @returns {boolean} - True if the logging operation was successful, false otherwise.
 */
function logMessageToFile(timestamp, message, severity) {
    const validSeverities = ["info", "warn", "error"];

    // Validate severity level
    if (!validSeverities.includes(severity)) {
        console.error("Invalid severity level. Valid options are: info, warn, error.");
        return false;
    }

    // Validate timestamp
    const date = new Date(timestamp);
    if (isNaN(date.getTime())) {
        console.error("Invalid timestamp format. Use ISO 8601 format (e.g., yyyy-mm-ddThh:mm:ssZ).");
        return false;
    }

    const logEntry = `[${timestamp}] [${severity.toUpperCase()}]: ${message}\n`;

    try {
        // Append the log entry to the logs.txt file
        fs.appendFileSync("logs.txt", logEntry, "utf8");
        return true; // Logging succeeded
    } catch (error) {
        console.error("Logging operation failed:", error.message);
        return false; // Logging failed
    }
}

// Get input from the command line
let params
try{
params = JSON.parse(process.argv[2]);
} catch (error) {
    console.error("Invalid input. Please provide the correct number of arguments.");
    process.exit(1);}
// Perform the logging operation
const result = logMessageToFile(params.timestamp, params.message, params.severity);
// Indicate success or failure
if (result) {
    console.log('logging operation succeeded');
} else {
    console.error("logging operation failed.");}