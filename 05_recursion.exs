# ============================================================
# Lesson 5: Recursion
# ============================================================
# FP Concepts: Recursion (replacing loops), Tail-call
#              Optimization (TCO), Accumulators
#
# In FP, there are no mutable loop counters. Instead, we
# use RECURSION: a function that calls itself with modified
# arguments. Combined with pattern matching (Lesson 2) and
# multi-clause functions, recursion becomes elegant and safe.
#
# The BEAM VM (Erlang/Elixir runtime) supports tail-call
# optimization: if the last thing a function does is call
# itself, no extra stack frame is created. This means
# recursive functions can run on infinite data without
# stack overflow.
# ============================================================

IO.puts("=== Lesson 5: Recursion ===\n")

# ----------------------------------------------------------
# 5.1 Basic Recursion: List Sum
# ----------------------------------------------------------
# Every recursive function needs:
#   1. Base case: when to STOP (prevents infinite recursion)
#   2. Recursive case: break problem into smaller pieces
IO.puts("5.1 Basic Recursion: List Sum")

defmodule MyList do
  # Base case: sum of empty list is 0
  def sum([]), do: 0
  # Recursive case: head + sum of tail
  def sum([head | tail]), do: head + sum(tail)
end

IO.puts("sum([1,2,3,4,5]) = #{MyList.sum([1, 2, 3, 4, 5])}")
IO.puts("sum([]) = #{MyList.sum([])}")

# Trace the execution:
# sum([1,2,3]) = 1 + sum([2,3])
#              = 1 + (2 + sum([3]))
#              = 1 + (2 + (3 + sum([])))
#              = 1 + (2 + (3 + 0))
#              = 6
IO.puts("Trace: 1 + (2 + (3 + (4 + (5 + 0)))) = 15")
IO.puts("")

# ----------------------------------------------------------
# 5.2 Tail Recursion with Accumulators
# ----------------------------------------------------------
# The basic sum above builds up stack frames.
# Tail recursion avoids this by carrying the result
# in an "accumulator" parameter.
IO.puts("5.2 Tail Recursion with Accumulators")

defmodule TailList do
  # Public API with nice interface
  def sum(list), do: do_sum(list, 0)

  # Private tail-recursive implementation
  # Base case: return accumulated result
  defp do_sum([], acc), do: acc
  # Recursive case: add head to accumulator, recurse on tail
  # This is TAIL recursive because the last operation is do_sum()
  defp do_sum([head | tail], acc), do: do_sum(tail, acc + head)
end

IO.puts("tail_sum([1,2,3,4,5]) = #{TailList.sum([1, 2, 3, 4, 5])}")

# Trace the execution (no stack buildup!):
# do_sum([1,2,3], 0)
# do_sum([2,3], 1)     <- acc = 0+1
# do_sum([3], 3)        <- acc = 1+2
# do_sum([], 6)         <- acc = 3+3
# 6                      <- return acc
IO.puts("Trace: do_sum([1,2,3,4,5], 0) -> do_sum([2,3,4,5], 1) -> ... -> do_sum([], 15) -> 15")
IO.puts("")

# ----------------------------------------------------------
# 5.3 Factorial
# ----------------------------------------------------------
# Classic recursion example: n! = n * (n-1) * ... * 1
IO.puts("5.3 Factorial")

defmodule Math do
  # Simple recursive version
  def factorial(0), do: 1
  def factorial(n) when n > 0, do: n * factorial(n - 1)

  # Tail-recursive version with accumulator
  def factorial_tail(n), do: do_factorial(n, 1)

  defp do_factorial(0, acc), do: acc
  defp do_factorial(n, acc) when n > 0, do: do_factorial(n - 1, n * acc)
end

for n <- [0, 1, 5, 10] do
  IO.puts("  #{n}! = #{Math.factorial(n)}")
end
IO.puts("  (tail-recursive) 10! = #{Math.factorial_tail(10)}")
IO.puts("")

# ----------------------------------------------------------
# 5.4 Implement Your Own `map`
# ----------------------------------------------------------
# Understanding recursion deeply by reimplementing Enum.map.
IO.puts("5.4 Custom map Implementation")

defmodule MyEnum do
  # map: apply function to each element, return new list
  def map([], _func), do: []
  def map([head | tail], func), do: [func.(head) | map(tail, func)]

  # Tail-recursive version (accumulates in reverse, then reverses)
  def map_tail(list, func), do: do_map(list, func, [])

  defp do_map([], _func, acc), do: Enum.reverse(acc)
  defp do_map([head | tail], func, acc), do: do_map(tail, func, [func.(head) | acc])

  # filter: keep elements where func returns true
  def filter([], _func), do: []
  def filter([head | tail], func) do
    if func.(head) do
      [head | filter(tail, func)]
    else
      filter(tail, func)
    end
  end

  # length: count elements
  def length([]), do: 0
  def length([_ | tail]), do: 1 + MyEnum.length(tail)
end

IO.puts("map([1,2,3], &1*2) = #{inspect(MyEnum.map([1, 2, 3], &(&1 * 2)))}")
IO.puts("map_tail([1,2,3], &1*2) = #{inspect(MyEnum.map_tail([1, 2, 3], &(&1 * 2)))}")
IO.puts("filter([1..6], even?) = #{inspect(MyEnum.filter([1, 2, 3, 4, 5, 6], &(rem(&1, 2) == 0)))}")
IO.puts("length([a, b, c]) = #{MyEnum.length([:a, :b, :c])}")
IO.puts("")

# ----------------------------------------------------------
# 5.5 FizzBuzz with Recursion and `cond`
# ----------------------------------------------------------
# FizzBuzz using recursion instead of a for loop.
# `cond` evaluates conditions top-to-bottom (like if/else if).
IO.puts("5.5 FizzBuzz (Recursive)")

defmodule FizzBuzz do
  def run(n), do: do_fizzbuzz(1, n, [])

  defp do_fizzbuzz(current, max, acc) when current > max do
    Enum.reverse(acc)
  end

  defp do_fizzbuzz(current, max, acc) do
    result = cond do
      rem(current, 15) == 0 -> "FizzBuzz"
      rem(current, 3) == 0 -> "Fizz"
      rem(current, 5) == 0 -> "Buzz"
      true -> Integer.to_string(current)
    end

    do_fizzbuzz(current + 1, max, [result | acc])
  end
end

FizzBuzz.run(20)
|> Enum.join(", ")
|> IO.puts()

IO.puts("")

# ----------------------------------------------------------
# 5.6 Tree Traversal (Bonus: Recursion on nested data)
# ----------------------------------------------------------
# Recursion shines on tree-structured data.
IO.puts("5.6 Tree Traversal")

defmodule Tree do
  # A tree node: {value, left, right} or :leaf for empty
  def sum(:leaf), do: 0
  def sum({value, left, right}) do
    value + sum(left) + sum(right)
  end

  def depth(:leaf), do: 0
  def depth({_value, left, right}) do
    1 + max(depth(left), depth(right))
  end
end

#       10
#      /  \
#     5    15
#    / \     \
#   3   7    20
tree = {10,
  {5,
    {3, :leaf, :leaf},
    {7, :leaf, :leaf}},
  {15,
    :leaf,
    {20, :leaf, :leaf}}}

IO.puts("Tree sum: #{Tree.sum(tree)}")
IO.puts("Tree depth: #{Tree.depth(tree)}")
IO.puts("")

# ----------------------------------------------------------
# 5.7 Comparing Loop vs. Recursion
# ----------------------------------------------------------
IO.puts("5.7 Loop vs. Recursion Comparison")
IO.puts("""
  Imperative (JavaScript):     Functional (Elixir):
  ─────────────────────────    ─────────────────────────
  let sum = 0;                 def sum([]), do: 0
  for (let x of list) {       def sum([h|t]), do: h + sum(t)
    sum += x;
  }

  Imperative mutates `sum`.    Functional returns NEW values.
  Uses a loop counter.         Uses pattern matching + recursion.
  State changes over time.     No state, no mutation.
""")

IO.puts("=== Key Takeaways ===")
IO.puts("1. Recursion = base case + recursive case")
IO.puts("2. Tail recursion uses an accumulator to avoid stack overflow")
IO.puts("3. Pattern matching on [head | tail] naturally decomposes lists")
IO.puts("4. `cond` is a multi-way conditional (like if/else if)")
IO.puts("5. Recursion naturally handles tree-structured data")
IO.puts("6. The BEAM VM optimizes tail calls - no stack overflow")
