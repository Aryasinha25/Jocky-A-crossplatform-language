import React, { useEffect, useState } from "react";
import { getEndpoints } from "../api/endpoints";
import { Endpoint } from "../types/endpoint";
import { Link } from "react-router-dom";

export function Endpoints() {
  const [endpoints, setEndpoints] = useState<Endpoint[]>([]);

  useEffect(() => {
    getEndpoints().then(res => setEndpoints(res.endpoints || []));
  }, []);

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">Endpoints</h2>
      <div className="bg-gray-900 border border-gray-800 rounded-lg overflow-hidden">
        <table className="w-full text-left text-sm">
          <thead className="bg-gray-800 text-gray-400">
            <tr>
              <th className="p-4">Status</th>
              <th className="p-4">Name</th>
              <th className="p-4">Platform</th>
              <th className="p-4">Last Seen</th>
              <th className="p-4">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-800">
            {endpoints.map(ep => (
              <tr key={ep.id} className="hover:bg-gray-800/50">
                <td className="p-4">
                  <span className={`px-2 py-1 text-xs rounded-full ${ep.status === 'ONLINE' ? 'bg-green-500/20 text-green-500' : 'bg-gray-500/20 text-gray-500'}`}>
                    {ep.status}
                  </span>
                </td>
                <td className="p-4 font-medium">{ep.name}</td>
                <td className="p-4">{ep.platform}</td>
                <td className="p-4 text-gray-400">{new Date(ep.last_seen).toLocaleString()}</td>
                <td className="p-4">
                  <Link to={`/investigations?endpoint_id=${ep.id}`} className="text-blue-500 hover:underline">View Cases</Link>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
