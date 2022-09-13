from django.test import TestCase, Client

# Create your tests here.

class Test(TestCase):
  def setUp(self):
    self.c = Client(content_type="application/json")

  def test_index(self):
    resp = self.c.get("/")
    self.assertJSONEqual(resp.content, {"data": "Hello, world. You're at the index."})

    # TODO: test 404
