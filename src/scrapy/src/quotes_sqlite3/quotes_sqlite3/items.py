# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


class QuotesSqlite3Item(scrapy.Item):
    # define the fields for your item here like:
    author_name = scrapy.Field()
    author_location = scrapy.Field()
    author_date = scrapy.Field()
    text = scrapy.Field()
    tags = scrapy.Field()
