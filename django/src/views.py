# types
from django.http import HttpRequest, HttpResponse
# functions
from misc.middleware import ObjectResponse
from .admin import login_required
from django.contrib.auth import authenticate, login
from django.views.generic.base import View
# decorators
from django.utils.decorators import method_decorator
from django.views.decorators.http import require_POST

# Create your views here.
def index(request: HttpRequest):
  return ObjectResponse("Hello, world. You're at the index.")

@require_POST
def signin(request: HttpRequest):
  username = request.POST.get('username', None) or request.POST.get('alias', None)
  password = request.POST.get('password', None)
  user = authenticate(request, username=username, password=password)
  if user is not None:
    login(request, user)
    return ObjectResponse("success")
  return ObjectResponse("failed", status=403)

class PostView(View):

  @method_decorator(login_required)
  def post(request: HttpRequest, *_, id: str):
    return ObjectResponse("method not supported", status=405)

  def get(request: HttpRequest, *_, id: str):
    return ObjectResponse(f"show post <{id}>")
