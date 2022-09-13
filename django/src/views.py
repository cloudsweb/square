# types
from django.http import HttpRequest, HttpResponse
# functions
from misc.middleware import ObjectResponse
from django.contrib.auth import authenticate, login
from django.views.generic.base import View
# decorators
from django.contrib.auth.decorators import login_required
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

class PostView(View):

  @login_required
  def post(request: HttpRequest):
    return ObjectResponse("method not supported", status=405)

  def get(request: HttpRequest):
    return ObjectResponse("show post")
