import React, { useState } from 'react';
import { Layout, Upload, Button, message, Spin } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import GraphVisualization from './components/GraphVisualization';
import './App.css';

const { Header, Content } = Layout;

const App: React.FC = () => {
  const [edges, setEdges] = useState<[number, number][]>([]);
  const [maxClique, setMaxClique] = useState<number[]>([]);
  const [loading, setLoading] = useState(false);

  const handleFileUpload = (file: File) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const text = e.target?.result as string;
      const lines = text.split('\n');
      const newEdges: [number, number][] = [];

      lines.forEach(line => {
        if (line.startsWith('e')) {
          const [, u, v] = line.split(' ').map(Number);
          newEdges.push([u, v]);
        }
      });

      setEdges(newEdges);
      findMaxClique(newEdges);
    };
    reader.readAsText(file);
    return false;
  };

  const findMaxClique = async (graphEdges: [number, number][]) => {
    setLoading(true);
    try {
      const response = await fetch('http://localhost:8080/api/find-max-clique', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ edges: graphEdges }),
      });

      if (!response.ok) {
        throw new Error('Network response was not ok');
      }

      const data = await response.json();
      setMaxClique(data.max_clique);
      message.success('最大团计算完成！');
    } catch (error) {
      message.error('计算失败，请重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Layout className="layout">
      <Header style={{ background: '#fff', padding: '0 20px' }}>
        <h1>最大团算法可视化</h1>
        <Upload
          beforeUpload={handleFileUpload}
          showUploadList={false}
          accept=".col,.clq"
        >
          <Button icon={<UploadOutlined />}>上传DIMACS文件(.col/.clq)</Button>
        </Upload>
      </Header>
      <Content style={{ padding: '20px' }}>
        <Spin spinning={loading}>
          <GraphVisualization edges={edges} maxClique={maxClique} />
        </Spin>
      </Content>
    </Layout>
  );
};

export default App;