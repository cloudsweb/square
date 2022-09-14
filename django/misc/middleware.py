from io import StringIO
import logging
import json
from django.http.request import QueryDict, HttpRequest
from django.http.response import HttpResponse
# from django.core import exceptions
from django.utils.deprecation import MiddlewareMixin
from django.db.models import Model
from django.core.serializers import get_serializer

logger = logging.getLogger("misc.middleware")

object_types = {
  'application/json': 'json',
  'application/yaml': 'yaml',
  'application/xml': 'xml',
}

class LoginRequired(Exception):
    """The operator requires login"""
    pass

class HttpResponseUnauthorized(HttpResponse):
  status_code = 401

class ObjectResponse(HttpResponse):
  def __init__(self, obj, *args, **kwargs) -> None:
    # TODO check if dict
    # logger.debug(f"{type(obj)} => {obj}")

    if isinstance(obj, Model):
      content = self.serialize_single('json', obj)
    else:
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
    # if type in object_types:
    #   serialize_type = object_types[type]
    if type == "application/json":
      self.content = f'{{"code":{json.dumps(self.status_code)}, "data":{self.content.decode(self.charset)}}}'
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

      request.data = data

      if request.method == 'POST' and request.POST._mutable:
        request.POST.update(data)

  """
  Process application/json requests data for GET and POST response.
  """
  def process_response(self, request: HttpRequest, response: HttpResponse):
    content_type = request.META.get('CONTENT_TYPE')
    if content_type and 'application/json' in content_type:
      # logger.info(f"{content_type} => {response}")
      if isinstance(response, ObjectResponse): # TODO: payload?
        response.as_type("application/json")
      elif isinstance(response, HttpResponse):
        response.content = json.dumps({ 'msg': response.content.decode('utf8'), 'code': response.status_code })
        response.headers["Content-Type"] = "application/json"

    return response

  def process_exception(self, request: HttpRequest, exception: Exception):
    # TODO: separate middleware ExceptionMessageMiddleware
    if isinstance(exception, LoginRequired):
      return HttpResponseUnauthorized("login required") # this would auto forward to process_response
    else:
      logger.warn(f"unhandled exception: {type(exception)}")
