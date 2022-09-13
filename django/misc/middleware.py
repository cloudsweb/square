import json
# from django.core.serializers.json import DjangoJSONEncoder, Deserializer
from django.http.request import QueryDict, HttpRequest
from django.http.response import HttpResponse, HttpResponseBase, JsonResponse
from django.utils.deprecation import MiddlewareMixin

class ObjectResponse(HttpResponse):
  def __init__(self, obj, *args, **kwargs) -> None:
    # TODO check if dict
    if not isinstance(obj, dict):
      obj = { 'data': obj }
    super().__init__(json.dumps(obj), *args, **kwargs)
    self.item = obj

  def as_type(self, type):
    if type == "application/json":
      self.headers.setdefault("Content-Type", type)

class JsonMiddleware(MiddlewareMixin):
  """
  Process application/json requests data from GET and POST requests.
  """
  def process_request(self, request: HttpRequest):
    content_type = request.META.get('CONTENT_TYPE')
    if content_type and 'application/json' in content_type:
      # load the json data
      data = None
      if request.body:
        data = json.loads(request.body) # Deserializer

      if not data:
        return

      if request.method == 'GET':
        if not request.GET._mutable:
          request.GET = QueryDict(mutable=True)
        request.GET.update(data)

      if request.method == 'POST':
        if not request.POST._mutable:
          request.POST = QueryDict(mutable=True)
        request.POST.update(data)

  """
  Process application/json requests data for GET and POST response.
  """
  def process_response(self, request: HttpRequest, response: HttpResponse):
    content_type = request.META.get('CONTENT_TYPE')
    if content_type and 'application/json' in content_type:
      if isinstance(response, ObjectResponse): # TODO: payload?
        response.as_type("application/json")
      elif isinstance(response, HttpResponse):
        response.content = json.dumps({ 'data': response.content.decode('utf8') })
        response.headers.setdefault("Content-Type", "application/json")

    return response
