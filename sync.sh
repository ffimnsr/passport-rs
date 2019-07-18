#!/usr/bin/env bash

set -e

ag -l | entr rsync -avz --delete --exclude .git --exclude .DS__Store --exclude target . ubuntu@ec2-18-216-154-77.us-east-2.compute.amazonaws.com:~/bbb/pass
