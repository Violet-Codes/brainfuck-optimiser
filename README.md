# brainfuck-optimiser
A brainfuck parser with a custom optimised bytecode

## the bytecode
A block of the bytecode will either be:
* `Ask` or `Put` for `IO`.
* `AtomicEffect` which is a section of parallel assignments.
* `Loop`, which will contain more bytecode.

Specifically `AtomicEffect` contains a hashmap of relatively indexed registers to `ProcExpr` (the expression type that I use) and an offset that the action applied to the head upon completion.

An expression is either:
* `Lit` or `Reg` for supporting literals and relative registers.
* 'And' or `Mul` for algebraic combinations.
* 'Into' for calculating the number of times a loop will run.

## the optimisations
Here is a concise list of the optimisations I apply, I do warn you though, the code that does this is not nearly as clean and concise as its description.

### grouping
Two `AtomicEffect`'s can be grouped together by first shifting the registers of the latter by the offset of the former,
and then replacing all `Reg`'s present in the former to the expressions they would resolve to.

For example:
```
block {
	~#0 = (1 + ~#0);
} (move 1)
```
...grouped with:
```
block {
	~#0 = (~#0 + 255);
} (move -1)
```
...results in this:
```
block {
	~#1 = (255 + ~#1);
	~#0 = (~#0 + 1);
} (move 0)
```

### expression-reduction
Expressions are normalised into multinomial-expressions where the symbols present are expressions that cannot be factored further with multinomials:

These are `Reg` (as the value of the register is not known at optimisation time) and `Into` as it behaves too delicately.

As the multinomial expressions are combined they converge to the most reduced form.

### loop-effect solving
Loops that have the following properties can have their effects calculated upfront:
* The bytecode can be reduced to a single `AtomicEffect`.
* This `AtomicEffect` has no net movement on the head.
* The amount subtracted from the starting register `~#0` is not dependent on anything in the loop.
* The amount added to each register in the loop is not dependent on anything in the loop.

The optimised loop is then an `AtomicEffect` such that:
* `~#0` is set to `0`.
* Any register is set to itself plus amount added each loop scaled by how many times what would be subtracted from `~#0` would fit `Into` `~#0`.
