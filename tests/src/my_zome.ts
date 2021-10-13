
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../utils';

export default (orchestrator: Orchestrator<any>) => 
  orchestrator.registerScenario("my_zome tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellNick.includes('/my-dna.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellNick.includes('/my-dna.dna')) as Cell;

    const postContents = "My Post";

    // Alice creates a post
    const postHash = await alice.call(
        "my_zome",
        "create_post",
        postContents
    );
    t.ok(postHash);

    await sleep(50);
    
    // Bob gets the created post
    const post = await bob.call("my_zome", "get_post", postHash);
    t.equal(post, postContents);
});
