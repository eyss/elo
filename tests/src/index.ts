
import { Orchestrator } from "@holochain/tryorama";

import my_zome from './my-dna/my_zome';
import new_zome_2 from './my-dna/new_zome_2';
import new_zome_4 from './new-dna-2/new_zome_4';
import new_zome_5 from './new-dna-2/new_zome_5';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
my_zome(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
new_zome_2(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
new_zome_4(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
new_zome_5(orchestrator);
orchestrator.run();



