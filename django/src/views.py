from django.shortcuts import render
from django.http import HttpRequest, HttpResponse

# Create your views here.
def index(request: HttpRequest):
  return HttpResponse("Hello, world. You're at the index.")

def create_user(request: HttpRequest):
  if request.method != 'POST':
    return HttpResponse("method not supported", status=405)
  # TODO: parse request.body
