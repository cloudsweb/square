from io import StringIO
# import logging
import json
# from django.core.serializers.json import DjangoJSONEncoder, Deserializer
from django.http.request import QueryDict, HttpRequest
from django.http.response import HttpResponse, HttpResponseBase, JsonResponse
from django.utils.deprecation import MiddlewareMixin
from django.db.models import Model
from django.core.serializers import get_serializer

class ObjectResponse(HttpResponse):
  def __init__(self, obj, *args, **kwargs) -> None:
    # TODO check if dict
    # logging.info(f"{type(obj)} => {obj}")

    if isinstance(obj, Model):
      content = self.serialize_single('json', obj)
    else:
      if not isinstance(obj, dict):
        obj = { 'data': obj }
      content = json.dumps(obj)
    super().__init__(content, *args, content_type='text/plain', **kwargs)
    self.item = obj

  def serialize_single(self, format, obj, **kwargs):
    serializer = get_serializer(format)()
    def wrap(s, func):
      def wrapped(*args, **kwargs):
        s.stream, old_stream = StringIO(), s.stream
        result = func(*args, **kwargs)
        s.stream = old_stream
        return result
      return wrapped
    serializer.start_serialization = wrap(serializer, serializer.start_serialization)
    serializer.end_serialization = wrap(serializer, serializer.end_serialization)
    return serializer.serialize([obj], **kwargs)

  def as_type(self, type):
    if type == "application/json":
      self.headers["Content-Type"] = type

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
      # logging.info(f"{content_type} => {response}")
      if isinstance(response, ObjectResponse): # TODO: payload?
        response.as_type("application/json")
      elif isinstance(response, HttpResponse):
        response.content = json.dumps({ 'msg': response.content.decode('utf8'), 'code': response.status_code })
        response.headers["Content-Type"] = "application/json"

    return response
