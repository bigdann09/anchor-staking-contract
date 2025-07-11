validator:
	solana-test-validator --reset

build:
	anchor build

test:
	anchor test --skip-local-validator