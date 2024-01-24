# open sqlite databse and print all rows, count rows
import sqlite3
import os

path = '/home/miki/remote_dev/watchywatchywatcher_tokio/watchy.db'

if not os.path.exists(path):
    print('database does not exist')
    exit(1)

conn = sqlite3.connect(path)
c = conn.cursor()
query = 'SELECT * FROM folders'
c.execute(query)
rows = c.fetchall()
for row in rows:
    print(row)
print(len(rows))
conn.close()

