#!/bin/sh

for resonance in `seq  950 50 4500`
do
    python interpolateVV13TeV.py input/WprimeToWZ_13TeV_ $resonance GeV &
done
