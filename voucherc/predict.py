"""
The template of the http server was taken from:
- https://gist.github.com/mdonkers/63e115cc0c79b4f6b8b3a6b797e485c7

POST /
    requestBody: string[]
    responseBody: float[]
    description: Given an array of strings, it predicts their class (voucher
        x nvoucher) and outputs an array of the same length as the input one
        with floats in interval [0; 1] which can be interpreted as likelihood
        of respective string being a voucher.
"""

import os

# don't print warnings about CPU
os.environ["TF_CPP_MIN_LOG_LEVEL"] = "2"

import numpy as np
from keras.models import load_model
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
from features import feature_from_str

# loads model which should be packed together with this script
# the model is produced by the train.py script in the same dir
print("Loading model")
model = load_model("model.data")


class Predictor(BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers["Content-Length"])
        body = self.rfile.read(content_length).decode("utf-8")
        words = json.loads(body)
        print("Processing list of size", len(words))

        fts = np.stack(list(map(feature_from_str, words)))
        inferences = model.predict(fts).flatten()

        resp = json.dumps(list(inferences.astype(float)))

        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.end_headers()
        self.wfile.write(resp.encode("utf-8"))


def run(host, port):
    server_address = (host, port)
    httpd = HTTPServer(server_address, Predictor)
    print("Starting http server on", server_address)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    httpd.server_close()
    print("Stopping http server")


run(os.getenv("HTTP_HOST", "0.0.0.0"), int(os.getenv("HTTP_PORT", "8080")))
