import { apiClient } from "./client";
import { EndpointListResponse } from "../types/endpoint";
import { InvestigationListResponse, InvestigationResult } from "../types/investigation";

export const getEndpoints = () => apiClient.get<EndpointListResponse>("/api/v1/endpoints");
export const getInvestigations = () => apiClient.get<InvestigationListResponse>("/api/v1/investigations");
export const getInvestigationDetail = (id: string) => apiClient.get<InvestigationResult>(`/api/v1/investigations/${id}`);
export const checkHealth = () => apiClient.get<{status: string}>("/api/v1/health");
