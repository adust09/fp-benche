---
title: ETH2-Inspired Consensus Workload Benchmark Results
last_updated: 2026-03-15
tags:
  - benchmark
  - consensus
  - ethereum
  - functional-programming
  - rustica
---

# ETH2-Inspired Consensus Workload Benchmark Results

## Overview

ETH 2.0 Beacon Chain のコンセンサスクライアントに頻出する操作パターンにおいて、
Rust の3つのコーディングスタイルのパフォーマンス差を計測した。

| スタイル | 説明 |
|---------|------|
| **Imperative** | for ループ + in-place ミューテーション |
| **Functional** | イテレータチェーン（`iter().map().collect()`）|
| **Rustica** | 永続データ構造（`PersistentVector`）による immutable 更新 |

### スコープと限定事項

- 全バリアントは **Rust** で実装。結果は「Rust 内でのコーディングスタイル差」を示す
- 「ETH2-inspired」であり ETH2 仕様準拠ではない
- Rustica バリアントは `balances` フィールドのみ `PersistentVector<u64>` に置換したハイブリッド表現。全面採用の代理ではない
- State Root の Rustica は「persistent random-access rebuild + 各層再構築の allocation コスト込み」

### ベンチマーク環境

- CPU: Kali Linux 6.18.12+kali-amd64
- Rust: Edition 2021, release profile (LTO, codegen-units=1, opt-level=3)
- Criterion 0.5 (sample_size=10-20)
- rustica 0.11, sha2 0.10

---

## 1. Epoch Processing（報酬/ペナルティ計算）

全バリデータの残高を `base_reward` に基づき更新する。2パス処理（`total_active_balance` 計算 → balance 更新）。

### e2e 結果（Vec 入力 → 処理 → 出力）

#### n=10,000 validators

| バリアント | seed=42 | seed=123 | seed=7777 | 倍率 (中央値) |
|-----------|---------|----------|-----------|:---:|
| **imperative** | 41.1 µs | 38.2 µs | 32.3 µs | 1.0x |
| **functional** | 24.1 µs | 23.4 µs | 23.3 µs | 0.6x |
| **rustica** | 7.17 ms | 7.44 ms | 7.31 ms | **194x** |

#### n=100,000 validators

| バリアント | seed=42 | seed=123 | seed=7777 | 倍率 (中央値) |
|-----------|---------|----------|-----------|:---:|
| **imperative** | 361 µs | 361 µs | 294 µs | 1.0x |
| **functional** | 235 µs | 235 µs | 268 µs | 0.7x |
| **rustica** | 76.6 ms | 76.6 ms | 80.3 ms | **226x** |

### 分析

- **FP (iterator chain) は命令型より速い**。`iter().zip().map().collect()` はコンパイラのベクトル化・分岐予測最適化の恩恵を受けやすい
- **Rustica は ~200倍遅い**。`PersistentVector::update()` を n 回呼ぶため、RRB ツリーの O(log n) 更新コストが n 回積み重なる
- 副作用（in-place ミューテーション）の排除は、FP スタイルでは性能劣化なし、永続データ構造では大幅な劣化

---

## 2. Balance Updates（deposit/withdrawal/slashing）

部分的な残高更新。100k validators に対して N 件の操作を適用（deposits 50% / withdrawals 30% / slashings 20%）。

### e2e 結果（seed=42）

| 操作数 | imperative | functional | rustica | FP倍率 | Rustica倍率 |
|--------|-----------|------------|---------|:---:|:---:|
| 100 ops | 231 ns | 28.0 µs | 293 µs | 121x | **1268x** |
| 1,000 ops | 1.54 µs | 29.0 µs | 1.35 ms | 18.8x | **877x** |
| 10,000 ops | 14.8 µs | 40.4 µs | 12.1 ms | 2.7x | **818x** |

### 分析

- **命令型が圧倒的に速い**。in-place 更新は `balances[idx] += amount` のみで allocation ゼロ
- **FP は Vec clone のオーバーヘッド**が支配的。操作数に関わらず ~28µs（100k 要素の clone コスト）
- **Rustica のコストは操作数に比例**。各 `update()` が O(log n) のため、1k ops で 1.35ms
- 少数の部分更新では FP の「Vec 全体 clone」が命令型の「数個の index 更新」より 100 倍以上遅い
- これはコンセンサスクライアントの典型的な操作パターン（1スロットで数十〜数百件の状態変更）において、FP スタイルの clone コストが無視できないことを示す

---

## 3. State Root（Merkle ツリー hash_tree_root）

バリデータ残高からボトムアップでMerkleルートを計算。SHA-256版とXOR版（構造コスト分離用）。

### SHA-256 版（e2e, seed=42）

| サイズ | imperative | functional | rustica | FP倍率 | Rustica倍率 |
|--------|-----------|------------|---------|:---:|:---:|
| n=1,024 | 182 µs | 187 µs | 215 µs | 1.03x | 1.18x |
| n=8,192 | 1.47 ms | 1.49 ms | 1.94 ms | 1.01x | 1.32x |

### XOR 版（structure-only, e2e, seed=42）

| サイズ | imperative | functional | rustica | FP倍率 | Rustica倍率 |
|--------|-----------|------------|---------|:---:|:---:|
| n=1,024 | 1.81 µs | 2.45 µs | 32.8 µs | 1.35x | **18.1x** |
| n=8,192 | 17.7 µs | 19.3 µs | 389 µs | 1.09x | **22.0x** |

### 分析

- **SHA-256 版**: ハッシュ計算が支配的で、スタイル差はわずか（Rustica でも 1.3 倍程度）
- **XOR 版（構造コスト分離）**: ハッシュを除くと Rustica は **22 倍遅い**。各レベルで `PersistentVector::from_slice()` による再構築が発生するため
- **FP はほぼ命令型と同等**。`chunks(2).map().collect()` は allocation コストが小さい
- コンセンサスクライアントの state root 計算では、ハッシュが支配的なためデータ構造スタイルの影響は相対的に小さい

---

## 総合考察: コンセンサスクライアントにおける副作用と関数型スタイル

### FP（イテレータチェーン）は Rust では実用的

| 操作パターン | FP vs 命令型 | 判定 |
|-------------|:---:|------|
| 全要素 map 更新 | **0.6-0.7x（FP が速い）** | FP 採用可能 |
| ハッシュ支配的な処理 | 1.01-1.09x | FP 採用可能 |
| 少数の部分 index 更新 | 2.7-121x（FP が遅い） | 命令型推奨 |

- Rust のイテレータは LLVM による積極的な最適化を受けるため、全要素処理ではループより速くなるケースがある
- ただし部分更新パターンでは Vec clone のオーバーヘッドが大きく、in-place 変更に劣る

### 永続データ構造（Rustica）はコンセンサスクライアントには不向き

| 操作パターン | Rustica vs 命令型 | 判定 |
|-------------|:---:|------|
| 全要素 map 更新（n=100k） | **226x** | 不可 |
| 部分 index 更新（100 ops） | **1268x** | 不可 |
| ハッシュ支配的（SHA-256, n=8k） | 1.32x | 許容可能 |
| 構造操作のみ（XOR, n=8k） | **22x** | 不可 |

- `PersistentVector::update()` の O(log n) コストは、バリデータ数が多い（10万〜50万）コンセンサスクライアントでは致命的
- 構造共有（structural sharing）の恩恵よりも、毎回の RRB ツリー走査コストが支配的
- Haskell 等の純粋関数型言語でも同様の永続データ構造を使うため、**コンセンサスクライアントの状態遷移関数においては、ミュータブルな配列ベースの実装が大幅に有利**

### 結論

1. **FP スタイル（イテレータチェーン）は Rust のコンセンサスクライアントに採用可能**。全要素処理では命令型と同等以上の性能
2. **永続データ構造ベースの純粋関数型アプローチは、コンセンサスクライアントの状態遷移には不向き**。特に大量の index 更新操作で 100-1000 倍のオーバーヘッド
3. **推奨アプローチ**: Rust の所有権システム + イテレータチェーンで「副作用を局所化した FP スタイル」を採用し、データ構造は通常の `Vec` を使用する
