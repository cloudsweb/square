from settings import *

DATABASES = {
  'default': {
    'ENGINE': 'django.db.backends.sqlite3',
    'NAME': BASE_DIR / '../data/django.sqlite3',
  },
  'data': {
    'ENGINE': 'django.db.backends.postgresql_psycopg2',
    'NAME': 'posts',
    'HOST': 'localhost',
    'PORT': 7039,
  },
}

DATABASE_MAIN = 'data'
