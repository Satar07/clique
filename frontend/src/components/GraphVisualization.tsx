import React, { useMemo, useCallback, useEffect, useState, useRef } from 'react';
import ForceGraph2D from 'react-force-graph-2d';
import { message } from 'antd';

interface GraphVisualizationProps {
  edges: [number, number][];
  maxClique: number[];
}

interface Node {
  id: number;
  val: number;
  color: string;
  label: string;
  x: number;
  y: number;
  cluster?: number;
}

interface Link {
  id: number;
  source: number;
  target: number;
  color: string;
  width: number;
}

const MAX_NODES = 1000;
const NODE_SIZE = 4;
const LINK_WIDTH = 0.8;
const CLUSTER_THRESHOLD = 50;

// 定义新的颜色方案
const COLORS = {
  node: {
    default: '#4a90e2',
    highlight: '#f5222d',
    text: '#ffffff',
    background: '#ffffff'
  },
  link: {
    default: '#e8e8e8',
    highlight: '#ff7875'
  }
};

const GraphVisualization: React.FC<GraphVisualizationProps> = ({ edges, maxClique }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });

  // 处理窗口大小变化
  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        const { width, height } = containerRef.current.getBoundingClientRect();
        setDimensions({ width, height });
      }
    };

    // 初始化尺寸
    updateDimensions();

    // 监听窗口大小变化
    window.addEventListener('resize', updateDimensions);

    // 清理函数
    return () => {
      window.removeEventListener('resize', updateDimensions);
    };
  }, []);

  const calculateClusters = useCallback((nodes: Node[], links: Link[]) => {
    const clusters: { [key: number]: number[] } = {};
    const visited = new Set<number>();

    const dfs = (nodeId: number, clusterId: number) => {
      visited.add(nodeId);
      if (!clusters[clusterId]) {
        clusters[clusterId] = [];
      }
      clusters[clusterId].push(nodeId);

      links.forEach(link => {
        if (link.source === nodeId && !visited.has(link.target)) {
          dfs(link.target, clusterId);
        }
        if (link.target === nodeId && !visited.has(link.source)) {
          dfs(link.source, clusterId);
        }
      });
    };

    let clusterId = 0;
    nodes.forEach(node => {
      if (!visited.has(node.id)) {
        dfs(node.id, clusterId);
        clusterId++;
      }
    });

    return clusters;
  }, []);

  const graphData = useMemo(() => {
    const uniqueNodes = Array.from(new Set(edges.flat()));

    if (uniqueNodes.length > MAX_NODES) {
      message.warning(`图形过大，仅显示前 ${MAX_NODES} 个节点`);
    }

    const limitedNodes = uniqueNodes.slice(0, MAX_NODES);
    const limitedEdges = edges.filter(([from, to]) =>
      limitedNodes.includes(from) && limitedNodes.includes(to)
    );

    const nodes: Node[] = limitedNodes.map(id => ({
      id,
      val: maxClique.includes(id) ? NODE_SIZE * 1.5 : NODE_SIZE,
      color: maxClique.includes(id) ? COLORS.node.highlight : COLORS.node.default,
      label: id.toString(),
      x: 0,
      y: 0,
    }));

    const links: Link[] = limitedEdges.map(([source, target], index) => ({
      id: index,
      source,
      target,
      color: maxClique.includes(source) && maxClique.includes(target) ? COLORS.link.highlight : COLORS.link.default,
      width: maxClique.includes(source) && maxClique.includes(target) ? LINK_WIDTH * 2 : LINK_WIDTH,
    }));

    const clusters = calculateClusters(nodes, links);

    nodes.forEach(node => {
      for (const [clusterId, clusterNodes] of Object.entries(clusters)) {
        if (clusterNodes.includes(node.id)) {
          node.cluster = parseInt(clusterId);
          break;
        }
      }
    });

    return { nodes, links };
  }, [edges, maxClique, calculateClusters]);

  return (
    <div
      ref={containerRef}
      style={{
        width: '100%',
        height: 'calc(100vh - 200px)',
        minHeight: '400px',
        border: '1px solid #f0f0f0',
        borderRadius: '8px',
        overflow: 'hidden',
        boxShadow: '0 2px 8px rgba(0, 0, 0, 0.06)'
      }}
    >
      <ForceGraph2D
        graphData={graphData}
        nodeLabel="label"
        nodeRelSize={NODE_SIZE}
        linkWidth={LINK_WIDTH}
        d3Force={[
          ['charge', null],
          ['center', null],
          ['collision', 10],
          ['link', 30],
        ]}
        d3VelocityDecay={0.4}
        width={dimensions.width}
        height={dimensions.height}
        backgroundColor={COLORS.node.background}
        onEngineStop={() => {
          if (containerRef.current) {
            const graph = containerRef.current.querySelector('canvas');
            if (graph) {
              graph.style.width = '100%';
              graph.style.height = '100%';
            }
          }
        }}
        nodeCanvasObject={(node: Node, ctx: CanvasRenderingContext2D, globalScale: number) => {
          if (globalScale > 0.5) {
            const label = node.label;
            const fontSize = 9 / globalScale;
            ctx.font = `${fontSize}px -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial`;
            ctx.fillStyle = node.color;
            ctx.beginPath();
            ctx.arc(node.x, node.y, node.val, 0, 2 * Math.PI);
            ctx.fill();
            ctx.fillStyle = COLORS.node.text;
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText(label, node.x, node.y);
          } else {
            ctx.fillStyle = node.color;
            ctx.beginPath();
            ctx.arc(node.x, node.y, node.val, 0, 2 * Math.PI);
            ctx.fill();
          }
        }}
      />
    </div>
  );
};

export default GraphVisualization; 