from django.db import models
from . import models_gen

# Create your models here.

from .models_gen import *

class Router:
  from django.conf import settings
  route_app_labels = [ "main" ]
  route_app_db = settings.DATABASE_MAIN

  def db_for_read(self, model, **hints):
    if model._meta.app_label in self.route_app_labels:
      return self.route_app_db
    return None

  def db_for_write(self, model, **hints):
    if model._meta.app_label in self.route_app_labels:
      return self.route_app_db
    return None

  def allow_relation(self, obj1, obj2, **hints):
    if (
      obj1._meta.app_label in self.route_app_labels and
      obj2._meta.app_label in self.route_app_labels
    ):
      return True
    return None

  def allow_migrate(self, db, app_label, model_name=None, **hints):
    if db == self.route_app_db and db == 'default':
      return None
    if app_label in self.route_app_labels:
      return db == self.route_app_db and db == 'default'
    if db == self.route_app_db:
      return app_label in self.route_app_labels and db == 'default'
    return None
