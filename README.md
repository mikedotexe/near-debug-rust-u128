There seems to be an issue with u128.
Note the last three numbers change from 700 to 800 for no apparent reason.
This doesn't happen in the tests. (`./test.sh`)

Try:
`near view debug.mike.testnet small_u '{"val": 1906293427246306700}'`

and

`near view debug.mike.testnet big_u '{"val": "1906293427246306700"}'`