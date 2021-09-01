# dealc
Simple HTTP server in `predict.py` uses a model trained by `train.py` which
gives an estimate about how likely a string is an interesting deal worth
exporting (or including a voucher code).


## Installation
Create a new virtual environment:

```bash
python3 -m venv deps
```

To active the environment:

```bash
source deps/bin/activate
```

Install following packages:

```bash
pip3 install numpy tensorflow keras sklearn black
```

## Training

```bash
python3 train.py
```

This writes a model into `model.data` and zips it into `model.data.zip`. The
Dockerfile then uses this stored model to run an http server which predicts how
likely given string is a deal.

## Troubleshooting
> ModuleNotFoundError: No module named 'pip'

`$ python -m ensurepip` from [https://stackoverflow.com/a/61562956/5093093]
