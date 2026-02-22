# ============================================================
# Lesson 3: Higher-Order Functions
# ============================================================
# FP Concepts: First-class Functions, Higher-order Functions
#
# Functions are "first-class citizens" in FP - they can be
# stored in variables, passed as arguments, and returned
# from other functions. Higher-order functions are functions
# that take or return other functions. This eliminates the
# need for imperative loops entirely.
# ============================================================

IO.puts("=== Lesson 3: Higher-Order Functions ===\n")

# ----------------------------------------------------------
# 3.1 Anonymous Functions
# ----------------------------------------------------------
# Create functions with `fn -> end`.
# Call them with a dot: `func.(args)`.
IO.puts("3.1 Anonymous Functions")

# Basic anonymous function
add = fn a, b -> a + b end
IO.puts("add.(3, 4) = #{add.(3, 4)}")

# Multi-line anonymous function
describe = fn x ->
  cond do
    x > 0 -> "positive"
    x < 0 -> "negative"
    true -> "zero"
  end
end

IO.puts("describe.(5) = #{describe.(5)}")
IO.puts("describe.(-3) = #{describe.(-3)}")
IO.puts("")

# ----------------------------------------------------------
# 3.2 Functions as Arguments
# ----------------------------------------------------------
# This is the core of higher-order functions:
# pass behavior as a parameter.
IO.puts("3.2 Functions as Arguments")

# apply_twice takes a function and applies it twice
apply_twice = fn func, value -> func.(func.(value)) end

double = fn x -> x * 2 end
IO.puts("double twice: #{apply_twice.(double, 3)}")

increment = fn x -> x + 1 end
IO.puts("increment twice: #{apply_twice.(increment, 10)}")
IO.puts("")

# ----------------------------------------------------------
# 3.3 Enum.map - Transform every element
# ----------------------------------------------------------
# map applies a function to each element, returning a new list.
# Replaces: for i in list { result.push(transform(i)) }
IO.puts("3.3 Enum.map")

numbers = [1, 2, 3, 4, 5]

squared = Enum.map(numbers, fn x -> x * x end)
IO.puts("Squared: #{inspect(squared)}")

names = ["alice", "bob", "charlie"]
capitalized = Enum.map(names, fn name -> String.capitalize(name) end)
IO.puts("Capitalized: #{inspect(capitalized)}")
IO.puts("")

# ----------------------------------------------------------
# 3.4 Enum.filter - Select elements
# ----------------------------------------------------------
# filter keeps only elements where the function returns true.
# Replaces: for i in list { if (pred(i)) result.push(i) }
IO.puts("3.4 Enum.filter")

evens = Enum.filter(numbers, fn x -> rem(x, 2) == 0 end)
IO.puts("Evens from #{inspect(numbers)}: #{inspect(evens)}")

long_names = Enum.filter(names, fn name -> String.length(name) > 3 end)
IO.puts("Long names: #{inspect(long_names)}")
IO.puts("")

# ----------------------------------------------------------
# 3.5 Enum.reduce - Collapse into a single value
# ----------------------------------------------------------
# reduce accumulates a result by applying a function to
# each element and an accumulator.
# Replaces: acc = init; for i in list { acc = combine(acc, i) }
IO.puts("3.5 Enum.reduce")

# Sum all numbers
sum = Enum.reduce(numbers, 0, fn x, acc -> acc + x end)
IO.puts("Sum of #{inspect(numbers)}: #{sum}")

# Build a string
sentence = Enum.reduce(["I", "love", "FP"], "", fn word, acc ->
  if acc == "", do: word, else: "#{acc} #{word}"
end)
IO.puts("Sentence: #{sentence}")

# Find max manually (to show reduce's power)
max = Enum.reduce(numbers, fn x, acc -> if x > acc, do: x, else: acc end)
IO.puts("Max: #{max}")
IO.puts("")

# ----------------------------------------------------------
# 3.6 The Capture Operator `&`
# ----------------------------------------------------------
# `&` captures a named function as a value, or creates
# short anonymous functions.
IO.puts("3.6 The Capture Operator &")

# Capture a named function: &Module.function/arity
lengths = Enum.map(names, &String.length/1)
IO.puts("Lengths of #{inspect(names)}: #{inspect(lengths)}")

# Short syntax for anonymous functions:
# &(&1 + &2) is equivalent to fn a, b -> a + b end
doubled = Enum.map(numbers, &(&1 * 2))
IO.puts("Doubled: #{inspect(doubled)}")

# &1, &2, etc. refer to the 1st, 2nd argument
pairs = Enum.map(numbers, &{&1, &1 * &1})
IO.puts("Number-square pairs: #{inspect(pairs)}")
IO.puts("")

# ----------------------------------------------------------
# 3.7 Closures
# ----------------------------------------------------------
# Anonymous functions capture variables from their
# surrounding scope. This "closes over" the variables.
IO.puts("3.7 Closures")

multiplier = 10
times_ten = fn x -> x * multiplier end
IO.puts("3 * 10 = #{times_ten.(3)}")

# The closure captures the VALUE, not a reference.
# Even if we rebind `multiplier`, the closure keeps 10.
multiplier = 99
IO.puts("Still 3 * 10 = #{times_ten.(3)} (closure captured 10)")
_ = multiplier
IO.puts("")

# ----------------------------------------------------------
# 3.8 Combining map, filter, reduce
# ----------------------------------------------------------
# Chain transformations to build data processing pipelines.
IO.puts("3.8 Combining Transformations")

# Problem: Sum of squares of even numbers from 1..10
result =
  1..10
  |> Enum.filter(&(rem(&1, 2) == 0))
  |> Enum.map(&(&1 * &1))
  |> Enum.reduce(0, &(&1 + &2))

IO.puts("Sum of squares of evens (1..10): #{result}")
IO.puts("Steps: [2,4,6,8,10] -> [4,16,36,64,100] -> 220")
IO.puts("")

# ----------------------------------------------------------
# 3.9 Functions Returning Functions
# ----------------------------------------------------------
# A function can return another function.
# This is called a "function factory" or "partial application".
IO.puts("3.9 Functions Returning Functions")

defmodule MathFactory do
  # Returns a function that multiplies by n
  def multiplier(n) do
    fn x -> x * n end
  end

  # Returns a function that adds n
  def adder(n) do
    fn x -> x + n end
  end
end

triple = MathFactory.multiplier(3)
add_five = MathFactory.adder(5)

IO.puts("triple.(7) = #{triple.(7)}")
IO.puts("add_five.(10) = #{add_five.(10)}")

# Compose them manually
IO.puts("add_five.(triple.(4)) = #{add_five.(triple.(4))}")
IO.puts("")

IO.puts("=== Key Takeaways ===")
IO.puts("1. Anonymous functions: fn args -> body end, called with .()")
IO.puts("2. Enum.map transforms, Enum.filter selects, Enum.reduce accumulates")
IO.puts("3. Capture operator & references named functions or creates short lambdas")
IO.puts("4. Closures capture values from their surrounding scope")
IO.puts("5. Functions can return functions (factories/partial application)")
IO.puts("6. Chain transformations with |> (pipe) for readable data pipelines")
