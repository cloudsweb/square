# %%
import records
import dataclasses
from dataclasses import dataclass
import sqlalchemy
import logging, time
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger("DB")
DbConn = records.Database
Null = set([None])

class DbData:
  @classmethod
  def table_name(cls):
    if '_' in cls.__name__:
      return cls.__name__.split('_', 1)[0]
    raise NotImplemented
  @classmethod
  def keys(cls):
    return [i.name for i in dataclasses.fields(cls)]
  def as_dict(self):
    return dataclasses.asdict(self)
  @staticmethod
  def _add_limit(stmt: str, limit:int=None, offset:int=None):
    if limit is not None:
      assert isinstance(limit, int)
      stmt += f" LIMIT {limit}"
    if offset is not None:
      assert isinstance(offset, int)
      stmt += f" OFFSET {offset}"
      return stmt
  def _add_filter(self, stmt: str):
    keys = self.keys()
    stmt += f" WHERE {','.join([f'{k}=:{k}' for k in keys])}"
    return stmt
  @classmethod
  def _query(cls, db: DbConn, stmt: str, data: dict = None):
    if data is None:
      data = {}
    start = time.perf_counter()
    rows = db.query(stmt, **data).all(as_dict=True)
    end = time.perf_counter()
    logger.info(f"query `{stmt}` with {data} returns {len(rows)} rows in {end-start}s")
    return [cls(**row) for row in rows]
  def insert(self, db: DbConn):
    keys = self.keys()
    s = f"INSERT INTO {self.table_name()} ({','.join(keys)}) VALUES ({','.join([':'+k for k in keys])})"
    try:
      self._query(db, s, self.as_dict())
    except sqlalchemy.exc.ResourceClosedError as e:
      # https://github.com/kennethreitz/records/issues/208
      raise e
      if e.args == ('This result object does not return rows. It has been closed automatically.',):
        return
  @classmethod
  def select(cls, db: DbConn, *, limit=10, offset=None, filter:"DbData"=None):
    keys = cls.keys()
    s = f"SELECT {','.join(keys)} FROM {cls.table_name()}"
    cls._add_limit(s, limit, offset)
    if filter is None:
      rows = cls._query(db, s)
    else:
      rows = cls._query(db, filter._add_filter(s), filter.as_dict())
    return rows
db = records.Database("postgresql://localhost:7039/posts")
db.get_table_names()

# %%
from datetime import datetime
@dataclass
class users_Create(DbData):
  name: str
  alias: str

def create_user(name: str, db: DbConn):
  return users_Create(name, name).insert(db)

@dataclass
class users_Get(DbData):
  id: int
  alias: str
  name: str
  description: str
  avatar: str
  inserted_at: datetime
  updated_at: datetime

@dataclass
class users_Ref(DbData):
  id: int

@dataclass
class users_SearchName(DbData):
  name: str

@dataclass
class posts_Get(DbData):
  author_id: users_Ref

# %%
for i in range(10):
  name = f"test2{i}"
  id = users_Ref.select(db, filter=users_SearchName(name))
  if len(id) == 0:
    create_user(name, db)
users_Get.select(db)
None

# %%
