from django.test import TestCase, Client
import logging

# Create your tests here.
logger = logging.getLogger("src.test")
logging.basicConfig(level=logging.INFO)

class JsonClient(Client):
  def get(self, path: str, data=None, content_type="application/json", **kwargs):
    return super().get(path, data=data, CONTENT_TYPE=content_type, **kwargs)
  def post(self, path: str, data=None, content_type="application/json", **kwargs):
    return super().post(path, data=data, content_type=content_type, **kwargs)
  def put(self, path: str, data=None, content_type="application/json", **kwargs):
    return super().put(path, data=data, content_type=content_type, **kwargs)
  def patch(self, path: str, data=None, content_type="application/json", **kwargs):
    return super().patch(path, data=data, content_type=content_type, **kwargs)

class Test(TestCase):
  fixtures = ["fixtures.yaml"]

  def setUp(self):
    self.c = JsonClient(content_type="application/json")
    logger.info(f"client with parameters {self.c.defaults}")

  def test_index(self):
    resp = self.c.get("/")
    self.assertJSONEqual(resp.content, {'code': 200, "data": "Hello, world. You're at the index."})

    resp = self.c.get("/post/0e7f1677-d1dd-4e46-add6-1861e8debacc")
    self.assertJSONEqual(resp.content, {'code': 404, 'msg': '0e7f1677-d1dd-4e46-add6-1861e8debacc'})

    resp = self.c.post("/post/new")
    self.assertEqual(resp.status_code, 401)
    self.assertJSONEqual(resp.content, {'code': 401, 'msg': 'login required'})
    # TODO: test 404

    resp = self.c.post("/signin", data={ "username": "not-exists", "password": "password" })
    self.assertEqual(resp.status_code, 401)
    self.assertJSONEqual(resp.content, {'code': 401, 'msg': 'login failed'})
    resp = self.c.post("/signin", data={ "username": "test", "password": "" })
    self.assertEqual(resp.status_code, 401)
    self.assertJSONEqual(resp.content, {'code': 401, 'msg': 'login failed'})

    resp = self.c.post("/signin", data={ "username": "test", "password": "password" })
    self.assertEqual(resp.status_code, 200)
    self.assertJSONEqual(resp.content, {'code': 200, 'data': 'success'})

    resp = self.c.post("/post/new", data={"title": "greet", "content": "hello world"})
    self.assertEqual(resp.status_code, 200)
    post_id = resp.json()['data']['id']
    logging.info(f"post created {post_id}")

    resp = self.c.get(f"/post/{post_id}")
    self.assertEqual(resp.json()['data']['pk'].replace('-', ''), post_id.replace('-', ''))
