<div align="center">
<h1>Learn Scrapy</h1>

My notes on learning Scrapy 🕷.

<a href="https://github.com/azzamsa/learn-scrapy/workflows/ci.yml">
    <img src="https://github.com/azzamsa/learn-scrapy/workflows/ci/badge.svg" alt="Build status" />
</a>
<a href="https://github.com/azzamsa/learn-scrapy">
    <img src="https://img.shields.io/badge/Python-3.9%2B-blue" alt="Python versions" />
</a>

<a href="https://github.com/psf/black">
    <img src="https://img.shields.io/badge/code%20style-black-000000.svg" alt="Code Style " />
</a>
</div>

---


This is not an exhausting resource for the public to learn.
Use [official Scrapy tutorial][scrapy-tutorial] instead.

Use the [guide](docs/guide.md) as a starting point.

This project contains the scrapers below:

- [single](src/single): The collection of one single file scrapy scraper.
- [quotes](src/quotes): A scraper for [quotes.toscrape.com](http://quotes.toscrape.com).
- [quotes_sqlite3](src/quotes_sqlite3): A scraper with a pipeline storing items to sqlite3 database.
- [quotes_js](src/quotes_js): A scraper for JavaScript generated content.
- [books](src/books): A scraper for [books.toscrape.com](http://books.toscrape.com).
- [amazon](src/amazon): A scraper for [amazon.com](http://amazon.com).

