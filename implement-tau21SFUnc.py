import sys,os
import ROOT
from ROOT import *
import math
systhp = 0.1 
systlp = 0.219

def get_SF_VV():
 fsys = []
 #systhp = 0.1
 fsys.append( (1+systhp)*(1+systhp) )
 fsys.append( (1-systhp)*(1-systhp) )
 #systlp = 0.219
 fsys.append( (1+systhp)*(1-systlp) )
 fsys.append( (1-systhp)*(1+systlp) )
 return fsys

def get_SF_qV():
 fsys = []
 
 fsys.append( (1+systhp) )
 fsys.append( (1-systhp) )
 fsys.append( (1-systlp) )
 fsys.append( (1+systlp) )
 return fsys


indir = 'datacards/'
outdir = 'datacards/'

signals  = ["altqW"]#,"altBulkZZ"]
signals  = ["WZ","ZprimeWW"]#
signals  = ["BulkZZ","BulkWW","WZ","ZprimeWW"]
signals  = ["qW","qZ"]
purities = ["LP","HP"]

for signal in signals:  
  masses =[m*100 for m in range(11,42+1)]
  channels = ["WW","WZ","ZZ"]
  if signal.find("q")!=-1:
    masses =[m*100 for m in range(12,62+1)]
    channels = ["qW","qZ"]
  for purity in purities:
    for ch in channels:
       for m in masses:
         
         
         fname_datacard_in  = indir  + "CMS_jj_%s_%i"%(signal,m)+"_13TeV_CMS_jj_"+ch+purity+".txt"
         fname_datacard_out = outdir + "CMS_jj_%s_%i"%(signal,m)+"_13TeV_CMS_jj_"+ch+purity+".txt"

         lines = []
         
         print "For input datacard:" ,fname_datacard_in
 
         newline  = '\nCMS_eff_vtag_tau21_sf_13TeV        lnN     '
           
         if signal.find("q")!=-1:
          
           sysqV = get_SF_qV()
           if   purity.find("HP") !=-1: 
            
             print "%.3f/%.3f           %.3f/%.3f        -" %( sysqV[0] ,sysqV[1] ,sysqV[0] ,sysqV[1] )
             newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysqV[0] ,sysqV[1] ,sysqV[0] ,sysqV[1] )
           elif purity.find("LP") !=-1: 
             
             newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysqV[2] ,sysqV[3] ,sysqV[2] ,sysqV[3] )
           else: print "THIS CATEGORY DOES NOT EXIST!!"            
         else: 
        
           sysVV = get_SF_VV()                                                  
           if   purity.find("HP") !=-1: newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysVV[0] ,sysVV[1] ,sysVV[0] ,sysVV[1] )
           elif purity.find("LP") !=-1: newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysVV[2] ,sysVV[3] ,sysVV[2] ,sysVV[3] )
           else: print "THIS CATEGORY DOES NOT EXIST!!"
         newline  +='\n'
         print newline   
         
     

        
         try:
           print " Opening " , fname_datacard_in
           with open(fname_datacard_in) as infile:
             for line in infile:
               if not line.find("CMS_eff_vtag_tau21_sf_13TeV")!=-1:
                 lines.append(line)
             lines.append(newline)

             with open(fname_datacard_out, 'w') as outfile:
               print " Writing to " , fname_datacard_out
               for line in lines:
                 outfile.write(line)
         except EnvironmentError: # parent of IOError, OSError *and* WindowsError where available
             print 'oops, datacard not found!'
