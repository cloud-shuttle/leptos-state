#!/bin/bash

# Phase 8: Trait Bounds Audit Script
# Identifies files with restrictive trait bounds that need cleanup

echo "=== PHASE 8: TRAIT BOUNDS AUDIT ==="
echo "Finding files with restrictive trait bounds..."
echo

# Find files with Default bounds (often unnecessary)
echo "Files with 'Default' trait bounds (not PartialEq):"
grep -r ": .*Default" src/ --include="*.rs" | grep -v "PartialEq.*Default" | wc -l
echo

# Find files with Eq bounds (restrictive - not PartialEq)
echo "Files with 'Eq' trait bounds (not PartialEq):"
grep -r ": .*Eq" src/ --include="*.rs" | grep -v "PartialEq" | wc -l
echo

# Find files with Hash bounds (very restrictive)
echo "Files with 'Hash' trait bounds:"
grep -r ": .*Hash" src/ --include="*.rs" | wc -l
echo

# Find generic parameters with multiple restrictive bounds
echo "Files with complex trait bound combinations:"
grep -r "Clone + .*Debug.*Default" src/ --include="*.rs" | wc -l
echo

# Top 10 files by truly restrictive bounds (Default/Eq/Hash, excluding PartialEq)
echo "=== TOP 10 FILES BY TRULY RESTRICTIVE BOUNDS ==="
for file in $(find src -name "*.rs" -type f); do
    # Count Default (not in PartialEq combinations), Eq (not PartialEq), Hash
    default_count=$(grep ": .*Default" "$file" 2>/dev/null | grep -v "PartialEq.*Default" | wc -l)
    eq_count=$(grep ": .*Eq" "$file" 2>/dev/null | grep -v "PartialEq" | wc -l)
    hash_count=$(grep -c ": .*Hash" "$file" 2>/dev/null)
    total=$((default_count + eq_count + hash_count))
    echo "$total $file"
done | sort -nr | head -10
echo

# Files with most trait bound lines
echo "=== FILES WITH MOST TRAIT BOUND LINES ==="
for file in $(find src -name "*.rs" -type f); do
    lines=$(grep -c "where\|:" "$file" 2>/dev/null || echo 0)
    echo "$lines $file"
done | sort -nr | head -10
echo

echo "Audit complete. Use this data to prioritize cleanup order."
