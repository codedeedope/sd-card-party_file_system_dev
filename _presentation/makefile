
all: presentation.pdf handout.pdf

presentation.pdf: presentation.md makefile
	pandoc presentation.md -fmarkdown -t beamer -o presentation.pdf \
		--slide-level=2\
		--normalize \
		-V theme:Singapore\
		-V colortheme:beaver

delete:
	rm presentation.pdf
