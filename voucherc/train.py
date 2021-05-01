import os

# don't print warnings about CPU
os.environ["TF_CPP_MIN_LOG_LEVEL"] = "2"

import numpy as np
from keras.models import Sequential, save_model
from keras.layers import Dense
from features import feature_from_str

"""
# neural network train script
We use multi layer neural nets as a binary categorizer for words to determine
whether a word is likely to be voucher or not.

## 1.
This script takes as an input two files:
    a. one word which is a voucher per line
    b. one word which isn't a vouchers per line

We then split the dataset in two: training and validation sets.

## 2.
We prepare the training input data X and their labels y.

## 3.
We set up the dnn model and train it.

## 4.
We validate the model on vouchers and nvouchers

## 5.
We evaluate the model on the testing data and prompt user to save the model.
"""

epochs = 120
batch_size = 300


def load_txt_and_convert_to_features(path):
    fts = []

    data = open(path, "r")
    for line in data.readlines():
        word = line.strip()
        fts.append(np.array(feature_from_str(word)))
    data.close()

    fts = np.stack(fts)
    np.random.shuffle(fts)
    return fts


# 1.
vouchers = load_txt_and_convert_to_features("data/vouchers.txt")
nvouchers = load_txt_and_convert_to_features("data/nvouchers.txt")

(vouchers_count, vfeatures_count) = vouchers.shape
(nvouchers_count, nvfeatures_count) = nvouchers.shape

assert vfeatures_count == nvfeatures_count
features_count = vfeatures_count

train_count = round(min(vouchers_count * 0.9, nvouchers_count * 0.9))
train_vouchers_count = train_count
train_nvouchers_count = train_count

# 2.
X = np.concatenate(
    (
        vouchers[:train_vouchers_count],
        nvouchers[:train_nvouchers_count],
    ),
    axis=0,
)
y = np.concatenate(
    (np.ones((1, train_vouchers_count)), np.zeros((1, train_nvouchers_count))),
    axis=1,
).flatten()
print("Training data shape", X.shape, y.shape)

# 3.
print()
model = Sequential()
model.add(Dense(48, input_dim=features_count, activation="relu"))
model.add(Dense(24, activation="relu"))
model.add(Dense(12, activation="relu"))
model.add(Dense(1, activation="sigmoid"))

model.compile(
    loss="binary_crossentropy", optimizer="adam", metrics=["accuracy"]
)

model.fit(X, y, epochs=epochs, batch_size=batch_size, shuffle=True)

# 4.
print()
_, vaccuracy = model.evaluate(
    vouchers[train_vouchers_count:],
    np.ones((1, vouchers_count - train_vouchers_count)).flatten(),
)
print("Accuracy on vouchers: %.2f" % (vaccuracy * 100))
_, nvaccuracy = model.evaluate(
    nvouchers[train_nvouchers_count:],
    np.zeros((1, nvouchers_count - train_nvouchers_count)).flatten(),
)
print("Accuracy on nvouchers: %.2f" % (nvaccuracy * 100))

# 5.
print()
store_yn = input("Store model? ")
if store_yn[0].lower() == "y":
    save_model(model, "model.data")
    print("Done")
else:
    print("Nothing to do")
