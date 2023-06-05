#!/usr/bin/env sh

PID=$(cat /var/reflexive/reflexive.pid)

if kill -9 $PID > /dev/null 2> /dev/null; then
    echo "The service process ${PID} is killed."
else
    killall reflexive
    echo 'Killed all reflexive process'
fi
