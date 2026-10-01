export interface InvestigationSummary {
  id: string;
  endpoint_id: string;
  platform: string;
  start_time: string;
  end_time: string;
  finding_count: number;
  risk_score: number;
  priority: string;
}

export interface InvestigationResult {
  schema_version: string;
  investigation_id: string;
  endpoint_id: string;
  target: string;
  platform: string;
  start_time: string;
  end_time: string;
  risk_score: {
    score: number;
    priority: string;
  };
  graph: {
    nodes: any[];
    edges: any[];
  };
  timeline: {
    events: any[];
  };
}

export interface InvestigationListResponse {
  investigations: InvestigationSummary[];
}
