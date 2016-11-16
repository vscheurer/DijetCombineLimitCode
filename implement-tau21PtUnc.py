import sys,os
import ROOT
from ROOT import *
import math
 
def get_sys_pt_vv(mass):
 fsys = []
 systhp = 5.90094*math.log(mass/2./200.)/100.
 fsys.append( (1+systhp)*(1+systhp) )
 fsys.append( (1-systhp)*(1-systhp) )
 tothp = math.sqrt(0.03*0.03+0.04*0.04+0.06*0.06)/1.03
 totlp = math.sqrt(0.12*0.12+0.17*0.17+0.12*0.12)/0.88
 systlp = systhp*totlp/tothp
 fsys.append( (1+systhp)*(1-systlp) )
 fsys.append( (1-systhp)*(1+systlp) )
 return fsys

def get_sys_pt_qv(mass):
 fsys = []
 systhp = 5.90094*math.log(mass/2./200.)/100.
 print systhp
 fsys.append( (1+systhp) )
 fsys.append( (1-systhp) )
 tothp = math.sqrt(0.03*0.03+0.04*0.04+0.06*0.06)/1.03
 totlp = math.sqrt(0.12*0.12+0.17*0.17+0.12*0.12)/0.88
 systlp = systhp*totlp/tothp
 fsys.append( (1-systlp) )
 fsys.append( (1+systlp) )
 return fsys
   

indir = 'datacards/'
outdir = 'datacards_test/'

signals  = ["BulkWW","BulkZZ","WZ","ZprimeWW","qW","qZ"]
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
 
         newline  = '\nCMS_eff_vtag_tau21_pt_13TeV  lnN       '
           
         if signal.find("q")!=-1:
          
           sysqV = get_sys_pt_qv(m)
           if   purity.find("HP") !=-1: 
            
             print "%.3f/%.3f           %.3f/%.3f        -" %( sysqV[0] ,sysqV[1] ,sysqV[0] ,sysqV[1] )
             newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysqV[0] ,sysqV[1] ,sysqV[0] ,sysqV[1] )
           elif purity.find("LP") !=-1: 
             
             newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysqV[2] ,sysqV[3] ,sysqV[2] ,sysqV[3] )
           else: print "THIS CATEGORY DOES NOT EXIST!!"            
         else: 
        
           sysVV = get_sys_pt_vv(m)                                                  
           if   purity.find("HP") !=-1: newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysVV[0] ,sysVV[1] ,sysVV[0] ,sysVV[1] )
           elif purity.find("LP") !=-1: newline += "%.3f/%.3f           %.3f/%.3f        -" %( sysVV[2] ,sysVV[3] ,sysVV[2] ,sysVV[3] )
           else: print "THIS CATEGORY DOES NOT EXIST!!"
         newline  +='\n'
         print newline   
         
     

        
         try:
           print " Opening " , fname_datacard_in
           with open(fname_datacard_in) as infile:
             for line in infile:
               if not line.find("CMS_eff_vtag_tau21_pt_13TeV")!=-1:
                 lines.append(line)
             lines.append(newline)

             with open(fname_datacard_out, 'w') as outfile:
               print " Writing to " , fname_datacard_out
               for line in lines:
                 outfile.write(line)
         except EnvironmentError: # parent of IOError, OSError *and* WindowsError where available
             print 'oops, datacard not found!'
