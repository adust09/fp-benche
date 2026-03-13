# ============================================================
# Lesson 1: Immutability and Expressions
# ============================================================
# FP Concepts: Immutability, Expression-based evaluation
#
# In functional programming, data is IMMUTABLE - once created,
# it cannot be changed. Every operation returns NEW data.
# The `=` operator is not assignment, it's PATTERN MATCHING
# (binding a value to a name).
# ============================================================

IO.puts("=== Lesson 1: Immutability and Expressions ===\n")

# ----------------------------------------------------------
# 1.1 Variable Binding
# ----------------------------------------------------------
# In Elixir, `=` is the match operator, not assignment.
# It binds a value to a name on the left side.

x = 42
IO.puts("1.1 Variable Binding")
IO.puts("x = #{x}")

# Rebinding creates a NEW binding, not mutation.
# The original value 42 still exists in memory until GC collects it.
x = x + 1
IO.puts("x (rebound) = #{x}")
IO.puts("")

# ----------------------------------------------------------
# 1.2 Basic Data Types
# ----------------------------------------------------------
IO.puts("1.2 Basic Data Types")

# Atoms - named constants (like symbols in Ruby)
status = :ok
IO.puts("Atom: #{status}")

# Strings are UTF-8 encoded binaries
greeting = "Hello, FP!"
IO.puts("String: #{greeting}")

# Numbers
integer = 100
float = 3.14
IO.puts("Integer: #{integer}, Float: #{float}")
IO.puts("")

# ----------------------------------------------------------
# 1.3 Tuples - Fixed-size collections
# ----------------------------------------------------------
# Tuples are stored contiguously in memory.
# Great for returning multiple values from functions.
IO.puts("1.3 Tuples")

result = {:ok, "success", 200}
IO.puts("Tuple: #{inspect(result)}")

# Accessing elements (zero-indexed)
IO.puts("First element: #{elem(result, 0)}")
IO.puts("Tuple size: #{tuple_size(result)}")
IO.puts("")

# ----------------------------------------------------------
# 1.4 Lists - Linked lists
# ----------------------------------------------------------
# Lists are linked lists, not arrays.
# Prepending is O(1), accessing by index is O(n).
IO.puts("1.4 Lists")

numbers = [1, 2, 3, 4, 5]
IO.puts("List: #{inspect(numbers)}")

# The cons operator [head | tail] - fundamental to FP
# This DECOMPOSES a list into its first element and the rest.
[head | tail] = numbers
IO.puts("Head: #{head}")
IO.puts("Tail: #{inspect(tail)}")

# Prepending is efficient (O(1)) - creates a new list
new_list = [0 | numbers]
IO.puts("Prepended: #{inspect(new_list)}")

# The original list is UNCHANGED (immutability!)
IO.puts("Original: #{inspect(numbers)}")
IO.puts("")

# ----------------------------------------------------------
# 1.5 Maps - Key-value pairs
# ----------------------------------------------------------
# Maps are the go-to data structure for structured data.
IO.puts("1.5 Maps")

user = %{name: "Alice", age: 30, role: :admin}
IO.puts("Map: #{inspect(user)}")

# Accessing values
IO.puts("Name: #{user.name}")
IO.puts("Role: #{user[:role]}")

# Updating a map returns a NEW map (immutability!)
# The `|` syntax only works for EXISTING keys.
updated_user = %{user | age: 31}
IO.puts("Updated: #{inspect(updated_user)}")
IO.puts("Original: #{inspect(user)}")
IO.puts("")

# ----------------------------------------------------------
# 1.6 Everything is an Expression
# ----------------------------------------------------------
# In FP, everything evaluates to a value.
# There are no "statements" - only expressions.
IO.puts("1.6 Everything is an Expression")

# `if` returns a value (it's an expression, not a statement)
message = if 10 > 5, do: "ten is greater", else: "five is greater"
IO.puts("if expression: #{message}")

# `case` is also an expression
label = case {:ok, 42} do
  {:ok, value} -> "Got value: #{value}"
  {:error, reason} -> "Error: #{reason}"
end
IO.puts("case expression: #{label}")
IO.puts("")

# ----------------------------------------------------------
# 1.7 Keyword Lists - Ordered key-value pairs
# ----------------------------------------------------------
# Keyword lists are lists of {atom, value} tuples.
# They allow duplicate keys and maintain order.
IO.puts("1.7 Keyword Lists")

options = [timeout: 5000, retries: 3, verbose: true]
IO.puts("Keyword list: #{inspect(options)}")
IO.puts("Timeout: #{options[:timeout]}")
IO.puts("")

IO.puts("=== Key Takeaways ===")
IO.puts("1. `=` is pattern matching (binding), not assignment")
IO.puts("2. Data is immutable - updates return NEW values")
IO.puts("3. Lists use [head | tail] decomposition")
IO.puts("4. Maps use %{map | key: new_val} for updates")
IO.puts("5. Everything is an expression that returns a value")
