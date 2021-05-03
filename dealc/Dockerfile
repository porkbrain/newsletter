FROM python:3.8-buster

RUN pip3 install numpy tensorflow keras
# install it separately because the above might be already cached for voucherc
RUN pip3 install sklearn

RUN apt install unzip

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

COPY predict.py predict.py
COPY normalize.py normalize.py

COPY model.data.zip model.data.zip
RUN mkdir model.data
RUN unzip model.data.zip
RUN rm model.data.zip

CMD python3 predict.py
