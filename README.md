#### How to use
```bash
# Export addresses and transactions from DB to jsonl
DATABASE_URL=${DATABASE_URL} RUSTFLAGS='-C target-cpu=native' cargo run --release -- export \
  --id ${SERVICE_ID} --key ${SECRET} --salt ${SALT}

# Import addresses and transactions from jsonl to DB
DATABASE_URL=${DATABASE_URL} RUSTFLAGS='-C target-cpu=native' cargo run --release -- import \
  --key ${SECRET} --salt ${SALT}
```