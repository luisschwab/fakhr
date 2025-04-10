# Fakhr

Fakhr (**فخر** - meaning _pride_ in arabic) is a Bitcoin vanity address generator.

# Usage

```
$ cargo install --path .

$ fakhr --help
Fakhr (فخر - meaning pride in arabic) is a Bitcoin vanity address generator.

Usage: fakhr --prefix <PREFIX> --suffix <SUFFIX>

Options:
  -p, --prefix <PREFIX>  [possible values: 1, 3, bc1q, bc1p, m, n, 2, tb1q, tb1p]
  -s, --suffix <SUFFIX>  
  -h, --help             Print help
  -V, --version          Print version


$ fakhr -p 1 -s BTC
1HFZSHDvm1pg12eDpHnubiVxjsVHWmCiu1
14TEQZmXVvQNgS7JT341fGoPkk5iBz1874
1Fd28Db3xcJzgvoJRFk7rBMWjxBypp8xwB
1EdHqH7yU1XaoE9oenjvnBTUtKRTjjE6tS
1L4pZhrua6mN5Aj7Gr6KmEyZ92REnEiY2Y
17qxa84AdtGXDy9WtTnvaFDABKwRhLeLZ7
...
1LFLEqJ5ubG5rbbYfjdZjxjbgAii14uQHf
1LPy7Wf2a7Rke4amc2cYVt5tzEoMtch7MZ
19ARJpRqGeU7SDx1UMWuyqY4tcQrn3hVJr
14Vh33c52ij2BB5kZf4mNW2L4LL91dbVhv
1AQNvMZZfjkGUapTwyRKJBUgQt2Wjp18mD
1BTCL2NyzEzXpNQFb75qFHTAt2zGSPPLXT

Found 1BTCL2NyzEzXpNQFb75qFHTAt2zGSPPLXT in 64465 iterations
Secret Key: a71899001a5547cd20faf6746187f510f0dfb6e190e72479738ba29293c32b19
WIF: L2pXLS4vEus9bCLKDg1YXujuE7oHsQBH3xv6LGx2QY8jHEjh5BEb
```

