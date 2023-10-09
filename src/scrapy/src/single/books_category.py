import scrapy


class BooksCategorySpider(scrapy.Spider):
    name = "books_category_spider"
    start_urls = ["https://books.toscrape.com/"]

    def parse(self, response):
        categories = response.css(".nav-list li li")
        for category in categories:
            category_url = category.css("li a::attr(href)").get()
            self.logger.info(f"::: Category URL: {category_url}")
            yield response.follow(category_url, callback=self.parse_category_page)

    def parse_category_page(self, response):
        books = response.css(".product_pod")
        category_name = response.css(".page-header.action h1::text").get()
        for book in books:
            book_url = book.css("h3 a::attr(href)").get()
            self.logger.info(f"::: Category URL: {book_url}")
            yield response.follow(
                book_url,
                callback=self.parse_book,
                cb_kwargs={"category_name": category_name},
            )

        next_page = response.css(".next a::attr(href)").get()
        if next_page is not None:
            self.logger.info(f"::: Next page: {next_page}")
            yield response.follow(next_page, callback=self.parse_category_page)

    def parse_book(self, response, category_name):
        title = response.css("h1::text").get()
        price = response.css(".price_color::text").get()
        stock = response.css(".availability::text").getall()[1].strip()

        yield {
            "title": title,
            "price": price,
            "stock": stock,
            "category": category_name,
        }
