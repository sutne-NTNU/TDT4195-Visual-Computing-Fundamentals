PDVARS += --highlight-style=pygments 
PDVARS += --filter pandoc-crossref

.PHONY: create

create:
	@make report.pdf

%.pdf: %.md Makefile
	pandoc -i $< ${PDVARS} -o $@ --pdf-engine=pdflatex