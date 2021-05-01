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

