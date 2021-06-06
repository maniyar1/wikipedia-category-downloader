Simple tool to download a wikipedia category, example usage:

Download pages from this category and (default) one level down of sub-categories

./wikipedia-category-downloader https://en.wikipedia.org/wiki/Category:Marxism 

Download pages just from this category, no sub-categories

./wikipedia-category-downloader https://en.wikipedia.org/wiki/Category:Marxism -l 0

It will store everything in ./wiki/, because wikipedia uses absolute link paths I recommend serving these with something like twisted (twistd). You can use the python web server (python3 -m http.server) but it doesn't correctly identify files as html. You can use the -a flag to output files with .html endings, but then links won't work.
