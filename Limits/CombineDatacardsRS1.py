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

masses =[m*100/2 for m in range(2*10,2*29+1)]

xsec_x_WW=[1000,1500,1800,2000,2500,3000]
xsec_y_WW=[log(4.254E-01),log(3.298E-02),log(9.056E-03),log(4.083E-03),log(6.191E-04),log(1.010E-04)]
xsec_x_ZZ=[1000,1500,1800,2000,3000]
xsec_y_ZZ=[log(2.137E-01),log(1.662E-02),log(4.559E-03),log(2.027E-03),log(5.099E-05)]
xsec_x_array_WW=array.array('d')
xsec_y_array_WW=array.array('d')
xsec_x_array_ZZ=array.array('d')
xsec_y_array_ZZ=array.array('d')
for p in xsec_x_WW: xsec_x_array_WW.append(p)
for p in xsec_y_WW: xsec_y_array_WW.append(p)
for p in xsec_x_ZZ: xsec_x_array_ZZ.append(p)
for p in xsec_y_ZZ: xsec_y_array_ZZ.append(p)
g_WW=TGraph(len(xsec_x_array_WW),xsec_x_array_WW,xsec_y_array_WW)
g_ZZ=TGraph(len(xsec_x_array_ZZ),xsec_x_array_ZZ,xsec_y_array_ZZ)
f_out_WW=open("theory_RS1_WW_8TeV.txt","w")
f_out_ZZ=open("theory_RS1_ZZ_8TeV.txt","w")
for mass in masses:
        print "mass = ",mass

        fWW=open("datacards/CMS_jj_RS1WW_"+str(mass)+"_8TeV_CMS_jj_VV.txt").readlines()
	fZZ=open("datacards/CMS_jj_RS1ZZ_"+str(mass)+"_8TeV_CMS_jj_VV.txt").readlines()
	outfile="datacards/CMS_jj_RS1_"+str(mass)+"_8TeV_CMS_jj_VV.txt"
	print outfile
        f=open(outfile,"w")
        theoryWW=exp(g_WW.Eval(mass))
        theoryZZ=exp(g_ZZ.Eval(mass))
	f_out_WW.write(str(mass)+" "+str(theoryWW)+"\n")
	f_out_ZZ.write(str(mass)+" "+str(theoryZZ)+"\n")
	for l in range(len(fWW)):
	  if "rate" in fWW[l]:
	    line="rate                                     "
	    fWWsplit=fWW[l].split(" ")
	    fZZsplit=fZZ[l].replace("  "," ").replace("  "," ").replace("  "," ").replace("  "," ").replace("  "," ").replace("  "," ").split(" ")
	    count=0
	    for s in range(len(fWWsplit)):
	      try:
	        float(fWWsplit[s])
		number=True
	      except: number=False
	      if number:
	        numberWW=float(fWWsplit[s])
	        numberZZ=float(fZZsplit[count+1])
		if count==0:
		    number=numberZZ*100.*theoryZZ
		elif count==2:
		    number=numberWW*100.*theoryWW
		elif count==4:
		    number=numberZZ*100.*theoryZZ
		elif count==6:
		    number=numberWW*100.*theoryWW
		else:
		    number=numberWW
		count+=1
                line+="%.5e  " % number
	    line+="\n"
	    f.write(line)
	  else:
	    f.write(fWW[l])
