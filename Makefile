
1a:
	echo "day 1 a:"
	cd 1 && cargo run --quiet one

1b:
	echo "day 1 b:"
	cd 1 && cargo run --quiet two

1: 1a 1b

2a:
	echo "day 2 a:"
	cd 2 && cargo run --quiet one

2: 2a
