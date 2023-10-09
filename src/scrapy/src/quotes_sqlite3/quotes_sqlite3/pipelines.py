# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://docs.scrapy.org/en/latest/topics/item-pipeline.html

import sqlite3

# useful for handling different item types with a single interface
# from itemadapter import ItemAdapter


class QuotesSqlite3Pipeline:
    def __init__(self):
        self.connect()

    def connect(self):
        self.conn = sqlite3.connect("quotes.db")
        self.curr = self.conn.cursor()

    def process_item(self, item, spider):
        self.store(item)
        return item

    def store_author(self, item):
        self.curr.execute(
            "SELECT * FROM author WHERE name=?",
            (item["author"],),
        )
        row = self.curr.fetchone()
        if row:
            author_id = row[0]
        else:
            self.curr.execute(
                "INSERT INTO author (name, date, location) values (?,?,?)",
                (
                    item["author_name"],
                    item["author_date"],
                    item["author_location"],
                ),
            )
            author_id = self.curr.lastrowid
        return author_id

    def store(self, item):
        author_id = self.store_author(item)

        self.curr.execute(
            "INSERT INTO quotes (text, author_id) values (?,?)",
            (
                item["text"],
                author_id,
            ),
        )
        quotes_id = self.curr.lastrowid

        for tag in item["tags"]:
            # use existing tag, otherwise insert a new one
            self.curr.execute(
                "SELECT * FROM tags WHERE tag=?",
                (tag,),
            )
            row = self.curr.fetchone()
            if row:
                tags_id = row[0]
            else:
                self.curr.execute(
                    "INSERT INTO tags (tag) values (?)",
                    (tag,),
                )
                tags_id = self.curr.lastrowid

            self.curr.execute(
                "INSERT INTO quotes_tags (quotes_id, tags_id)  values (?,?)",
                (quotes_id, tags_id),
            )

        self.conn.commit()
