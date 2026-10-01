import React from "react";
import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { Dashboard } from "./pages/Dashboard";
import { Endpoints } from "./pages/Endpoints";
import { Investigations } from "./pages/Investigations";
import { InvestigationDetail } from "./pages/InvestigationDetail";
import { Settings } from "./pages/Settings";
import { Shield, Activity, HardDrive, Search, Settings as SettingsIcon } from "lucide-react";

export default function App() {
  return (
    <BrowserRouter>
      <div className="flex h-screen bg-gray-950 text-gray-100 font-sans">
        <aside className="w-64 bg-gray-900 border-r border-gray-800 flex flex-col">
          <div className="p-4 border-b border-gray-800">
            <h1 className="text-xl font-bold flex items-center gap-2">
              <Shield className="text-blue-500" />
              JOCKY Analyst
            </h1>
          </div>
          <nav className="flex-1 p-4 space-y-2">
            <Link to="/" className="flex items-center gap-3 p-2 rounded hover:bg-gray-800 transition">
              <Activity size={18} /> Dashboard
            </Link>
            <Link to="/endpoints" className="flex items-center gap-3 p-2 rounded hover:bg-gray-800 transition">
              <HardDrive size={18} /> Endpoints
            </Link>
            <Link to="/investigations" className="flex items-center gap-3 p-2 rounded hover:bg-gray-800 transition">
              <Search size={18} /> Investigations
            </Link>
          </nav>
          <div className="p-4 border-t border-gray-800">
            <Link to="/settings" className="flex items-center gap-3 p-2 rounded hover:bg-gray-800 transition text-gray-400">
              <SettingsIcon size={18} /> Settings
            </Link>
          </div>
        </aside>
        
        <main className="flex-1 overflow-hidden flex flex-col">
          <header className="h-14 border-b border-gray-800 flex items-center justify-between px-6 bg-gray-900/50">
            <div className="text-sm text-gray-400">Read-Only Forensic View</div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-green-500"></div>
              <span className="text-sm font-medium text-green-500">Control Plane Connected</span>
            </div>
          </header>
          <div className="flex-1 overflow-auto p-6 relative">
            {localStorage.getItem("jocky_demo_mode") === "true" && (
              <div className="absolute top-0 right-0 bg-yellow-500/20 text-yellow-500 px-3 py-1 text-xs font-bold uppercase rounded-bl">
                Demo Data
              </div>
            )}
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/endpoints" element={<Endpoints />} />
              <Route path="/investigations" element={<Investigations />} />
              <Route path="/investigations/:id" element={<InvestigationDetail />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </div>
        </main>
      </div>
    </BrowserRouter>
  );
}
