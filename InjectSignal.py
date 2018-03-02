import numpy as np
from ROOT import *
import fileinput
import sys


effi = 0.5
lumi = 35867
mass = sys.argv[1]
print len(sys.argv)



if len(sys.argv) <= 2:
    WQ = 1
    print len(sys.argv)
else:
    
    WQ = sys.argv[2]

Limits = [0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01,0.01]

NumSignal = []
for masspointLimit in Limits:
    NumSignal.append(effi*lumi*masspointLimit*float(WQ))



with open("old_datacards/datacards0.6/CMS_jj_BulkZZ_"+str(mass)+"_13TeV_CMS_jj_comb_bb.txt") as datacard:

    for line in datacard:
        if line.find("observation")!=-1:
              strBKG = line.split("   ")
              strBKG[0] = strBKG[0].replace("observation ", "")
              #strBKG.pop(0)
              strBKG[-1] = strBKG[-1].replace("\n", "")
              print strBKG
              BKG = map(float, strBKG)
                
        if line.find("rate")!=-1:
         
              strRATE = line.split("       ")
              strRATE[0] = strRATE[0].replace("rate", "")
              strRATE.pop()
              for element in range(0,12):
                while strRATE[element] == " 1.0000" or strRATE[element] == " 0.0000" or strRATE[element] == "" or strRATE[element] == "1.0000" or strRATE[element] == "0.0000" or strRATE[element] == "  1.0000":
                    strRATE.pop(element)


              RATE = map(float, strRATE)
print BKG
print RATE
totRATE = np.sum(RATE)
       
newBKG = []
Stff = []
for i in range(len(BKG)):
    newBKG.append(BKG[i]+  (NumSignal[((int(mass)-1400))/100]*RATE[i]/totRATE))
    #print (int(mass)-1400)/100
    Stff.append(newBKG[i] -BKG[i])
print map(int, Stff)

np.savetxt("FakeSignal"+str(mass)+".txt", Stff)

