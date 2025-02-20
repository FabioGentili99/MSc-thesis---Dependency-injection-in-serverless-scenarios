const args = process.argv.slice(2);

// Check if an argument is provided
if (args.length < 1) {
    console.log("Usage: node main.js <your_argument>");
    return;
}

// Read the first argument
const arg = args[0];
hello(arg);

function hello(arg) {
    console.log("hello " + arg);
}

