# boolean-evaluator

Evaluate boolean expressions and produce truth tables. The expressions are parsed using
Dijkstra's Shunting Yard algorithm and then evaluated in postfix form.

Supports single letter variables, Logical OR/AND/IMP/NOT and unlimited nested expressions

## Example
Input file:
```
(E & ~R) -> J
~J & E
R -> (J v M)
~E & M
```

The last expression represents the conclusion, while the previous rows represent the
premises.

Produce a truth table:
```
./boolean-evaluator input.txt
```
```
           │ E     │ J     │ M     │ R     │ expr1 │ expr2 │ expr3 │ expr4 │
│  1 │ OK  │ true  │ true  │ true  │ true  │ true  │ false │ true  │ false │
│  2 │ OK  │ true  │ true  │ true  │ false │ true  │ false │ true  │ false │
│  3 │ OK  │ true  │ true  │ false │ true  │ true  │ false │ true  │ false │
│  4 │ OK  │ true  │ true  │ false │ false │ true  │ false │ true  │ false │
│  5 │ ERR │ true  │ false │ true  │ true  │ true  │ true  │ true  │ false │
│  6 │ OK  │ true  │ false │ true  │ false │ false │ true  │ true  │ false │
│  7 │ OK  │ true  │ false │ false │ true  │ true  │ true  │ false │ false │
│  8 │ OK  │ true  │ false │ false │ false │ false │ true  │ true  │ false │
│  9 │ OK  │ false │ true  │ true  │ true  │ true  │ false │ true  │ true  │
│ 10 │ OK  │ false │ true  │ true  │ false │ true  │ false │ true  │ true  │
│ 11 │ OK  │ false │ true  │ false │ true  │ true  │ false │ true  │ false │
│ 12 │ OK  │ false │ true  │ false │ false │ true  │ false │ true  │ false │
│ 13 │ OK  │ false │ false │ true  │ true  │ true  │ false │ true  │ true  │
│ 14 │ OK  │ false │ false │ true  │ false │ true  │ false │ true  │ true  │
│ 15 │ OK  │ false │ false │ false │ true  │ true  │ false │ false │ false │
│ 16 │ OK  │ false │ false │ false │ false │ true  │ false │ true  │ false │
```
