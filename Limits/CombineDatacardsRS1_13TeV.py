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

masses =[m*100 for m in range(10,40+1)]

xsec_x_WW=[1000,1500,2000,2500,3000,4000]
xsec_y_WW=[log(2.37e-3*1e3),log(2.35e-4*1e3),log(4.797e-5*1e3),log(9.4e-6*1e3),log(2.92e-6*1e3),log(2.739e-7*1e3)]
xsec_x_ZZ=[1000,4000]
xsec_y_ZZ=[log(1e-10),log(1e-10)]
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
f_out_WW=open("theory_RS1_WW_13TeV.txt","w")
f_out_ZZ=open("theory_RS1_ZZ_13TeV.txt","w")
for mass in masses:
        print "mass = ",mass

        fWW=open("datacards/CMS_jj_RS1WW_"+str(mass)+"_13TeV_CMS_jj_VVHP.txt").readlines()
	fZZ=open("datacards/CMS_jj_RS1WW_"+str(mass)+"_13TeV_CMS_jj_VVHP.txt").readlines()
	outfile="datacards/CMS_jj_RS1_"+str(mass)+"_13TeV_CMS_jj_VVHP.txt"
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
