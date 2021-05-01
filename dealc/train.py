# https://stackabuse.com/text-classification-with-python-and-scikit-learn

from normalize import normalize_phrase
from sklearn import metrics
from sklearn.datasets import load_files
from sklearn.ensemble import RandomForestRegressor
from sklearn.feature_extraction.text import TfidfTransformer, CountVectorizer
from sklearn.model_selection import train_test_split
import datetime
import numpy as np
import pickle
import re
from zipfile import ZipFile, ZIP_DEFLATED

"""
# random forest train script
We use random forest regressor for binary classification of phrases on deals and
not deals. Each string phrase is firstly normalized (stripped of non word
characters etc) and then fed into a vectorizer. Then we use term-frequency
inverse-document-frequency transformer to pick out words with highest entropy.

## 1.
Loads phrases from data sets and normalizes each phrase. Then labels are created
for the input data and the data are shuffled.

## 2.
The phrases are transformed into floats, training and testing data are separated
out and the model is trained.

## 3.
Model is evaluated with Mean Absolute Error and Mean Squared Error. If I
understand correctly, the MAE is a better metric since the output is in interval
[0; 1]. However, when I tried to set this as a criterion for the regressor, it
got stuck (perhaps a bug in the library?).

Promt is displayed to the user to confirm saving of the model.
"""

vectorizer = CountVectorizer(max_features=1500, min_df=0.005, max_df=0.8)
tfidf = TfidfTransformer()
model = RandomForestRegressor(n_estimators=1000, n_jobs=16, verbose=1)

# 1.
def load_lines(path):
    return np.array(
        list(
            map(
                lambda s: normalize_phrase(str(s.encode("utf-8"))),
                tuple(open(path, "r")),
            )
        )
    )


deals = load_lines("data/deals.en.txt")
ndeals = load_lines("data/ndeals.en.txt")

X = np.concatenate((deals, ndeals))
y = np.concatenate(
    (np.ones((1, deals.size)), np.zeros((1, ndeals.size))), axis=1
)[0]

rng_state = np.random.get_state()
np.random.shuffle(X)
np.random.set_state(rng_state)
np.random.shuffle(y)

# 2.
X = vectorizer.fit_transform(X).toarray()
X = tfidf.fit_transform(X).toarray()
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.1)
model.fit(X_train, y_train)

# 3.
print()
y_pred = model.predict(X_test)
print()
print("MAE:", metrics.mean_absolute_error(y_test, y_pred))
print("MSE:", metrics.mean_squared_error(y_test, y_pred))

print()
store_yn = input("Store model? ")
if store_yn[0].lower() == "y":
    with open("model.data/model", "wb") as f:
        pickle.dump(model, f)

    with open("model.data/vectorizer", "wb") as f:
        pickle.dump(vectorizer, f)

    with open("model.data/tfidf", "wb") as f:
        pickle.dump(tfidf, f)

    with ZipFile("model.data.zip", mode="w", compression=ZIP_DEFLATED) as zipf:
        zipf.write("model.data/model")
        zipf.write("model.data/vectorizer")
        zipf.write("model.data/tfidf")

    print("Done")
else:
    print("Nothing to do")
