from django.urls import path

from . import views

urlpatterns = [
  path('', views.index),
  path('signin', views.signin),
  path('post/new', views.PostView._post),
  path('post/<id>', views.PostView.as_view()),
]
