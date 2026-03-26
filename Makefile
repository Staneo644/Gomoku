NAME = Gomoku

all:
	cargo build --release
	cp target/release/gomoku $(NAME)

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all