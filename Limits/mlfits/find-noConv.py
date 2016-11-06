import os,commands, os.path
import sys
from ROOT import *
import time

gROOT.SetBatch(True)
 
path = "/mnt/t3nfs01/data01/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/Limits/mlfits"

#  Run for one signal and one channel at a time
sample = "qZ"
channel = "qVnew"

# List all fits that did not converge
for root, _, files in os.walk(path):
  for f in files:
    fullpath = os.path.join(root, f)

    if not fullpath.find("%sCMS_"%sample)!=-1:
      continue  
    if not fullpath.find(channel)!=-1:
      continue  
      
    if fullpath.find(".txt")!=-1: 
      with open(fullpath,"r") as File:
        failed = False
        for l in File:
          if l.find('Fit failed') != -1 or l.find('(-nan)') != -1 :
            print "Failed for " ,f.split("t")[1]
            break  


# Produce plots with all fits            
for root, _, files in os.walk(path):
  for f in files:
    fullpath = os.path.join(root, f)

    if not fullpath.find("%sCMS_"%sample)!=-1:
      continue  
    if not fullpath.find(channel)!=-1:
      continue  
    if fullpath.find(".root")!=-1:
      binname = f.split("_")[2]
      bin = binname[:5]
      print bin
      fileIN = TFile.Open(fullpath)
      if not fileIN:
        continue
      if channel.find("qVnew") !=-1:
        channels = ["qWHP","qZHP","qWLP","qZLP"]
        for ch in channels:
         c1 = TCanvas(ch,ch,1200,800)
         c1.cd(1)
         c1.SetLogy()
         sbFit = fileIN.Get("CMS_jj_%s_fit_s"%ch)
         if not sbFit:
           print "FILE DOES NOT EXIST!! : " ,f
           print "For channel " ,ch
           continue
         sbFit.Draw()
         c1.SaveAs("%s/plots/%s_%s.pdf" %( path,f.split(".")[0],ch ))
      else:
        c1 = TCanvas("c1","c1",1200,800)
        c1.cd(1)
        c1.SetLogy()
        sbFit = fileIN.Get("CMS_jj_%s_fit_s"%channel)
        if not sbFit:
          print "FILE DOES NOT EXIST!! : " ,f
          continue
        sbFit.Draw()
        c1.SaveAs("%s/plots/%s.pdf" %(path,f.split(".")[0]))


