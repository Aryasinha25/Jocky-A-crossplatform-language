import React, { useMemo } from 'react';
import ReactFlow, { Background, Controls, Node, Edge } from 'reactflow';
import 'reactflow/dist/style.css';

interface GraphProps {
  nodes: any[];
  edges: any[];
}

export function InvestigationGraph({ nodes, edges }: GraphProps) {
  const rfNodes: Node[] = useMemo(() => {
    return nodes.map((n, i) => ({
      id: n.id,
      position: { x: (i % 3) * 250, y: Math.floor(i / 3) * 150 },
      data: { label: `${n.evidence_type || 'FINDING'}\n${n.attributes?.name || n.title || ''}` },
      style: {
        background: n.type === 'finding' ? '#ef4444' : '#1f2937',
        color: '#fff',
        border: '1px solid #374151',
        borderRadius: '4px',
        padding: '10px',
        fontSize: '12px'
      }
    }));
  }, [nodes]);

  const rfEdges: Edge[] = useMemo(() => {
    return edges.map((e, i) => ({
      id: `e-${i}`,
      source: e.source,
      target: e.target,
      label: e.relationship,
      animated: true,
      style: { stroke: '#60a5fa' }
    }));
  }, [edges]);

  return (
    <div style={{ width: '100%', height: '100%' }} className="bg-gray-950">
      <ReactFlow nodes={rfNodes} edges={rfEdges} fitView>
        <Background color="#374151" gap={16} />
        <Controls />
      </ReactFlow>
    </div>
  );
}
