import { Orchestrator } from "@holochain/tryorama";

import elo from "./elo";
let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
elo(orchestrator);
orchestrator.run();
