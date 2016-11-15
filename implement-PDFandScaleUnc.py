from array import array
import sys
import ROOT
from ROOT import *

import os,commands, os.path
import fileinput


def get_xsec_unc(mass):
   uncs = {}
   fin = TFile.Open('EXOVVSystematics/xsec-unc-13TeV.root','READ')   
   cin = fin.Get('c')
   for p in cin.GetListOfPrimitives():
    if p.InheritsFrom("TMultiGraph"):
     for g in p.GetListOfGraphs(): uncs[g.GetName()] = g.Eval(mass) 
   fin.Close() 
   return uncs
   

prefixDCin  = "datacards_withJESJMSunc/CMS_jj_"
prefixDCout = "datacards/CMS_jj_"

signals=["BulkWW","BulkZZ","WZ","ZprimeWW"]
masses =[m*100 for m in range(10,42+1)]
purities = ["LP","HP"]
channels = ["WW","WZ","ZZ"]

# signals=["WZ"]
# masses = [1200]
     
for mass in masses:   
 xsecUnc      =  get_xsec_unc(mass)
 
 pdf_Wprime   = 1+xsecUnc['qq_PDF_Wprime']
 pdf_Zprime   = 1+xsecUnc['qq_PDF_Zprime']
 scale_Wprime = 1+xsecUnc['qq_scale_Wprime']
 scale_Zprime = 1+xsecUnc['qq_scale_Zprime']
 pdf_Bulk     = 1+xsecUnc['gg_PDF']
 scale_Bulk   = 1+xsecUnc['gg_scale'] 
 
 for signal in signals:
   
   
   pdf   = pdf_Wprime
   scale = scale_Wprime
   
   if signal.find("Bulk") != -1:
     newline1 = '\nCMS_XS_gg_PDF                lnN				    '
     newline2 = '\nCMS_XS_gg_scale              lnN				    '
     pdf   =  pdf_Bulk  
     scale =  scale_Bulk
   else: 
     newline1 = '\nCMS_XS_qq_PDF                lnN				    '
     newline2 = '\nCMS_XS_qq_scale              lnN				    '
     if signal.find("Zprime") != -1:
       pdf   = pdf_Zprime
       scale = scale_Zprime
       
 
 
   newline1+="%.3f        %.3f        -" %(pdf   ,pdf)
   newline2+="%.3f        %.3f        -" %(scale ,scale)
   
   print newline1
   print newline2
   
   for purity in purities:
     for ch in channels:
       
       fname_datacard_in  = prefixDCin  + "%s_%i"%(signal,mass)+"_13TeV_CMS_jj_"+ch+purity+".txt"
       fname_datacard_out = prefixDCout + "%s_%i"%(signal,mass)+"_13TeV_CMS_jj_"+ch+purity+".txt"
       lines = []   
       try:
         print " Opening " , fname_datacard_in
         with open(fname_datacard_in) as infile:
           for line in infile:
             if not line.find("CMS_XS_")!=-1:
               lines.append(line)
           
           lines.append(newline1)
           lines.append(newline2)
           with open(fname_datacard_out, 'w') as outfile:
             print " Writing to " , fname_datacard_out
             for line in lines:
               outfile.write(line)
       except EnvironmentError: # parent of IOError, OSError *and* WindowsError where available
           print 'oops, datacard not found!'
       
 
 
  