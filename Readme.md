# Organization smart contract

 Simple smart contract for scoring contributors in an organization using ink!, and swanky suit. 


 ## How to use it in a local environment

In a first terminal
```
swanky node start
```

In a second terminal we could deploy the smart contract
```
 swanky contract compile organization
 swanky contract deploy organization --account eve
```

If everything is ok; we can interact with it. Adding a new contributor to the organization  with specific score
```
swanky contract tx organization addcontributor --account alice -p 2
```

We should get the information associated to specific account with

```
swanky contract query organization getcontributor --account alice
```
