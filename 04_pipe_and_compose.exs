# ============================================================
# Lesson 4: Pipe Operator and Function Composition
# ============================================================
# FP Concepts: Pipe Operator, Function Composition,
#              Data Transformation Pipelines
#
# The pipe operator `|>` transforms nested function calls
# into a readable left-to-right data flow. This is Elixir's
# signature feature and embodies the FP principle of
# composing small, focused functions into larger behaviors.
# ============================================================

IO.puts("=== Lesson 4: Pipe Operator and Function Composition ===\n")

# ----------------------------------------------------------
# 4.1 The Problem: Nested Function Calls
# ----------------------------------------------------------
# Without pipes, complex transformations become deeply nested
# and hard to read (inside-out evaluation).
IO.puts("4.1 Nested vs. Piped")

sentence = "  the quick brown fox jumps over the lazy dog  "

# Nested (read inside-out): hard to follow
nested_result = String.replace(String.upcase(String.trim(sentence)), " ", "-")
IO.puts("Nested:  #{nested_result}")

# Piped (read left-to-right): natural data flow
piped_result =
  sentence
  |> String.trim()
  |> String.upcase()
  |> String.replace(" ", "-")

IO.puts("Piped:   #{piped_result}")
IO.puts("")

# ----------------------------------------------------------
# 4.2 How `|>` Works
# ----------------------------------------------------------
# `x |> f(a, b)` becomes `f(x, a, b)`
# The left side becomes the FIRST argument of the right side.
IO.puts("4.2 How Pipe Works")

# These are equivalent:
IO.puts(inspect(Enum.take(Enum.sort([3, 1, 4, 1, 5]), 3)))

[3, 1, 4, 1, 5]
|> Enum.sort()
|> Enum.take(3)
|> inspect()
|> IO.puts()

IO.puts("")

# ----------------------------------------------------------
# 4.3 Building a Text Processing Pipeline
# ----------------------------------------------------------
IO.puts("4.3 Text Processing Pipeline")

defmodule TextProcessor do
  # Each function does ONE thing (Single Responsibility)
  def normalize(text), do: String.downcase(String.trim(text))
  def remove_punctuation(text), do: String.replace(text, ~r/[^\w\s]/u, "")
  def split_words(text), do: String.split(text, ~r/\s+/)
  def count_words(words), do: length(words)
  def unique_words(words), do: Enum.uniq(words)
end

text = "  Hello, World! Hello, Elixir!  "

word_count =
  text
  |> TextProcessor.normalize()
  |> TextProcessor.remove_punctuation()
  |> TextProcessor.split_words()
  |> TextProcessor.count_words()

IO.puts("\"#{String.trim(text)}\" has #{word_count} words")

unique =
  text
  |> TextProcessor.normalize()
  |> TextProcessor.remove_punctuation()
  |> TextProcessor.split_words()
  |> TextProcessor.unique_words()

IO.puts("Unique words: #{inspect(unique)}")
IO.puts("")

# ----------------------------------------------------------
# 4.4 Order Processing Pipeline
# ----------------------------------------------------------
# A practical example: process a food order with
# discounts, tax, and formatting.
IO.puts("4.4 Order Processing Pipeline")

defmodule OrderProcessor do
  def create_order(items) do
    %{items: items, subtotal: calculate_subtotal(items)}
  end

  defp calculate_subtotal(items) do
    Enum.reduce(items, 0, fn {_name, price, qty}, acc -> acc + price * qty end)
  end

  def apply_discount(order, threshold, percent) do
    if order.subtotal >= threshold do
      discount = order.subtotal * percent / 100
      %{order | subtotal: order.subtotal - discount}
      |> Map.put(:discount, discount)
    else
      Map.put(order, :discount, 0)
    end
  end

  def apply_tax(order, rate) do
    tax = order.subtotal * rate / 100
    order
    |> Map.put(:tax, tax)
    |> Map.put(:total, order.subtotal + tax)
  end

  def format_receipt(order) do
    """
    --- Receipt ---
    Items: #{length(order.items)}
    Subtotal: $#{:erlang.float_to_binary(order.subtotal / 1, [decimals: 2])}
    Discount: -$#{:erlang.float_to_binary(order.discount / 1, [decimals: 2])}
    Tax: $#{:erlang.float_to_binary(order.tax / 1, [decimals: 2])}
    Total: $#{:erlang.float_to_binary(order.total / 1, [decimals: 2])}
    ---------------\
    """
  end
end

items = [
  {"Burger", 12, 2},
  {"Fries", 5, 3},
  {"Soda", 3, 2}
]

receipt =
  items
  |> OrderProcessor.create_order()
  |> OrderProcessor.apply_discount(20, 10)
  |> OrderProcessor.apply_tax(8)
  |> OrderProcessor.format_receipt()

IO.puts(receipt)
IO.puts("")

# ----------------------------------------------------------
# 4.5 Word Frequency Counter
# ----------------------------------------------------------
# Combine pipes with Enum and Map operations to build
# a word frequency counter.
IO.puts("4.5 Word Frequency Counter")

defmodule WordFrequency do
  def count(text) do
    text
    |> String.downcase()
    |> String.replace(~r/[^\w\s]/u, "")
    |> String.split(~r/\s+/, trim: true)
    |> Enum.reduce(%{}, fn word, acc ->
      Map.update(acc, word, 1, &(&1 + 1))
    end)
  end

  def top_n(freq_map, n) do
    freq_map
    |> Enum.sort_by(fn {_word, count} -> count end, :desc)
    |> Enum.take(n)
  end

  def format(frequencies) do
    Enum.map(frequencies, fn {word, count} ->
      bar = String.duplicate("█", count)
      "  #{String.pad_trailing(word, 10)} #{bar} (#{count})"
    end)
    |> Enum.join("\n")
  end
end

sample_text = """
the quick brown fox jumps over the lazy dog
the fox and the dog became friends
the quick fox runs and jumps again
"""

IO.puts("Top 5 words:")
sample_text
|> WordFrequency.count()
|> WordFrequency.top_n(5)
|> WordFrequency.format()
|> IO.puts()

IO.puts("")

# ----------------------------------------------------------
# 4.6 Private Functions with `defp`
# ----------------------------------------------------------
# `defp` defines private functions - only callable within
# the module. This enforces encapsulation.
IO.puts("4.6 Private Functions (defp)")

defmodule Temperature do
  # Public API
  def celsius_to_description(celsius) do
    celsius
    |> to_fahrenheit()
    |> describe()
  end

  # Private helpers - implementation details hidden
  defp to_fahrenheit(c), do: c * 9 / 5 + 32

  defp describe(f) when f < 32, do: "freezing (#{format_f(f)}°F)"
  defp describe(f) when f < 68, do: "cold (#{format_f(f)}°F)"
  defp describe(f) when f < 86, do: "comfortable (#{format_f(f)}°F)"
  defp describe(f), do: "hot (#{format_f(f)}°F)"

  defp format_f(f), do: :erlang.float_to_binary(f / 1, [decimals: 1])
end

for temp <- [-10, 10, 25, 35] do
  IO.puts("  #{temp}°C is #{Temperature.celsius_to_description(temp)}")
end
IO.puts("")

IO.puts("=== Key Takeaways ===")
IO.puts("1. `|>` passes the left result as the first arg to the right function")
IO.puts("2. Pipes transform nested calls into readable left-to-right flow")
IO.puts("3. Design functions to take 'data' as the first argument (pipe-friendly)")
IO.puts("4. Small, composable functions > large monolithic ones")
IO.puts("5. `defp` hides implementation details (encapsulation)")
IO.puts("6. Map.update/4 is great for building frequency maps")
