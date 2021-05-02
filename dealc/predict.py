"""
The template of the http server was taken from:
- https://gist.github.com/mdonkers/63e115cc0c79b4f6b8b3a6b797e485c7

POST /
    requestBody: string[]
    responseBody: float[]
    description: Given an array of strings, it predicts their class (deal x
        ndeal) and outputs an array of the same length as the input one
        with floats in interval [0; 1] which can be interpreted as likelihood
        of respective string being a useful deal.
"""

from os import getenv
from http.server import BaseHTTPRequestHandler, HTTPServer
from normalize import normalize_phrase
import json
import numpy as np
import pickle

# Loads model which should be packed together with this script. The model is
# produced by the train.py script in the same dir.
print("Loading model")
with open("model.data/model", "rb") as f:
    model = pickle.load(f)
with open("model.data/vectorizer", "rb") as f:
    vectorizer = pickle.load(f)
with open("model.data/tfidf", "rb") as f:
    tfidf = pickle.load(f)

model.n_jobs = int(getenv("WORKERS", "1"))
model.verbose = int(getenv("VERBOSITY", "0"))


class Predictor(BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers["Content-Length"])
        body = self.rfile.read(content_length).decode("utf-8")
        phrases = json.loads(body)
        print("Processing list of size", len(phrases))

        phrases = np.array(list(map(normalize_phrase, phrases)))
        phrases = vectorizer.transform(phrases)
        phrases = tfidf.transform(phrases)
        inferences = model.predict(phrases).flatten()

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


run(getenv("HTTP_HOST", "0.0.0.0"), int(getenv("HTTP_PORT", "8081")))
