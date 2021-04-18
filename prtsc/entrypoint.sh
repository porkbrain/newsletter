#!/bin/bash

rm /tmp/.X99-lock &>/dev/null
Xvfb :99 &

DISPLAY=':99' geckodriver &

# wait a bit for geckodriver http server to boot
sleep 5
prtsc
