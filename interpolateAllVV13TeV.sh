#!/bin/sh

for resonance in `seq  1000 100 4500`
do
    python interpolateVV13TeV.py input/ZprimeWW_13TeV_ $resonance GeV
    python interpolateVV13TeV.py input/WprimeWZ_13TeV_ $resonance GeV
    # python interpolateVV13TeV.py input/BulkZZ_13TeV_ $resonance GeV
    # python interpolateVV13TeV.py input/qW/QstarQW_13TeV_ $resonance GeV &
    # python interpolateVV13TeV.py input/qZ/QstarQZ_13TeV_ $resonance GeV &
    # python interpolateVV13TeV.py input/BulkWW_13TeV_ $resonance GeV
    # python interpolateVV13TeV.py input/RS1WW_13TeV_ $resonance GeV &
    # python interpolateVV13TeV.py input/RS1ZZ_13TeV_ $resonance GeV &
done
