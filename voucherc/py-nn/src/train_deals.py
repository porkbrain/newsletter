# https://stackabuse.com/text-classification-with-python-and-scikit-learn

from sklearn.model_selection import train_test_split
from sklearn.feature_extraction.text import TfidfTransformer
from sklearn.feature_extraction.text import CountVectorizer
from nltk.stem import WordNetLemmatizer
import numpy as np
import re
import nltk
from sklearn.datasets import load_files
from sklearn.ensemble import RandomForestRegressor
from sklearn import metrics
import pickle
import datetime


def load_lines(f):
    return np.array(
        normalize(
            list(
                map(
                    lambda s: s.encode("utf-8"),
                    tuple(open(f, "r")),
                )
            )
        )
    )


def normalize(phrases):
    documents = []

    stemmer = WordNetLemmatizer()

    for sen in range(0, len(phrases)):
        document = str(phrases[sen]).lower()
        document = re.sub(r"\$", " USD ", document)
        document = re.sub(r"£", " GBP ", document)
        document = re.sub(r"€", " EUR ", document)
        document = re.sub(r"%", " PERCENT ", document)
        document = re.sub(r"\W", " ", document)
        document = re.sub(r"\s+[a-z]\s+", " ", document)
        document = re.sub(r"\^[a-z]\s+", " ", document)
        document = re.sub(r"\s+", " ", document)
        document = re.sub(r"^b\s+", "", document)

        documents.append(document)

    return documents


vectorizer = CountVectorizer(max_features=1500, min_df=0.005, max_df=0.8)
tfidfconverter = TfidfTransformer()
classifier = RandomForestRegressor(n_estimators=1000, n_jobs=16, verbose=1)

deals = load_lines("../data/deals.en.txt")
ndeals = load_lines("../data/ndeals.en.txt")

X = np.concatenate((deals, ndeals))
y = np.concatenate(
    (np.ones((1, deals.size)), np.zeros((1, ndeals.size))), axis=1
)[0]

rng_state = np.random.get_state()
np.random.shuffle(X)
np.random.set_state(rng_state)
np.random.shuffle(y)

X = vectorizer.fit_transform(X).toarray()
X = tfidfconverter.fit_transform(X).toarray()
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.1)

classifier.fit(X_train, y_train)

print()
y_pred = classifier.predict(X_test)
print("Mean Absolute Error:", metrics.mean_absolute_error(y_test, y_pred))
print("Mean Squared Error:", metrics.mean_squared_error(y_test, y_pred))
