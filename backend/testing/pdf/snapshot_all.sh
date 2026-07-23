#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../../"

snapshots=(
  customers
  warehouses
  taxes
  products
  inventory
  inventory-movements
  inventory-reservations
  services
  tasks
  worksheets
)

for snapshot in "${snapshots[@]}"; do
  cargo run --bin obvia_cli dev pdf-test-snapshot "$snapshot"
done
