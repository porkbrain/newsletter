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
        document = re.sub(r"\W", " ", str(phrases[sen]))
        document = re.sub(r"\s+[a-zA-Z]\s+", " ", document)
        document = re.sub(r"\^[a-zA-Z]\s+", " ", document)
        document = re.sub(r"\s+", " ", document, flags=re.I)
        document = re.sub(r"^b\s+", "", document)
        document = document.lower()

        documents.append(document)

    return documents


vectorizer = CountVectorizer(max_features=1500, min_df=0.005, max_df=0.8)
tfidfconverter = TfidfTransformer()
classifier = RandomForestRegressor(n_estimators=1000, random_state=0, n_jobs=16)

deals = load_lines("../data/deals.txt")
ndeals = load_lines("../data/ndeals.txt")

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
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.1, random_state=0
)

classifier.fit(X_train, y_train)

y_pred = classifier.predict(X_test)
print("Mean Absolute Error:", metrics.mean_absolute_error(y_test, y_pred))
print("Mean Squared Error:", metrics.mean_squared_error(y_test, y_pred))
print(
    "Root Mean Squared Error:",
    np.sqrt(metrics.mean_squared_error(y_test, y_pred)),
)

"""
test_lines = load_lines("../data/ndeals2.txt")
y_test = np.zeros((1, test_lines.size))[0]
X_test = vectorizer.transform(test_lines).toarray()
X_test = tfidfconverter.transform(X_test).toarray()

y_pred = classifier.predict(X_test)
for x in range(0, len(y_pred)):
    if y_pred[x] == 1.0:
        print(x, test_lines[x])
print(confusion_matrix(y_test, y_pred))
print(accuracy_score(y_test, y_pred))
"""
