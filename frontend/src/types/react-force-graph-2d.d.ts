declare module 'react-force-graph-2d' {
  import { ComponentType } from 'react';

  interface GraphData {
    nodes: Array<{
      id: number;
      val?: number;
      color?: string;
      label?: string;
      x?: number;
      y?: number;
      cluster?: number;
    }>;
    links: Array<{
      id: number;
      source: number;
      target: number;
      color?: string;
      width?: number;
    }>;
  }

  type D3Force = [string, any] | [string, null];

  interface ForceGraph2DProps {
    graphData: GraphData;
    nodeLabel?: string;
    nodeRelSize?: number;
    linkWidth?: number;
    linkDirectionalParticles?: number;
    linkDirectionalParticleSpeed?: number;
    d3Force?: D3Force | D3Force[];
    d3VelocityDecay?: number;
    width?: number;
    height?: number;
    backgroundColor?: string;
    nodeCanvasObject?: (node: any, ctx: CanvasRenderingContext2D, globalScale: number) => void;
    onEngineStop?: () => void;
  }

  const ForceGraph2D: ComponentType<ForceGraph2DProps>;
  export default ForceGraph2D;
} 