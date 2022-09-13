from django.test import TestCase, Client

# Create your tests here.

class Test(TestCase):
  fixtures = ["fixtures.yaml"]

  def setUp(self):
    self.c = Client(content_type="application/json")

  def test_index(self):
    resp = self.c.get("/")
    self.assertJSONEqual(resp.content, {"data": "Hello, world. You're at the index."})

    resp = self.c.get("/post/1")
    self.assertJSONEqual(resp.content, {'data': 'show post <1>'})

    resp = self.c.post("/post/1")
    self.assertEqual(resp.status_code, 403)
    # self.assertJSONEqual(resp.content, {'data': 'show post <1>'})
    # TODO: test 404

    resp = self.c.post("/signin", data={ "username": "not-exists", "password": "password" })
    self.assertEqual(resp.status_code, 403)
    resp = self.c.post("/signin", data={ "username": "test", "password": "" })
    self.assertEqual(resp.status_code, 403)

    resp = self.c.post("/signin", data={ "username": "test", "password": "password" })
    self.assertEqual(resp.status_code, 200)

    resp = self.c.post("/post/1")
    self.assertEqual(resp.status_code, 405)
