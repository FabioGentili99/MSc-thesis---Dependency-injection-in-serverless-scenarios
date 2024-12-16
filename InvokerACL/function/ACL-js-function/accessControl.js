// Define the Access Control List (ACL)
const acl = {
    admin: ["read", "write", "delete"],
    editor: ["read", "write"],
    viewer: ["read"]
};

const roles = {
    Bob: "admin",
    Alice: "editor",
    John: "viewer"
}

/**
 * Function to check if a user role has a specific permission
 * @param {string} user - The user name
 * @param {string} permission - The permission to check (e.g., "read", "write", "delete")
 * @returns {boolean} - True if the role has the permission, false otherwise
 */
function hasAccess(user, permission) {
    //Get the role
    const role = roles[user];

    // Get the permissions for the role
    const permissions = acl[role];

    // If the role is not found in the ACL, deny access
    if (!permissions) {
        return false;
    }

    // Check if the permission exists in the role's permissions
    return permissions.includes(permission);
}

// Get input from the command line
const args = process.argv.slice(2);

// Ensure the correct number of arguments are provided
if (args.length !== 2) {
    console.error("Usage: node accessControl.js <userName> <permission>");
    process.exit(1);
}

const [user, permission] = args;

// Check access and display the result
const accessGranted = hasAccess(user, permission);
console.log(`${accessGranted}`);
