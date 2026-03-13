#!/usr/bin/env python3
"""Generate test data for benchmarks.

All data is generated with seed=42 for reproducibility.

Output files:
  - large_text.txt    : ~10MB of lorem-ipsum-style text for word count
  - users_100k.json   : 100K user records for JSON filter
  - sales_100k.csv    : 100K sales records for group-by aggregation

Format specifications:
  - users JSON: [{"id": int, "name": str, "age": int, "city": str, "active": bool}]
  - sales CSV:  category,amount (no header)
"""

import json
import os
import random

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
random.seed(42)

WORDS = [
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "it",
    "for", "not", "on", "with", "he", "as", "you", "do", "at", "this",
    "but", "his", "by", "from", "they", "we", "say", "her", "she", "or",
    "an", "will", "my", "one", "all", "would", "there", "their", "what",
    "so", "up", "out", "if", "about", "who", "get", "which", "go", "me",
    "when", "make", "can", "like", "time", "no", "just", "him", "know",
    "take", "people", "into", "year", "your", "good", "some", "could",
    "them", "see", "other", "than", "then", "now", "look", "only", "come",
    "its", "over", "think", "also", "back", "after", "use", "two", "how",
    "our", "work", "first", "well", "way", "even", "new", "want", "because",
    "any", "these", "give", "day", "most", "us",
]

CITIES = ["Tokyo", "Osaka", "Kyoto", "Nagoya", "Sapporo",
          "Fukuoka", "Kobe", "Yokohama", "Sendai", "Hiroshima"]

CATEGORIES = ["food", "drink", "electronics", "clothing", "books",
              "sports", "toys", "health", "garden", "automotive"]


def generate_text(target_bytes: int) -> str:
    """Generate random text from word list."""
    lines = []
    current_size = 0
    while current_size < target_bytes:
        line_words = random.randint(5, 15)
        line = " ".join(random.choice(WORDS) for _ in range(line_words))
        lines.append(line)
        current_size += len(line) + 1
    return "\n".join(lines)


def generate_users(count: int) -> list:
    """Generate user records matching the JSON schema."""
    users = []
    for i in range(count):
        users.append({
            "id": i,
            "name": f"user_{i}",
            "age": random.randint(18, 65),
            "city": random.choice(CITIES),
            "active": random.choice([True, False]),
        })
    return users


def generate_sales(count: int) -> list:
    """Generate sales records as (category, amount) tuples."""
    return [
        (random.choice(CATEGORIES), round(random.uniform(1.0, 500.0), 2))
        for _ in range(count)
    ]


def main():
    # Large text (~10MB)
    path = os.path.join(SCRIPT_DIR, "large_text.txt")
    print(f"Generating {path} (~10MB)...")
    text = generate_text(10 * 1024 * 1024)
    with open(path, "w") as f:
        f.write(text)
    print(f"  -> {os.path.getsize(path) / 1024 / 1024:.1f}MB")

    # Users JSON (100K records)
    path = os.path.join(SCRIPT_DIR, "users_100k.json")
    print(f"Generating {path} (100K users)...")
    users = generate_users(100_000)
    with open(path, "w") as f:
        json.dump(users, f)
    print(f"  -> {os.path.getsize(path) / 1024 / 1024:.1f}MB")

    # Sales CSV (100K records)
    path = os.path.join(SCRIPT_DIR, "sales_100k.csv")
    print(f"Generating {path} (100K sales)...")
    sales = generate_sales(100_000)
    with open(path, "w") as f:
        for cat, amount in sales:
            f.write(f"{cat},{amount}\n")
    print(f"  -> {os.path.getsize(path) / 1024 / 1024:.1f}MB")

    print("Done!")


if __name__ == "__main__":
    main()
