#!/bin/bash

# Clear existing files
rm -f users.db
rm -f .sqlx/sqlx-data.json

# Set environment
export DATABASE_URL="sqlite://users.db"

# Create fresh database
sqlite3 users.db "VACUUM;"

# Install dependencies
cargo install sqlx-cli --no-default-features --features native-tls,sqlite

# First run to create schema
cargo run -- --setup 2>/dev/null || true

# Prepare SQLx queries
cargo sqlx prepare

# Regular run
cargo run