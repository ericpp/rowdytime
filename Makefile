all: db build

db: schema.sql
	sqlite3 /tmp/rowdytime.db < schema.sql

feeds:
	curl -o 1.txt https://iono.fm/rss/chan/6203
	curl -o 2.txt https://feeds.podcastindex.org/pc20.xml

build:
	cargo build
