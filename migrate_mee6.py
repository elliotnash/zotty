"""
Script to import mee6 data into etech bot db
Need to make sure mee6-py-api module is installed
`pip3 install mee6-py-api`
BACKUP YOUR DATABI
"""

import asyncio
import sys
import sqlite3
from mee6_py_api import API as mee6Api

async def add_user(user):
    id = user['id']
    level = user['level']
    xp = int(int(user['xp']) - ((5/6)*level * (level + 7) * (2*level + 13)))
    sql = f"INSERT OR {method} INTO '{sys.argv[2]}_levels' values ({id}, {level}, {xp}, 0);"
    cur.execute(sql)

async def main():

    if (len(sys.argv)!=4 or sys.argv[3].lower() not in ['true', 'false']):
        print("Invalid usage!")
        print("Usage: python3 migrate_mee6.py <path/to/database.sqlite> <server_id> <should_overwrite>")
        sys.exit(1)

    con = sqlite3.connect(sys.argv[1])
    global cur 
    cur = con.cursor()

    global method
    method = "REPLACE" if sys.argv[3].lower() == 'true' else "IGNORE"

    mee6 = mee6Api(sys.argv[2])

    try:
        await mee6.levels.get_leaderboard_page(0)
    except:
        print("Invalid server id!")
        sys.exit(1)

    cur.execute(f"""
    CREATE TABLE IF NOT EXISTS '{sys.argv[2]}_levels' (
        user_id INTEGER PRIMARY KEY,
        level INTEGER NOT NULL DEFAULT 0,
        xp INTEGER NOT NULL DEFAULT 0,
        last_xp INTEGER NOT NULL DEFAULT 0);
    """)

    pages = await mee6.levels.get_all_leaderboard_pages()

    for page in pages:
        for user in page['players']:
            await add_user(user)

    con.commit()
    con.close()

asyncio.run(main());
