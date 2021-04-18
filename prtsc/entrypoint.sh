#!/bin/bash

rm /tmp/.X99-lock &>/dev/null
Xvfb :99 &

DISPLAY=':99' geckodriver &

prtsc
