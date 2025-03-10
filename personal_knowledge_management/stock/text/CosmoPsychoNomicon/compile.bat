docker build -t my_lualatex .
docker run --rm -v "%~dp0\materials:/app" my_lualatex