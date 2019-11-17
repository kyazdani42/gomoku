.PHONY: all clean fclean re

all:
	@./build.sh

clean:
	@rm -rf public/* node_modules target .cache

fclean: clean
	@rm -f gomoku

re: fclean all
