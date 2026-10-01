import React, { useEffect, useState } from "react";
import { getInvestigations } from "../api/endpoints";
import { InvestigationSummary } from "../types/investigation";
import { Link } from "react-router-dom";

export function Investigations() {
  const [invs, setInvs] = useState<InvestigationSummary[]>([]);

  useEffect(() => {
    getInvestigations().then(res => setInvs(res.investigations || []));
  }, []);

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">Investigations</h2>
      <div className="bg-gray-900 border border-gray-800 rounded-lg overflow-hidden">
        <table className="w-full text-left text-sm">
          <thead className="bg-gray-800 text-gray-400">
            <tr>
              <th className="p-4">ID</th>
              <th className="p-4">Platform</th>
              <th className="p-4">Start Time</th>
              <th className="p-4">Findings</th>
              <th className="p-4">Risk Score</th>
              <th className="p-4">Priority</th>
              <th className="p-4">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-800">
            {invs.map(inv => (
              <tr key={inv.id} className="hover:bg-gray-800/50">
                <td className="p-4 font-mono text-xs">{inv.id}</td>
                <td className="p-4">{inv.platform}</td>
                <td className="p-4 text-gray-400">{new Date(inv.start_time).toLocaleString()}</td>
                <td className="p-4">{inv.finding_count}</td>
                <td className="p-4">{inv.risk_score}</td>
                <td className="p-4">
                  <span className={`px-2 py-1 text-xs rounded-full font-bold ${
                    inv.priority === 'CRITICAL' ? 'bg-red-500/20 text-red-500' :
                    inv.priority === 'HIGH' ? 'bg-orange-500/20 text-orange-500' :
                    inv.priority === 'MEDIUM' ? 'bg-yellow-500/20 text-yellow-500' :
                    'bg-blue-500/20 text-blue-500'
                  }`}>
                    {inv.priority}
                  </span>
                </td>
                <td className="p-4">
                  <Link to={`/investigations/${inv.id}`} className="text-blue-500 hover:underline">Inspect</Link>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
