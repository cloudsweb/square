from django.contrib import admin

# Register your models here.

from .models_gen import Users, Secrets

from django.contrib.auth.backends import BaseBackend

class AuthBackend(BaseBackend):
  def authenticate(self, request, username=None, password=None, **kwargs):
    if username is None or password is None or password == '':
      return
    try:
      user: Users = Users._default_manager.get(alias=username)
      secret: Secrets = Secrets._default_manager.get(id=user.id)
    except (Users.DoesNotExist, Secrets.DoesNotExist):
      return
    else:
      if secret.salt == '' and secret.current == password: # TODO sha256("{salt}${password}")
        return user

  def get_user(self, user_id):
    try:
      user = Users._default_manager.get(pk=user_id)
    except Users.DoesNotExist:
      return None
    return user
