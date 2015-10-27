#!/bin/sh

for resonance in `seq  1000 50 4500`
do
    python interpolateVV13TeV.py input/WprimeToWZ_ $resonance GeV &
done
