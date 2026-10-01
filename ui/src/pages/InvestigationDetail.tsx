import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { getInvestigationDetail } from "../api/endpoints";
import { InvestigationResult } from "../types/investigation";
import { InvestigationGraph } from "../components/graph/InvestigationGraph";

export function InvestigationDetail() {
  const { id } = useParams<{id: string}>();
  const [inv, setInv] = useState<InvestigationResult | null>(null);
  const [tab, setTab] = useState("overview");

  useEffect(() => {
    if (id) getInvestigationDetail(id).then(setInv).catch(console.error);
  }, [id]);

  if (!inv) return <div className="p-4 text-gray-400">Loading investigation...</div>;

  return (
    <div className="h-full flex flex-col space-y-4">
      <div className="bg-gray-900 border border-gray-800 p-6 rounded-lg flex justify-between items-start">
        <div>
          <h2 className="text-xl font-bold font-mono">{inv.investigation_id}</h2>
          <p className="text-gray-400 text-sm mt-1">Endpoint: {inv.endpoint_id} • Target: {inv.target} • {inv.platform}</p>
        </div>
        <div className="text-right">
          <div className="text-3xl font-bold">{inv.risk_score.score}</div>
          <div className={`text-sm font-bold ${inv.risk_score.priority === 'HIGH' ? 'text-orange-500' : 'text-yellow-500'}`}>
            {inv.risk_score.priority} RISK
          </div>
        </div>
      </div>

      <div className="flex gap-4 border-b border-gray-800 pb-2">
        {['overview', 'graph', 'timeline'].map(t => (
          <button 
            key={t}
            onClick={() => setTab(t)}
            className={`px-4 py-2 capitalize font-medium rounded-t ${tab === t ? 'bg-gray-800 text-white' : 'text-gray-400 hover:text-white'}`}
          >
            {t}
          </button>
        ))}
      </div>

      <div className="flex-1 bg-gray-900 border border-gray-800 rounded-lg overflow-hidden relative">
        {tab === 'overview' && (
          <div className="p-6">
            <h3 className="text-lg font-bold mb-4">Investigation Details</h3>
            <pre className="text-xs text-gray-400 bg-gray-950 p-4 rounded">{JSON.stringify(inv, null, 2)}</pre>
          </div>
        )}
        {tab === 'graph' && (
          <InvestigationGraph nodes={inv.graph.nodes} edges={inv.graph.edges} />
        )}
        {tab === 'timeline' && (
          <div className="p-6 overflow-auto h-full">
            <div className="space-y-4 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-gray-700 before:to-transparent">
              {inv.timeline.events.map((ev, i) => (
                <div key={i} className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active">
                  <div className="flex items-center justify-center w-10 h-10 rounded-full border border-gray-700 bg-gray-800 text-gray-400 shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow">
                    <span className="text-[10px] font-bold">{ev.event_type.substring(0,2)}</span>
                  </div>
                  <div className="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-gray-800 p-4 rounded border border-gray-700">
                    <div className="flex items-center justify-between mb-1">
                      <div className="font-bold text-sm text-blue-400">{ev.event_type}</div>
                      <time className="text-xs text-gray-500 font-mono">{new Date(ev.timestamp).toLocaleTimeString()}</time>
                    </div>
                    <div className="text-sm text-gray-300">{ev.description}</div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
