We've all been locked out of our word-combination locks at some point, haven't we? This simply iterates through every combination on that lock and checks them against some local dictionary to see if there's a match. Order doesn't matter, so this will even work on scrambled words.

Usage:

```
> ./cracker --dict=/usr/share/dict/my-local-dict.txt ABC ABC TTT
		Combination: ACT
			Match: act
		Combination: CAT
			Match: cat
```

NOTE: please don't use this to steal things
