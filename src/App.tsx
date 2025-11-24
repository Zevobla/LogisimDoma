import { useState } from "react";
import "./App.css";
import { NodeEditor } from "flume";
import config from "./config";

function App() {
  const [nodes, setNodes] = useState({});
  return (
    <main className="container">
      <NodeEditor
        portTypes={config.portTypes}
        nodeTypes={config.nodeTypes}
        nodes={nodes}
        onChange={setNodes}
      />
    </main>
  );
}

export default App;
