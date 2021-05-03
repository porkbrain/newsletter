FROM python:3.8-buster

RUN pip3 install numpy tensorflow keras

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

COPY predict.py predict.py
COPY features.py features.py
COPY model.data model.data

RUN mkdir data
COPY data/dictionary.en.txt data/dictionary.en.txt

CMD python3 predict.py
