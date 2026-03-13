# ============================================================
# Lesson 2: Pattern Matching
# ============================================================
# FP Concepts: Pattern Matching, Destructuring
#
# Pattern matching is the backbone of Elixir.
# Instead of imperative if/else chains, we DECLARE what
# shape of data we expect, and Elixir routes execution
# accordingly. This leads to cleaner, more declarative code.
# ============================================================

IO.puts("=== Lesson 2: Pattern Matching ===\n")

# ----------------------------------------------------------
# 2.1 The Match Operator `=`
# ----------------------------------------------------------
# `=` doesn't assign - it MATCHES the left side against
# the right side and binds variables.
IO.puts("2.1 The Match Operator")

# Simple match
x = 1
IO.puts("x = #{x}")

# This works because left side matches right side
1 = x
IO.puts("1 = x succeeds (x is already 1)")

# Rebinding
x = 2
IO.puts("x rebound to #{x}")
IO.puts("")

# ----------------------------------------------------------
# 2.2 Destructuring Tuples
# ----------------------------------------------------------
# Extract values from tuples by matching their structure.
IO.puts("2.2 Destructuring Tuples")

{status, message} = {:ok, "Data loaded"}
IO.puts("Status: #{status}, Message: #{message}")

# Common pattern: match on :ok/:error
{:ok, value} = {:ok, 42}
IO.puts("Matched :ok, value = #{value}")

# Nested destructuring
{:user, {name, age}} = {:user, {"Bob", 25}}
IO.puts("Name: #{name}, Age: #{age}")
IO.puts("")

# ----------------------------------------------------------
# 2.3 Destructuring Lists
# ----------------------------------------------------------
# Use [head | tail] to decompose lists.
IO.puts("2.3 Destructuring Lists")

[first | rest] = [10, 20, 30, 40]
IO.puts("First: #{first}")
IO.puts("Rest: #{inspect(rest)}")

# Match multiple heads
[a, b | remaining] = [1, 2, 3, 4, 5]
IO.puts("a=#{a}, b=#{b}, remaining=#{inspect(remaining)}")

# Match exact list
[x, y, z] = [:foo, :bar, :baz]
IO.puts("Matched exactly: #{x}, #{y}, #{z}")
IO.puts("")

# ----------------------------------------------------------
# 2.4 Destructuring Maps
# ----------------------------------------------------------
# Match specific keys from a map. The pattern doesn't
# need to include ALL keys - partial matching works.
IO.puts("2.4 Destructuring Maps")

user = %{name: "Charlie", age: 28, email: "charlie@example.com"}

# Partial match - only extract what you need
%{name: user_name} = user
IO.puts("Extracted name: #{user_name}")

%{name: n, age: a} = user
IO.puts("Name: #{n}, Age: #{a}")
IO.puts("")

# ----------------------------------------------------------
# 2.5 The Wildcard `_`
# ----------------------------------------------------------
# Use `_` to ignore values you don't care about.
# This makes your intent explicit.
IO.puts("2.5 The Wildcard _")

{_, second, _} = {"ignored", "important", "ignored"}
IO.puts("Second element: #{second}")

[_ | tail] = [1, 2, 3]
IO.puts("Tail (head ignored): #{inspect(tail)}")
IO.puts("")

# ----------------------------------------------------------
# 2.6 Pattern Matching in `case`
# ----------------------------------------------------------
# `case` routes execution based on pattern matches.
# This replaces if/else chains with declarative patterns.
IO.puts("2.6 Pattern Matching in case")

http_response = {:ok, %{status: 200, body: "Hello World"}}

result = case http_response do
  {:ok, %{status: 200, body: body}} ->
    "Success: #{body}"
  {:ok, %{status: 404}} ->
    "Not Found"
  {:ok, %{status: status}} ->
    "HTTP #{status}"
  {:error, reason} ->
    "Error: #{reason}"
end

IO.puts(result)

# Matching on list patterns
list = [1, 2, 3]

description = case list do
  [] -> "empty list"
  [single] -> "single element: #{single}"
  [_, _] -> "two elements"
  [h | _] -> "starts with #{h}, has #{length(list)} elements"
end

IO.puts("List is: #{description}")
IO.puts("")

# ----------------------------------------------------------
# 2.7 Multi-clause Functions
# ----------------------------------------------------------
# Functions can have multiple clauses, each with different
# patterns. Elixir tries them top to bottom, using the
# first one that matches.
IO.puts("2.7 Multi-clause Functions")

defmodule Greeter do
  # Clause 1: match specific language
  def greet(:english, name), do: "Hello, #{name}!"
  def greet(:japanese, name), do: "こんにちは、#{name}！"
  def greet(:spanish, name), do: "¡Hola, #{name}!"

  # Clause 4: catch-all
  def greet(_language, name), do: "Hi, #{name}!"
end

IO.puts(Greeter.greet(:english, "Alice"))
IO.puts(Greeter.greet(:japanese, "太郎"))
IO.puts(Greeter.greet(:spanish, "Carlos"))
IO.puts(Greeter.greet(:french, "Pierre"))
IO.puts("")

# ----------------------------------------------------------
# 2.8 Guards with `when`
# ----------------------------------------------------------
# Guards add extra conditions to pattern matches.
# They enable type checking and value constraints.
IO.puts("2.8 Guards")

defmodule TypeChecker do
  def check(x) when is_integer(x) and x > 0, do: "positive integer: #{x}"
  def check(x) when is_integer(x) and x < 0, do: "negative integer: #{x}"
  def check(0), do: "zero"
  def check(x) when is_float(x), do: "float: #{x}"
  def check(x) when is_binary(x), do: "string: #{x}"
  def check(x) when is_atom(x), do: "atom: #{x}"
  def check(_), do: "something else"
end

IO.puts(TypeChecker.check(42))
IO.puts(TypeChecker.check(-7))
IO.puts(TypeChecker.check(0))
IO.puts(TypeChecker.check(3.14))
IO.puts(TypeChecker.check("hello"))
IO.puts(TypeChecker.check(:world))
IO.puts("")

# ----------------------------------------------------------
# 2.9 The Pin Operator `^`
# ----------------------------------------------------------
# Use `^` to match against an existing variable's value
# instead of rebinding it.
IO.puts("2.9 The Pin Operator ^")

expected = :ok
{^expected, value} = {:ok, 42}
IO.puts("Pinned match succeeded, value = #{value}")

# Without pin, `status` would rebind:
# {status, _} = {:error, "fail"}  => status = :error
# With pin:
# {^expected, _} = {:error, "fail"}  => ** (MatchError)
IO.puts("Pin prevents rebinding - ensures value matches exactly")
IO.puts("")

IO.puts("=== Key Takeaways ===")
IO.puts("1. `=` is pattern matching - it matches shapes of data")
IO.puts("2. Destructure tuples, lists, and maps to extract values")
IO.puts("3. `_` ignores values explicitly")
IO.puts("4. `case` replaces if/else chains with pattern matching")
IO.puts("5. Multi-clause functions dispatch by data shape")
IO.puts("6. Guards (`when`) add conditions to matches")
IO.puts("7. Pin operator `^` matches without rebinding")
