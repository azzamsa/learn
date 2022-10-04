import scrapy


class BookSpiderSpider(scrapy.Spider):
    name = "book_spider"
    allowed_domains = ["books.toscrape.com"]
    start_urls = ["https://books.toscrape.com/"]

    def parse(self, response):
        books = response.css(".product_pod")
        for book in books:
            book_url = book.css("h3 a::attr(href)").get()
            yield response.follow(book_url, callback=self.parse_book)

        next_page = response.css(".next a::attr(href)").get()
        if next_page is not None:
            yield response.follow(next_page, callback=self.parse)

    def parse_book(self, response):
        title = response.css("h1::text").get()
        price = response.css(".price_color::text").get()
        stock = response.css(".availability::text").getall()[1].strip()

        yield {
            "title": title,
            "price": price,
            "stock": stock,
        }
