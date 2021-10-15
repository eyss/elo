This is the result of the following executions:

1. Carol plays against Alice.
2. Alice plays against Bob.
3. Carol plays against Bob

```graphviz
digraph entries {
  node [
      shape ="record"
  ]

  GameResultAliceBob -> GameResultAliceCarol [style="dashed"]
  GameResultBobCarol -> {GameResultAliceBob, GameResultAliceCarol} [style="dashed"]
  Alice -> {GameResultAliceBob, GameResultAliceCarol}
  Bob -> {GameResultBobCarol, GameResultAliceBob}
  Carol -> {GameResultBobCarol, GameResultAliceCarol}
}
```

## Countersignature flow

```mermaid 
sequenceDiagram

participant Alice
participant Bob

Alice->>Bob: request_start_countersigning_session
Bob-->>Bob: validate_last_game_result_is_not_outdated
Bob-->>Bob: validate_game
Bob->>Alice: request_


```