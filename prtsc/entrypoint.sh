#!/bin/bash

export DISPLAY=':99'

rm /tmp/.X99-lock &>/dev/null
Xvfb ${DISPLAY} &

geckodriver --host 0.0.0.0 &

# wait a bit for geckodriver http server to boot
sleep 3
prtsc
