#!/bin/bash

srun --partition cpunodes -c 16 --mem=32G --pty run.sh $1
