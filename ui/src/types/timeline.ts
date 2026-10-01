export interface TimelineEvent {
  timestamp: string;
  event_type: string;
  evidence_id: string;
  host: string;
  description: string;
}

export interface TimelineResponse {
  events: TimelineEvent[];
}
