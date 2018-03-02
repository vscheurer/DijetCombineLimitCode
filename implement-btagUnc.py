import sys,os
import ROOT
from ROOT import *
import math


def get_SF_VV(btagup,btagdown):
  fsys = []
  fsys.append( (1+btagup))
  fsys.append( (1-btagdown) )
  
  return fsys



indir = 'datacards/'
outdir = 'datacards/'

signals  = ["BulkZZ"]#,"altBulkZZ"]
#signals  = ["BulkWW","BulkZZ"]#,"altBulkZZ"]#
#signals=["altBulkWW","BulkWW"]
#signals  = ["BulkZZ","BulkWW","WZ","ZprimeWW","qW","qZ"]
#signals  = ["qW","qZ"]
purities = ["LPBtagged","HPBtagged"]

for signal in signals:  
  masses =[m*100 for m in range(12,40+1)]
  channels = ["WW","WZ","ZZ"]
  for purity in purities:
    for ch in channels:
       for m in masses:
         
         if m > 1500:
             btagup = 0.04
             btagdown = 0.08
         else:
             btagup = 0.02
             btagdown = 0.04
         fname_datacard_in  = indir  + "CMS_jj_%s_%i"%(signal,m)+"_13TeV_CMS_jj_"+ch+purity+".txt"
         fname_datacard_out = outdir + "CMS_jj_%s_%i"%(signal,m)+"_13TeV_CMS_jj_"+ch+purity+".txt"

         lines = []
         
         print "For input datacard:" ,fname_datacard_in
 
         newline  = '\nCMS_eff_btag_sf_13TeV        lnN     '
           

        
         sysVV = get_SF_VV(btagup,btagdown)                                                  
         if purity.find("Btagged") !=-1: 
             newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysVV[0] ,sysVV[1] ,sysVV[0] ,sysVV[1] )
           
         else: print "THIS CATEGORY DOES NOT EXIST!!"
             
         newline  +='\n'
         print newline   
         
     

        
         try:
           print " Opening " , fname_datacard_in
           with open(fname_datacard_in) as infile:
             for line in infile:
               if not line.find("CMS_eff_btag_sf_13TeV")!=-1:
                 lines.append(line)
             lines.append(newline)

             with open(fname_datacard_out, 'w') as outfile:
               print " Writing to " , fname_datacard_out
               for line in lines:
                 outfile.write(line)
         except EnvironmentError: # parent of IOError, OSError *and* WindowsError where available
             print 'oops, datacard not found!'
