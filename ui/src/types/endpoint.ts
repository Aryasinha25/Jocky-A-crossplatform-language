export interface Endpoint {
  id: string;
  name: string;
  platform: string;
  status: string;
  last_seen: string;
}

export interface EndpointListResponse {
  endpoints: Endpoint[];
}
