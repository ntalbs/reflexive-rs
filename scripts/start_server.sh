#!/usr/bin/env sh

cd /var/reflexive
./reflexive > /dev/null 2> /dev/null < /dev/null &

echo $! > /var/reflexive/reflexive.pid
echo "Server started"
