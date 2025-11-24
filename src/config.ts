import { FlumeConfig, Colors, Controls } from "flume";

const config = new FlumeConfig();

config
  .addPortType({
    type: "boolean",
    name: "boolean",
    label: "Boolean",
    color: Colors.green,
    controls: [
      Controls.checkbox({
        name: "value",
        label: "Value",
      }),
    ],
  })
  .addPortType({
    type: "number",
    name: "number",
    label: "Number",
    color: Colors.red,
    controls: [
      Controls.number({
        name: "value",
        label: "Value",
      }),
    ],
  });

config.addNodeType({
  type: "input",
  label: "Input",
  description: "Boolean input signal",
  initialWidth: 150,
  inputs: [],
  outputs: (ports) => [ports.boolean()],
  root: true,
});

config.addNodeType({
  type: "output",
  label: "Output",
  description: "Boolean output signal",
  initialWidth: 150,
  inputs: (ports) => [ports.boolean()],
  outputs: [],
  root: true,
});

export default config;
