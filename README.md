# VIP Score

## InstantiateMsg

Initialize VIP score

```json
{
  "allow_list": ["init1...", "init1..."] // allow address list that can execute contract msg
}
```

## ExecuteMsg

All execution messages can only be executed by addresses that are in the allow list.

### `prepare_stage`

Initialize Stage

```json
{
  "prepare_stage": {
    "stage": 123 // stage to initialize
  }
}
```

### `finalize_stage`

Finalize stage. Once stage finalized, can't change score of that stage anymore.

```json
{
  "finalize_stage": {
    "stage": 123 // stage to finalized
  }
}
```

### `increase_score`

Increase score.

```json
{
  "increase_score": {
    "stage": 123, // stage
    "addr": "init1...", // account address to increase
    "amount": 123 // increase score amount
  }
}
```

### `decrease_score`

Decrease score.

```json
{
  "decrease_score": {
    "stage": 123, // stage
    "addr": "init1...", // account address to decrease
    "amount": 123 // decrease score amount
  }
}
```

### `update_score`

Update the score by setting it to the given amount.

```json
{
  "update_score": {
    "stage": 123, // stage
    "addr": "init1...", // account address to set score
    "amount": 123 // score to set
  }
}
```

### `update_scores`

Update several scores at once.

```json
{
  "update_scores": {
    "stage": 123, // stage
    "scores": [
      ["init1...", 123],
      ["init1...", 123]
    ] // array of [account, amount]
  }
}
```

### `add_allow_list`

Add new address to allow list

```json
{
  "add_allow_list": {
    "addr": "init1..." // address to add
  }
}
```

### `remove_allow_list`

Remove address from allow list

```json
{
  "remove_allow_list": {
    "addr": "init1..." // address to remove
  }
}
```

## QueryMsg

### `get_score`

Get score of given address and stage

```json
{
  "get_score": {
    "addr": "init1...", // address
    "stage": 123 // stage
  }
}
```

Response type

```json
{
  "stage": 123,
  "addr": "init1...",
  "score": 123
}
```

### `get_scores`

Get scores of given stage

```json
{
  "get_scores": {
    "stage": 123, // stage
    "limit": 123, // amount of result
    "start_after": "init1..." // optional, where to begin fetching the next batch of results
  }
}
```

Response type

```json
{
  "scores": [
    {
      "stage": 123,
      "addr": "init1...",
      "score": 123
    },
    {
      "stage": 123,
      "addr": "init1...",
      "score": 123
    },
    ...
  ]
}
```

### `get_stage_info`

Get stage info

```json
{
  "get_stage_info": {
    "stage": 123 // stage
  }
}
```

Response type

```json
{
  "stage": 123,
  "total_score": 123,
  "is_finalized": false
}
```
