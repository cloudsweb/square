# types
import uuid
from django.http import HttpRequest, HttpResponse
# functions
from misc.middleware import ObjectResponse
from .admin import login_required
from django.contrib.auth import authenticate, login
from django.views.generic.base import View
from .models_gen import Posts, Users
from django.http.response import HttpResponseNotFound, HttpResponseForbidden
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

  @login_required
  @staticmethod
  def _post(request: HttpRequest):
    user: Users = request.user._user
    title = request.POST.get('title', None)
    content = request.POST.get('content', None)
    post = Posts._default_manager.create(
      id=uuid.uuid4().hex,
      topic_id=0,
      author=user,
      author_name=user.name,
      title=title,
      content=content,
      revision=0,
      floor=0,
    )
    return ObjectResponse({ "msg": "created", "id": post.id }, status=200)

  @method_decorator(login_required)
  def put(self, request: HttpRequest, *_, id: str):
    user: Users = request.user._user
    post = Posts.objects.get(id=id)
    if post.author.id != user.id:
      return HttpResponseForbidden(id)
    title = request.data.get('title', None)
    content = request.data.get('content', None)
    columns = []
    if title is not None:
      post.title = title
      columns.append('title')
    if content is not None:
      post.content = content
      columns.append('content')
    post.save(columns=columns)
    return ObjectResponse({ "msg": "updated", "id": post.id }, status=200)

  def get(self, request: HttpRequest, *_, id: str):
    try:
      post = Posts.objects.get(id=id)
    except Posts.DoesNotExist:
      return HttpResponseNotFound(id)
    return ObjectResponse(post)
