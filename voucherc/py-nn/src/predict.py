import os
import sys

# don't print warnings about CPU
os.environ["TF_CPP_MIN_LOG_LEVEL"] = "2"

import numpy as np
from keras.models import load_model

# path to the folder created by keras.models.save_model
model_path = sys.argv[1]

# parent process passes csv as stdin
words = np.loadtxt(sys.stdin, delimiter=",", ndmin=2)

# it'd be nice to run this as an http server, because loading the model takes
# a few seconds
model = load_model(model_path)

# make a prediction for each word and print it in stdout
prediction = model.predict(words).flatten()

print("\n".join([str(v) for v in prediction[0:]]))
