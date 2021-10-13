import { Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const eloDna = path.join(
  __dirname,
  "../../example/workdir/example-elo.dna"
);

export const config = Config.gen();

export const installation: InstallAgentsHapps = [
  // one agent
  [
    [
      eloDna, // contains this dna
    ],
  ],
];

export const sleep = (ms: number) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));
