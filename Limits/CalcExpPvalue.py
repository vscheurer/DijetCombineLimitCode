from ROOT import *
import ROOT
import array, math
import os

gROOT.Reset()
gROOT.SetStyle("Plain")
gStyle.SetOptStat(0)
gStyle.SetOptFit(0)
gStyle.SetTitleOffset(1.2,"Y")
gStyle.SetPadLeftMargin(0.18)
gStyle.SetPadBottomMargin(0.15)
gStyle.SetPadTopMargin(0.03)
gStyle.SetPadRightMargin(0.05)
gStyle.SetMarkerSize(1.5)
gStyle.SetHistLineWidth(1)
gStyle.SetStatFontSize(0.020)
gStyle.SetTitleSize(0.06, "XYZ")
gStyle.SetLabelSize(0.05, "XYZ")
gStyle.SetNdivisions(510, "XYZ")
gStyle.SetLegendBorderSize(0)

channel=[0]
sigWW=[]
pvalWW=[]

sigZZ=[]
pvalZZ=[]

sChan=["ZZ"]
#bin=[0,1,2,3,4,5]
bin=["012"]

#masses =[1400.0, 1500.0, 1600.0, 1700.0, 1800.0, 1900.0, 2000.0, 2100.0, 2200.0, 2300.0, 2400.0, 2500.0, 2600.0, 2700.0]
masses = [1500.0,1700.0, 1800.0,1900.0, 2000.0,2100.0, 2300.0,2500.0, 2700.0,3000.0,3500.0,4000.0]
#masses = [3000.0,3500.0,4000.0]

toysPerJob=300
jobs=1

for ibin in bin:
    for chan in channel:
        for dmass in masses:
	  for job in range(jobs):
            mass = int(dmass)  
            outputname = "pvalue"+str(mass)+"_"+sChan[chan]+".src"
            logname = "pvalue"+str(mass)+"_"+sChan[chan]+".out"
            outputfile = open(outputname,'w')
            outputfile.write('#!/bin/bash\n')
            outputfile.write("cd ..; eval `scramv1 run -sh`\n")
            outputfile.write("combine datacards/CMS_jj_BulkZZ_"+str(mass)+"_13TeV_CMS_jj_comb_bb.txt -M ProfileLikelihood -v2 -m "+str(mass) + " --signif --expectSignal=1 -t -1 -s "+str(job)+"\n")
            
            outputfile.close()
  
            #command="rm "+logname
	    #print command
            #os.system(command)
            command = "combine ../datacards/CMS_jj_BulkZZ_"+str(mass)+"_13TeV_CMS_jj_comb_bb.txt -M ProfileLikelihood -v2 -m "+str(mass) +"  --signif  --expectSignal=1 -t -1 "
            #command="bsub -q 1nh -o "+logname+" source "+outputname
	    print command
            os.system(command)
