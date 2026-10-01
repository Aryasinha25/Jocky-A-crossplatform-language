import React, { useState } from "react";
import { ApiClient } from "../api/client";

export function Settings() {
  const [url, setUrl] = useState(localStorage.getItem("jocky_api_url") || "http://127.0.0.1:8080");
  const [token, setToken] = useState(localStorage.getItem("jocky_api_token") || "");
  const [demo, setDemo] = useState(localStorage.getItem("jocky_demo_mode") === "true");

  const save = () => {
    localStorage.setItem("jocky_api_url", url);
    localStorage.setItem("jocky_api_token", token);
    localStorage.setItem("jocky_demo_mode", demo ? "true" : "false");
    window.location.reload();
  };

  return (
    <div className="max-w-2xl space-y-6">
      <h2 className="text-2xl font-bold">Settings</h2>
      
      <div className="bg-gray-900 border border-gray-800 p-6 rounded-lg space-y-4">
        <h3 className="text-lg font-medium border-b border-gray-800 pb-2">Control Plane Connection</h3>
        
        <div>
          <label className="block text-sm text-gray-400 mb-1">Server URL</label>
          <input 
            type="text" 
            value={url} 
            onChange={e => setUrl(e.target.value)}
            className="w-full bg-gray-950 border border-gray-800 rounded p-2 text-white"
          />
        </div>
        
        <div>
          <label className="block text-sm text-gray-400 mb-1">API Token (Bearer)</label>
          <input 
            type="password" 
            value={token} 
            onChange={e => setToken(e.target.value)}
            className="w-full bg-gray-950 border border-gray-800 rounded p-2 text-white"
          />
        </div>

        <div className="flex items-center gap-2 pt-2">
          <input 
            type="checkbox" 
            checked={demo} 
            onChange={e => setDemo(e.target.checked)}
            id="demo"
          />
          <label htmlFor="demo" className="text-sm">Enable Demo Mode (Mock Synthetic Data)</label>
        </div>

        <button 
          onClick={save}
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded font-medium transition"
        >
          Save & Reconnect
        </button>
      </div>
    </div>
  );
}
