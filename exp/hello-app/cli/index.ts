import { ClientLogin, AccessTokenWrapper } from "@calimero-network/calimero-client";

// Types
interface ApiClient {
  getApplications: () => Promise<any>;
}

// Auth Client Setup
const setupAuthClient = async () => {
  const nodeUrl = process.env.NODE_URL || "http://localhost:2428";
  const applicationId = process.env.APPLICATION_ID;

  if (!applicationId) {
    throw new Error("APPLICATION_ID environment variable is required");
  }

  // Initialize auth client
  const client = new AccessTokenWrapper({
    getNodeUrl: () => nodeUrl,
  });

  return client;
};

// API Client Setup
const setupApiClient = (authClient: any): ApiClient => {
  return {
    getApplications: async () => {
      const response = await authClient.get("/admin-api/applications");
      return response.data;
    },
  };
};

// Main Function
async function main() {
  try {
    console.log("🔑 Setting up auth client...");
    const authClient = await setupAuthClient();

    console.log("🌐 Setting up API client...");
    const apiClient = setupApiClient(authClient);

    console.log("📋 Fetching applications...");
    const apps = await apiClient.getApplications();
    console.log("Applications:", apps);
  } catch (error) {
    console.error("❌ Error:", error);
  }
}

main();
