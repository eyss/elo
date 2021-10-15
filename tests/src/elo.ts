import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from "./utils";
import { serializeHash } from "@holochain-open-dev/core-types";

export default (orchestrator: Orchestrator<any>) =>
  orchestrator.registerScenario("my_zome tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([
      config,
      config,
    ]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find((cell) =>
      cell.cellNick.includes("/example-elo.dna")
    ) as Cell;
    const bob = bob_happ.cells.find((cell) =>
      cell.cellNick.includes("/example-elo.dna")
    ) as Cell;

    const aliceKey = serializeHash(alice.cellId[1]);
    const bobKey = serializeHash(bob.cellId[1]);

    await alice.call("elo", "publish_result", [bobKey, 1.0]);

    await sleep(5000);

    let gameResults = await bob.call("elo", "get_games_results_for_agents", [
      aliceKey,
      bobKey,
    ]);
    let aliceGameResult = gameResults[aliceKey][0];
    let bobGameResult = gameResults[aliceKey][0];
    t.deepEqual(aliceGameResult[1].player_a, {
      player_address: aliceKey,
      current_elo: 1032,
      previous_game_result: null,
    });
    t.deepEqual(aliceGameResult[1].player_b, {
      player_address: bobKey,
      current_elo: 968,
      previous_game_result: null,
    });
    t.equal(aliceGameResult.score_a, 1.0);
    t.deepEqual(aliceGameResult[1], bobGameResult[1]);

    let elos = await bob.call("elo", "get_elo_rating_for_agents", [
      aliceKey,
      bobKey,
    ]);
    t.deepEqual(elos, { [aliceKey]: 1032, [bobKey]: 968 });

    let previousAliceGameResultHash = bobGameResult[0];
    let previousBobGameResultHash = bobGameResult[0];

    await bob.call("elo", "publish_result", [aliceKey, 0.0]);

    await sleep(2000);

    gameResults = await bob.call("elo", "get_games_results_for_agents", [
      aliceKey,
      bobKey,
    ]);
    aliceGameResult = gameResults[aliceKey][0];
    bobGameResult = gameResults[aliceKey][0];
    t.deepEqual(aliceGameResult.player_a, {
      player_address: aliceKey,
      current_elo: 1032,
      previous_game_result: previousAliceGameResultHash,
    });
    t.deepEqual(aliceGameResult.player_b, {
      player_address: bobKey,
      current_elo: 968,
      previous_game_result: previousBobGameResultHash,
    });
    t.equal(aliceGameResult.score_a, 1.0);
    t.deepEqual(aliceGameResult[1], bobGameResult[1]);

    elos = await bob.call("elo", "get_elo_rating_for_agents", [
      aliceKey,
      bobKey,
    ]);
    t.deepEqual(elos, { [aliceKey]: 1032, [bobKey]: 968 });
  });
