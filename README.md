<p align="center">
  <img src="fakhr.png" width="50%" alt="Fakhr">
</p>

# Fakhr فخر

Fakhr (**فخر** - meaning _pride_ in arabic) is a vanity Bitcoin address and Nostr key generator.

# Installation

```
$ cargo install --path .
```

# Usage
```
$ fakhr --help
Fakhr (فخر - meaning pride in arabic) is a vanity Bitcoin address and Nostr key generator.

Usage: fakhr --prefix <PREFIX> --suffix <SUFFIX>

Options:
  -p, --prefix <PREFIX>  [possible values: 1, 3, bc1q, bc1p, m, n, 2, tb1q, tb1p, npub1]
  -s, --suffix <SUFFIX>
  -h, --help             Print help
  -V, --version          Print version

$ fakhr -p 1 -s Beer
1HFZSHDvm1pg12eDpHnubiVxjsVHWmCiu1
14TEQZmXVvQNgS7JT341fGoPkk5iBz1874
1Fd28Db3xcJzgvoJRFk7rBMWjxBypp8xwB
1EdHqH7yU1XaoE9oenjvnBTUtKRTjjE6tS
...
19hPDDoHpum3DRtBPjJPY4JHk2Gatzewq8
1GNzNLu5aEURShyoTHf5GA8AJVk3FFd6Le
16KQRD5EjfEsZB9zoJ1gKpNvZggd1LFpm6
1ArJxbP9rKUcso3ycgcVi5zPR3fi3QvAsA
1BeerxiA3utRA69sZxtHDaNqbewLWj9z3C

Found 1BeerxiA3utRA69sZxtHDaNqbewLWj9z3C in 00:00:55 and 7,253,641 iterations (131,884 iter/s)
SecretKey: 036e9c65a99cd0ecc6449505c5851e71f2748088feaa1133e4b119b16304c44b
WIF: KwLP8bTqyDxxVrpH8wRHtc5wg9F8zxzQW8XFkshcdR57xYGmzbzk

$ fakhr -p npub1 -s n0str
npub13cws3afpz7yd2zghcasml73skjlcywnh6z85y789dumu07tpsa9scrtqqr
npub1kkwfvpwz882n7lzlw7p2fgtw3afdq6tdy93a4ldvqssx7rs6xn5sr9wr2l
npub1ekkuaw42x8dnxj834hg9h74yjwhxxmfgmw0waguyhj72nl5gl5wse5gh0q
npub18tnyhs3cswwtudgh4xmpa7087cc4jex3n9wg9djve5wrn85azews50ptzq
...
npub1w6qy5s0vmdmx0hk7jhvcl9aqdfsvl5ulz4l4c5d038ml6xgxxg7qmt3lu9
npub1lxsyd05j2w9la0mnyzhcxeu3rkjgt6h0l2gpad97hrwrr0fq9d5qzd595l
npub1ph83zlrxv97xle49mh4nlk8v7gc9rg4e9twwfs6q22w4vg7yz3dqy3vf33
npub1wxhtkkl9egp93d2tjsgwt69aj5vrykr4apuh039p8eal07fdyx6qm32mma
npub1n0strz0wsmherzvl60etcak9lpjgslay620hznerzt2jwcf57yfql75j8l

Found npub1n0strz0wsmherzvl60etcak9lpjgslay620hznerzt2jwcf57yfql75j8l in 00:07:05 and 54,012,252 iterations (127,087 iter/s)
SecretKey: nsec14tlymffz6ghunsrnma59fah7gsnurqn8pdewad3dk52d0a4pyq5qd6fxlc
```

