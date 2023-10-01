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
✔ Initialising OK
✔ Getting WASM OK
✔ Connecting to node OK
✔ Deploying OK
✔ Writing config OK
Contract deployed!
Contract address: 5Gh5Hh589zC9CvJF1ToaeN4srZBehZ8tDWBtSWvSE3jrn9rH
```

Adding a new contributor to the organization with specific score, for that we will pass as a parameter `-p 2`
```
swanky contract tx organization addcontributor -p 7 --account alice --add 5Gh5Hh589zC9CvJF1ToaeN4srZBehZ8tDWBtSWvSE3jrn9rH
✔ Initialising OK
✔ Getting metadata OK
Gas required: 3947587584 (proofSize: 629760)
Tx result:
{
  dispatchError: undefined,
  dispatchInfo: {
    weight: { refTime: '1,798,029,284', proofSize: '119,733' },
    class: 'Normal',
    paysFee: 'Yes'
  },
  events: [
    { phase: [Object], event: [Object], topics: [] },
    { phase: [Object], event: [Object], topics: [Array] },
    { phase: [Object], event: [Object], topics: [] },
    { phase: [Object], event: [Object], topics: [] },
    { phase: [Object], event: [Object], topics: [] }
  ],
  internalError: undefined,
  status: {
    Finalized: '0xc04030aa2fc9cac1f631c79e7b853cd17b1f48fd0c5683d13e5d225e358af8b7'
  }
}
```

We should get the information associated to specific account with

```
swanky contract query organization getcontributor -a alice --add 5Gh5Hh589zC9CvJF1ToaeN4srZBehZ8tDWBtSWvSE3jrn9rH 
✔ Initialising OK
✔ Getting metadata OK
Query result: {"ok":{"address":"5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw","score":7}}
```
