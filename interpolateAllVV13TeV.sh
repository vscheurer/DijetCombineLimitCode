#!/bin/sh

for resonance in `seq  1000 50 4000`
do
    python interpolateVV13TeV.py input/RS1WW_13TeV_ $resonance GeV &
    python interpolateVV13TeV.py input/RS1ZZ_13TeV_ $resonance GeV &
    python interpolateVV13TeV.py input/BulkWW_13TeV_ $resonance GeV &
done
