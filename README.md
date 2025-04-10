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

$ fakhr -p 1 -s Beer
1HFZSHDvm1pg12eDpHnubiVxjsVHWmCiu1
14TEQZmXVvQNgS7JT341fGoPkk5iBz1874
1Fd28Db3xcJzgvoJRFk7rBMWjxBypp8xwB
1EdHqH7yU1XaoE9oenjvnBTUtKRTjjE6tS
1L4pZhrua6mN5Aj7Gr6KmEyZ92REnEiY2Y
17qxa84AdtGXDy9WtTnvaFDABKwRhLeLZ7
1LFLEqJ5ubG5rbbYfjdZjxjbgAii14uQHf
1LPy7Wf2a7Rke4amc2cYVt5tzEoMtch7MZ
1NCzwUhfbGUFV4wTWhnFBrcT6LVCV2X4Wf
19hPDDoHpum3DRtBPjJPY4JHk2Gatzewq8
...
1GNzNLu5aEURShyoTHf5GA8AJVk3FFd6Le
1687HEwaGaEQQ4ByVwFJYHasNaLv8kkV8m
19XsGgBz6WVfjxp3b7w3SXJFAAG5cJPs1q
15bwWfbqXhwFYLYSgToAPhyb8SVN8omFNa
1633xLzwm1zrTZF7fsbiSQ23TPBjGaJAmG
1L1bse5Fp5YYycsyiW7kdzFgnJceHE6Kzs
1HRCofa9tv4kTCUfw3PnRoMSRhikcXQjy9
1BeerR9cW4WopD5oMe9yqZmwbvwAY6vVY5

Found 1BeerR9cW4WopD5oMe9yqZmwbvwAY6vVY5 in 2689825 iterations and 00:01:16 (35392 iters / s)
Secret Key: 8a5eb510b33f75852cc22d8cb120ffbaa6ca42ac95d3427ce45062204f60b190
WIF: L1rgdB1gnPp2u4y9qzEuCyFSZtgST1SHsxyTa1DQ8YbNZfy367Jd
```

