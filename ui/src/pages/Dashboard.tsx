import React, { useEffect, useState } from "react";
import { getEndpoints, getInvestigations } from "../api/endpoints";
import { ShieldAlert, Activity, Monitor } from "lucide-react";

export function Dashboard() {
  const [stats, setStats] = useState({ epOnline: 0, epTotal: 0, invTotal: 0, highFinds: 0 });

  useEffect(() => {
    Promise.all([getEndpoints(), getInvestigations()]).then(([epRes, invRes]) => {
      const eps = epRes.endpoints || [];
      const invs = invRes.investigations || [];
      let highs = 0;
      invs.forEach(i => {
        if (i.priority === "HIGH" || i.priority === "CRITICAL") highs += i.finding_count;
      });

      setStats({
        epTotal: eps.length,
        epOnline: eps.filter(e => e.status === "ONLINE").length,
        invTotal: invs.length,
        highFinds: highs
      });
    }).catch(e => console.error(e));
  }, []);

  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">Dashboard</h2>
      <div className="grid grid-cols-3 gap-6">
        <div className="bg-gray-900 border border-gray-800 p-6 rounded-lg">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-gray-400 font-medium">ENDPOINTS</h3>
            <Monitor className="text-blue-500" />
          </div>
          <p className="text-3xl font-bold">{stats.epTotal} total</p>
          <p className="text-sm text-green-500 mt-2">{stats.epOnline} online</p>
        </div>
        <div className="bg-gray-900 border border-gray-800 p-6 rounded-lg">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-gray-400 font-medium">INVESTIGATIONS</h3>
            <Activity className="text-purple-500" />
          </div>
          <p className="text-3xl font-bold">{stats.invTotal} total</p>
        </div>
        <div className="bg-gray-900 border border-gray-800 p-6 rounded-lg">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-gray-400 font-medium">HIGH FINDINGS</h3>
            <ShieldAlert className="text-red-500" />
          </div>
          <p className="text-3xl font-bold text-red-500">{stats.highFinds}</p>
        </div>
      </div>
    </div>
  );
}
