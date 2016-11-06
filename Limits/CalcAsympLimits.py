from ROOT import *
import ROOT
import array, math
import os,sys
import time


rmin=0.1
rmax=100

channels=["RS1WW","RS1ZZ","WZ","qW","qZ","BulkWW","BulkZZ"]
channels=["BulkWW","BulkZZ","WZ","ZprimeWW"]
channels=["qZ","qW"]
fullToys=False
postfix =""
freezeCMD="--freezeNuisances CMS_bkg_fit_slope1_CMS_jj_ZZHP_13TeV"
# freezeCMD=""
for chan in channels:
  print "chan =",chan
  if "q" in chan:
    masses =[m*100 for m in range(12,62+1)]
    bins = ["CMS_jj_qVnew","CMS_jj_qWHP","CMS_jj_qWLP","CMS_jj_qZHP","CMS_jj_qZLP"]
    bins = ["CMS_jj_qVnew"]#,"CMS_jj_qWHP","CMS_jj_qWLP","CMS_jj_qZLP"]
    # masses =[1600,1900,2100,2300,2400,2500,2600,2700,2900,3100,3700,4200,4600,5100,5400,5800,5900,6000,6100]
    # masses = [2500,2900,5100,2600,5400,5800,5900,2300]
    # masses = [1900,2000,2100,2300,2400,2500,2600,2700,2900]


    
    
    
  
    
    
  else:
    masses =[m*100 for m in range(11,45+1)]
    bins = ["CMS_jj_WWHP","CMS_jj_WZHP","CMS_jj_ZZHP","CMS_jj_WWLP","CMS_jj_WZLP","CMS_jj_ZZLP","CMS_jj_VVHPnew","CMS_jj_VVLPnew","CMS_jj_VVnew"]
    # bins = ["CMS_jj_WWHP","CMS_jj_ZZHP","CMS_jj_WWLP","CMS_jj_ZZLP"]
    bins = ["CMS_jj_ZZLP"]
    masses =[1100]
    
  if fullToys:
    points=[]
    for p in range(1,10):
      points+=[float(p/10.)]
      points+=[float(p/10.+0.05)]
      points+=[float(p/1.)]
      points+=[float(p/1.+0.5)]
      points+=[float(p*10.)]
      points+=[float(p*10.+5.)]
  else:
    points=[0.1]
  i = 0
  for bin in bins:
    for mass in masses:
      print "mass =",mass
      for point in points:
        # i +=1
        # if i%50 == 0: time.sleep(10)
        command = "qsub -q short.q Limits/submitAsympLimits.sh %i %s %i %s %f %s %s %s"%( fullToys,chan,mass,bin,point, str(rmin), str(rmax), postfix )
        print command
        print "combine datacards/%sCMS_jj_%s_%i_13TeV_%s.txt -M Asymptotic -v2 -m %i -n %s%s --rMin %s --rMax %s >> myout.txt 2>>myerr.txt"%(postfix,chan,mass,bin,mass,chan,bin, str(rmin), str(rmax))
        os.system(command)
        # time.sleep(2)
