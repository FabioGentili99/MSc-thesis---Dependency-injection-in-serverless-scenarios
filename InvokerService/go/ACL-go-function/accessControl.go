package main

import (
	"encoding/json"
	"fmt"
	"os"
)

// Define the Access Control List (ACL)
var acl = map[string][]string{
	"admin":  {"read", "write", "delete"},
	"editor": {"read", "write"},
	"viewer": {"read"},
}

var roles = map[string]string{
	"Bob":   "admin",
	"Alice": "editor",
	"John":  "viewer",
}

// Function to check if a user role has a specific permission
func hasAccess(user string, permission string) bool {
	// Get the role
	role, _ := roles[user]

	// Get the permissions for the role
	permissions, roleExists := acl[role]

	// If the role is not found in the ACL, deny access
	if !roleExists {
		return false
	}

	// Check if the permission exists in the role's permissions
	for _, p := range permissions {
		if p == permission {
			return true
		}
	}
	return false
}

func main() {
	// Get input from the command line
	var params struct {
		User       string `json:"user"`
		Permission string `json:"permission"`
	}

	if len(os.Args) < 2 {
		fmt.Println("Error: No input provided")
		os.Exit(1)
	}

	err := json.Unmarshal([]byte(os.Args[1]), &params)
	if err != nil {
		fmt.Println("Error parsing input:", err)
		os.Exit(1)
	}

	// Check access and display the result
	accessGranted := hasAccess(params.User, params.Permission)
	os.Stdout.WriteString(fmt.Sprintf("%t", accessGranted))
}
