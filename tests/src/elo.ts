import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from "./utils";
import { serializeHash } from "@holochain-open-dev/core-types";

export default (orchestrator: Orchestrator<any>) =>
  orchestrator.registerScenario("my_zome tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player, carol_player]: Player[] = await s.players([
      config,
      config,
      config,
    ]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);
    const [[carol_happ]] = await carol_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player, carol_player]);

    const alice = alice_happ.cells.find((cell) =>
      cell.cellNick.includes("/example-elo.dna")
    ) as Cell;
    const bob = bob_happ.cells.find((cell) =>
      cell.cellNick.includes("/example-elo.dna")
    ) as Cell;
    const carol = carol_happ.cells.find((cell) =>
      cell.cellNick.includes("/example-elo.dna")
    ) as Cell;

    const aliceKey = serializeHash(alice.cellId[1]);
    const bobKey = serializeHash(bob.cellId[1]);
    const carolKey = serializeHash(carol.cellId[1]);

    await sleep(1000);
    let { type, game_result_hash } = await alice.call("elo", "publish_result", [
      bobKey,
      1.0,
    ]);
    t.equal(type, "Published");
    await alice.call("elo", "link_my_game_results", [game_result_hash]);

    let outcome = await bob.call("elo", "publish_result", [aliceKey, 0.0]);
    t.equal(outcome.type, "OutdatedLastGameResult");

    await sleep(500);

    let gameResults = await bob.call("elo", "get_game_results_for_agents", [
      aliceKey,
      bobKey,
    ]);

    let aliceGameResult = gameResults[aliceKey][0];
    let bobGameResult = gameResults[bobKey][0];
    t.deepEqual(aliceGameResult[1].player_a, {
      player_address: aliceKey,
      current_elo: 1016,
      previous_game_result: null,
    });
    t.deepEqual(aliceGameResult[1].player_b, {
      player_address: bobKey,
      current_elo: 984,
      previous_game_result: null,
    });
    t.equal(aliceGameResult[1].score_player_a, 1);
    t.deepEqual(aliceGameResult[1], bobGameResult[1]);

    let elos = await bob.call("elo", "get_elo_rating_for_agents", [
      aliceKey,
      bobKey,
    ]);
    t.deepEqual(elos, { [aliceKey]: 1016, [bobKey]: 984 });

    let previousAliceGameResultHash = serializeHash(aliceGameResult[0].hash);
    let previousBobGameResultHash = serializeHash(bobGameResult[0].hash);

    await sleep(1000);

    outcome = await bob.call("elo", "publish_result", [aliceKey, 0.0]);
    t.equal(outcome.type, "Published");
    game_result_hash = outcome.game_result_hash;
    await bob.call("elo", "link_my_game_results", [game_result_hash]);

    await sleep(500);

    gameResults = await bob.call("elo", "get_game_results_for_agents", [
      aliceKey,
      bobKey,
    ]);
    aliceGameResult = gameResults[aliceKey][1];
    bobGameResult = gameResults[bobKey][1];
    t.deepEqual(aliceGameResult[1].player_b, {
      player_address: aliceKey,
      current_elo: 1030,
      previous_game_result: previousAliceGameResultHash,
    });
    t.deepEqual(aliceGameResult[1].player_a, {
      player_address: bobKey,
      current_elo: 970,
      previous_game_result: previousBobGameResultHash,
    });
    t.equal(aliceGameResult[1].score_player_a, 0);
    t.deepEqual(aliceGameResult[1], bobGameResult[1]);

    elos = await bob.call("elo", "get_elo_rating_for_agents", [
      aliceKey,
      bobKey,
    ]);
    t.equal(elos[aliceKey], 1030);
    t.equal(elos[bobKey], 970);

    await carol_player.shutdown();

    try {
      outcome = await bob.call("elo", "publish_result", [carolKey, 0.0]);
      t.ok(false);
    } catch (e) {
      t.ok(true);
    }

    await bob.call("elo", "publish_game_result_and_flag", [carolKey, 1.0]);

    elos = await bob.call("elo", "get_elo_rating_for_agents", [
      carolKey,
      bobKey,
    ]);
    t.equal(elos[carolKey], 1000);
    t.equal(elos[bobKey], 987);
    gameResults = await bob.call("elo", "get_game_results_for_agents", [
      carolKey,
    ]);
    t.equal(gameResults[carolKey].length, 0);

    // When carol awakes, they resolve their flagged result
    await carol_player.startup({});

    await sleep(30000);

    // TODO: fix error handling
    await carol.call(
      "elo",
      "scheduled_try_resolve_unpublished_game_results",
      null
    );
    await sleep(500);
    gameResults = await bob.call("elo", "get_game_results_for_agents", [
      carolKey,
    ]);
    t.equal(gameResults[carolKey].length, 1);

    elos = await bob.call("elo", "get_elo_rating_for_agents", [carolKey]);
    t.equal(elos[carolKey], 983);
  });
