export class ApiClient {
  private baseUrl: string;
  private token: string;

  constructor() {
    this.baseUrl = localStorage.getItem("jocky_api_url") || "http://127.0.0.1:8080";
    this.token = localStorage.getItem("jocky_api_token") || "";
  }

  updateConfig(url: string, token: string) {
    this.baseUrl = url;
    this.token = token;
    localStorage.setItem("jocky_api_url", url);
    localStorage.setItem("jocky_api_token", token);
  }

  async get<T>(path: string): Promise<T> {
    const isDemo = localStorage.getItem("jocky_demo_mode") === "true";
    if (isDemo) {
      return this.mockData(path) as any as T;
    }

    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };
    
    if (this.token) {
      headers["Authorization"] = `Bearer ${this.token}`;
    }

    const res = await fetch(`${this.baseUrl}${path}`, {
      method: "GET",
      headers,
    });

    if (!res.ok) {
      throw new Error(`API Error: ${res.statusText}`);
    }

    return res.json() as Promise<T>;
  }

  private mockData(path: string) {
    if (path.includes("/health")) return { status: "ok" };
    if (path.includes("/endpoints")) return { endpoints: [{ id: "mock-ep-1", name: "DEMO-ENDPOINT", platform: "windows", status: "ONLINE", last_seen: new Date().toISOString() }] };
    if (path.includes("/investigations")) {
      if (path === "/api/v1/investigations") {
        return { investigations: [{ id: "inv-demo-1", endpoint_id: "mock-ep-1", platform: "windows", start_time: new Date().toISOString(), end_time: new Date().toISOString(), finding_count: 2, risk_score: 50, priority: "HIGH" }] };
      }
      return { 
        schema_version: "0.1", investigation_id: "inv-demo-1", endpoint_id: "mock-ep-1", target: "LOCAL", platform: "windows", start_time: new Date().toISOString(), end_time: new Date().toISOString(),
        risk_score: { score: 50, priority: "HIGH" },
        graph: { nodes: [{ type: "evidence", id: "ev-1", evidence_type: "process", source: "windows", attributes: { pid: 1234, name: "cmd.exe" } }], edges: [] },
        timeline: { events: [{ timestamp: new Date().toISOString(), event_type: "PROCESS", evidence_id: "ev-1", host: "LOCAL", description: "cmd.exe started" }] },
        findings: []
      };
    }
    return {};
  }
}

export const apiClient = new ApiClient();
