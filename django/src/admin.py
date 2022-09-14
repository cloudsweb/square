from django.contrib import admin

# Register your models here.

from .models_gen import Users, Secrets

from typing import Optional
import functools
from misc.middleware import LoginRequired
from django.contrib.auth.backends import BaseBackend
from django.contrib.auth.models import User

def login_required(f):
  @functools.wraps(f)
  def wrapped(request, *args, **kwargs):
    if not request.user.is_authenticated:
      raise LoginRequired
    return f(request, *args, **kwargs)
  return wrapped

class AuthBackend(BaseBackend):
  def _check_key(self, secret: Secrets, password):
    # TODO sha256("{salt}${password}")
    return secret.salt == '' and secret.current == password

  def _get_auth_user(self, user: Users, secret: Optional[Secrets] = None):
    try:
      secret = secret or Secrets._default_manager.get(pk=user.id)
    except Secrets.DoesNotExist:
      password = ""
    else:
      password = secret.current
    # auth_user, _ = User._default_manager.get_or_create(
    #   username=f"@{user.id}",
    #   defaults = {
    #     "email": f"{user.alias}@main",
    #     "first_name": user.name,
    #     "password": password
    #   })
    auth_user, _ = User._default_manager.get_or_create(username=f"@{user.id}")
    auth_user._user = user
    auth_user.email = f"{user.alias}@main"
    auth_user.first_name = user.name
    auth_user.password = password
    return auth_user

  def authenticate(self, request, username=None, password=None, **kwargs):
    if username is None or password is None or password == '':
      return
    try:
      user = Users._default_manager.get(alias=username)
      secret = Secrets._default_manager.get(id=user.id)
    except (Users.DoesNotExist, Secrets.DoesNotExist):
      return
    else:
      auth_user = self._get_auth_user(user, secret)
      if self._check_key(secret, password):
        return auth_user

  def get_user(self, user_id):
    try:
      user = Users._default_manager.get(pk=user_id)
    except Users.DoesNotExist:
      return None
    return self._get_auth_user(user)
