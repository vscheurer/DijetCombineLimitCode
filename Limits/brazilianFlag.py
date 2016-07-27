import ROOT as rt
import time
import CMS_lumi, tdrstyle
from ROOT import *


import os
import glob
import math
import array
import sys
import time
import random



tdrstyle.setTDRStyle()
CMS_lumi.lumi_13TeV = "10.0 fb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Preliminary"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod=4

def Plot(files, label, obs):

    radmasses = []
    for f in files:
        radmasses.append(float(f.replace("CMS_jj_","").split("_")[0])/1000.)
    #print radmasses

    efficiencies={}
    for mass in radmasses:
      efficiencies[mass]=0.01# assume 10/fb signal cross section #FOR Wprime= 
      if "WZ" in label.split("_")[0] or  "Zprime" in label.split("_")[0]:
        print "Taking care of V hadronic branching fractions for exclusive samples!"
        # efficiencies[mass]=0.01/((0.6991*0.6760)*(0.6991*0.6760)) # assume 10/fb signal and get rid of hadronic branching fraction
        efficiencies[mass]=0.01/((0.6991*0.6760)) # assume 10/fb signal and get rid of hadronic branching fraction

    fChain = []
    for onefile in files:
        print onefile
        fileIN = rt.TFile.Open(onefile)
        fChain.append(fileIN.Get("limit;1"))  

        rt.gROOT.ProcessLine("struct limit_t {Double_t limit;};")
        from ROOT import limit_t
        limit_branch = limit_t()

        for j in range(0,len(fChain)):
            chain = fChain[j]
            chain.SetBranchAddress("limit", rt.AddressOf(limit_branch,'limit'))

    rad = []
    for j in range(0,len(fChain)):
        chain = fChain[j]
        thisrad = []
        for  i in range(0,6):
            chain.GetTree().GetEntry(i)
            thisrad.append(limit_branch.limit)
            print "limit = %f" %limit_branch.limit
        
        print thisrad
        rad.append(thisrad)


    # we do a plot r*MR
    mg = rt.TMultiGraph()
    mg.SetTitle("X -> ZZ")
    x = []
    yobs = []
    y2up = []
    y1up = []
    y1down = []
    y2down = []
    ymean = []

    for i in range(0,len(fChain)):
        y2up.append(rad[i][0]*efficiencies[radmasses[j]])
        y1up.append(rad[i][1]*efficiencies[radmasses[j]])
        ymean.append(rad[i][2]*efficiencies[radmasses[j]])
        y1down.append(rad[i][3]*efficiencies[radmasses[j]])
        y2down.append(rad[i][4]*efficiencies[radmasses[j]])
        yobs.append(rad[i][5]*efficiencies[radmasses[j]])
        print "Yobs (before adding sigma) = %f"%rad[i][5]

    grobs = rt.TGraphErrors(1)
    grobs.SetMarkerStyle(8)
    grobs.SetMarkerSize(0.8)
    grobs.SetLineColor(rt.kBlack)
    grobs.SetLineWidth(2)
    gr2up = rt.TGraphErrors(1)
    gr2up.SetMarkerColor(0)
    gr1up = rt.TGraphErrors(1)
    gr1up.SetMarkerColor(0)
    grmean = rt.TGraphErrors(1)
    grmean.SetLineColor(1)
    grmean.SetLineWidth(2)
    grmean.SetLineStyle(3)
    gr1down = rt.TGraphErrors(1)
    gr1down.SetMarkerColor(0)
    gr2down = rt.TGraphErrors(1)
    gr2down.SetMarkerColor(0)
  
    for j in range(0,len(fChain)):
        grobs.SetPoint(j, radmasses[j], yobs[j])
        gr2up.SetPoint(j, radmasses[j], y2up[j])
        gr1up.SetPoint(j, radmasses[j], y1up[j])
        grmean.SetPoint(j, radmasses[j], ymean[j])
        gr1down.SetPoint(j, radmasses[j], y1down[j])    
        gr2down.SetPoint(j, radmasses[j], y2down[j])
        print "------------"
        print label.split("_")[0]
        print " observed %f %f" %(radmasses[j],yobs[j])
    
    mg.Add(gr2up)#.Draw("same")
    mg.Add(gr1up)#.Draw("same")
    mg.Add(grmean,"L")#.Draw("same,AC*")
    mg.Add(gr1down)#.Draw("same,AC*")
    mg.Add(gr2down)#.Draw("same,AC*")
    # if obs: mg.Add(grobs,"L")#.Draw("AC*")
    
    H_ref = 600 
    W_ref = 630 
    W = W_ref
    H  = H_ref

    T = 0.08*H_ref
    B = 0.12*H_ref 
    L = 0.12*W_ref
    R = 0.04*W_ref

    c1 = rt.TCanvas("c1","c1",50,50,W,H)
    c1.SetGrid()
    c1.SetLogy()
    c1.cd()
    frame = c1.DrawFrame(1.4,0.001, 4.1, 10)

    frame.GetYaxis().CenterTitle()
    frame.GetYaxis().SetTitleSize(0.05)
    frame.GetXaxis().SetTitleSize(0.05)
    frame.GetXaxis().SetLabelSize(0.04)
    frame.GetYaxis().SetLabelSize(0.04)
    frame.GetYaxis().SetTitleOffset(1.15)
    frame.GetXaxis().SetTitleOffset(1.05)
    frame.GetXaxis().CenterTitle()
    frame.SetMinimum(0.001)
    frame.SetMaximum(50)
    frame.GetXaxis().SetNdivisions(508)
    frame.GetYaxis().CenterTitle(True)
    
  # mg.SetTitle("")
    # mg.Draw("P")
 #
 #    mg.GetYaxis().SetTitleSize(0.05)
 #    mg.GetXaxis().SetTitleSize(0.05)
 #    mg.GetYaxis().SetTitleOffset(1.15)
 #    mg.GetYaxis().SetLabelSize(0.04)
 #    mg.GetXaxis().SetLabelSize(0.04)
 #    mg.SetMinimum(0.001)
 #    mg.SetMaximum(10)
 #    mg.GetXaxis().SetNdivisions(508)
    
    
    # mg.GetXaxis().CenterTitle(True)
    
    if "qW" in label.split("_")[0] or "qZ" in label.split("_")[0]:
        resonance="q*"
    if "RS1" in label.split("_")[0]:
        resonance="G_{RS}"
    if "Bulk" in label.split("_")[0]:
        frame.GetXaxis().SetTitle("M_{G_{Bulk}} (TeV)")
        resonance="G_{Bulk}"
    if "WZ" in label.split("_")[0]:
        resonance="W'"
        frame.GetXaxis().SetTitle("M_{W'} (TeV)")
    if "ZprimeWW" in label.split("_")[0]:
        resonance="Z'"
        frame.GetXaxis().SetTitle("M_{Z'} (TeV)")
    frame.GetYaxis().SetTitle("#sigma_{95%} #times BR("+resonance+" #rightarrow "+label.split("_")[0].replace("RS1","").replace("Bulk","").replace("Zprime","")+") [pb]")

    

    if "qW" in label.split("_")[0] or "qZ" in label.split("_")[0]:
        mg.GetXaxis().SetLimits(0.9,6.1)
    else:
        mg.GetXaxis().SetLimits(1.1,4.1)
        

    # histo to shade
    n=len(fChain)

    grgreen = rt.TGraph(2*n)
    for i in range(0,n):
        grgreen.SetPoint(i,radmasses[i],y2up[i])
        grgreen.SetPoint(n+i,radmasses[n-i-1],y2down[n-i-1])

    grgreen.SetFillColor(rt.kYellow)
    grgreen.SetLineColor(rt.kYellow)
    grgreen.SetFillStyle(1001)
    grgreen.Draw("F") 


    gryellow = rt.TGraph(2*n)
    for i in range(0,n):
        gryellow.SetPoint(i,radmasses[i],y1up[i])
        gryellow.SetPoint(n+i,radmasses[n-i-1],y1down[n-i-1])

    gryellow.SetFillColor(rt.kGreen)
    gryellow.SetLineColor(rt.kGreen)
    gryellow.SetFillStyle(1001)
    gryellow.Draw("Fsame") 

    grmean.Draw("L")
    if obs: grobs.Draw("LPsame")

    gtheory = rt.TGraphErrors(1)
    gtheory.SetLineColor(rt.kRed)
    gtheory.SetLineWidth(3)
    ftheory=open("signalcrosssections13TeV.txt")
    j=0
    glogtheory = rt.TGraphErrors(1)
    for lines in ftheory.readlines():
     for line in lines.split("\r"):
       if label.split("_")[0] in line:
        split=line.split(":")
        print split[1]
        print split[0][-4:]
        gtheory.SetPoint(j, float(split[0][-4:])/1000., float(split[1]))
        glogtheory.SetPoint(j, float(split[0][-4:])/1000., log(float(split[1])))
        j+=1
    
    mg.Add(gtheory,"L")
    gtheory.Draw("L")
    if "WZ" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(G_{W'}#rightarrowWZ) HVT_{B}"
    if "BulkWW" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(G_{Bulk}#rightarrowWW) #tilde{k}=0.5"
    if "BulkZZ" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(G_{Bulk}#rightarrowZZ) #tilde{k}=0.5"  
    if "ZprimeWW" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(G_{Z'}#rightarrowWW) HVT_{B}"
    crossing=0
    for mass in range(int(radmasses[0]*1000.),int(radmasses[-1]*1000.)):
        if exp(glogtheory.Eval(mass/1000.))>grmean.Eval(mass/1000.) and crossing>=0:
          print label,"exp crossing",mass
          crossing=-1
        if exp(glogtheory.Eval(mass/1000.))<grmean.Eval(mass/1000.) and crossing<=0:
          print label,"exp crossing",mass
          crossing=1
          crossing=0
    for mass in range(int(radmasses[0]*1000.),int(radmasses[-1]*1000.)):
        if exp(glogtheory.Eval(mass/1000.))>grobs.Eval(mass/1000.) and crossing>=0:
          print label,"obs crossing",mass
          crossing=-1
        if exp(glogtheory.Eval(mass/1000.))<grobs.Eval(mass/1000.) and crossing<=0:
          print label,"obs crossing",mass
          crossing=1
    
    # if "WW" in label.split("_")[0] or "ZZ" in label.split("_")[0]:
    #    leg = rt.TLegend(0.43,0.65,0.95,0.89)
    #    leg2 = rt.TLegend(0.43,0.65,0.95,0.89)
    # else:
    leg = rt.TLegend(0.368995,0.6602591,0.9146734,0.9011917)
    leg2 = rt.TLegend(0.368995,0.6602591,0.9146734,0.9011917)
    leg.SetTextSize(0.028)
    leg.SetLineColor(1)
    leg.SetShadowColor(0)
    leg.SetLineStyle(1)
    leg.SetLineWidth(1)
    leg.SetFillColor(kWhite)
    # leg.SetFillStyle(0)
    leg.SetMargin(0.35)
    leg2.SetTextSize(0.028)
    leg2.SetLineColor(1)
    leg2.SetShadowColor(0)
    leg2.SetLineStyle(1)
    leg2.SetLineWidth(1)
    leg2.SetFillColor(0)
    leg2.SetFillStyle(0)
    leg2.SetMargin(0.35)
    leg.SetBorderSize(1)

    if obs: leg.AddEntry(grobs, "Asymptotic CL_{S} Observed", "Lp")
    leg.AddEntry(gryellow, "Asymptotic CL_{S} Expected #pm 1#sigma", "f")
    leg.AddEntry(grgreen, "Asymptotic CL_{S} Expected #pm 2#sigma", "f")
    leg.AddEntry(gtheory, ltheory, "L")

    if obs: leg2.AddEntry(grobs, " ", "")
    leg2.AddEntry(grmean, " ", "L")
    leg2.AddEntry(grmean, " ", "L")
    leg2.AddEntry(gtheory, " ", "")



    
    # addInfo = rt.TPaveText(0.548995,0.1830769,0.9346734,0.2897203,"NDC")
    addInfo = rt.TPaveText(0.6946309,0.5437063,0.795302,0.6363636,"NDC")
    addInfo.SetFillColor(0)
    addInfo.SetLineColor(0)
    addInfo.SetFillStyle(0)
    addInfo.SetBorderSize(0)
    addInfo.SetTextFont(42)
    addInfo.SetTextSize(0.040)
    addInfo.SetTextAlign(12)
    
  
    # addInfo.AddText("Pruned mass sideband")
    if(label.find("HP")!=-1):
      if(label.find("_WW")!=-1):addInfo.AddText("WW enriched")
      elif(label.find("_WZ")!=-1):addInfo.AddText("WZ enriched")
      elif(label.find("_ZZ")!=-1):addInfo.AddText("ZZ enriched")
      elif(label.find("_VV_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VVHP_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VV_old")!=-1):addInfo.AddText("VV category")
      addInfo.AddText("High-purity")
    elif(label.find("LP")!=-1):
      if(label.find("_WW")!=-1):addInfo.AddText("WW enriched")
      elif(label.find("_WZ")!=-1):addInfo.AddText("WZ enriched")
      elif(label.find("_ZZ")!=-1):addInfo.AddText("ZZ enriched")
      elif(label.find("_VV_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VVLP_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VV_old")!=-1):addInfo.AddText("VV category")
      addInfo.AddText("Low-purity")
    else:
      if label.find("old")!=-1:
        addInfo.AddText("VV category")
        addInfo.AddText("HP+LP")
      if label.find("new")!=-1:
        addInfo.AddText("WW+WZ+ZZ")
        addInfo.AddText("HP+LP")
    addInfo.Draw()
    c1.Update() 
    frame = c1.GetFrame()
    frame.Draw()
    CMS_lumi.CMS_lumi(c1, iPeriod, iPos)
    c1.cd()
    c1.Update()
    c1.RedrawAxis()
    c1.RedrawAxis("g")
    c1.cd()
    c1.Update()
    
    c1.cd()
    c1.Update()
    
    leg.Draw()
    leg2.Draw("same")
    
    c1.SaveAs("80X/brazilianFlag_%s_13TeV.root" %label)
    c1.SaveAs("80X/brazilianFlag_%s_13TeV.pdf" %label)
    # c1.SaveAs("/shome/thaarres/Notes/notes/AN-15-211/trunk/figures/limits/brazilianFlag_%s_13TeV.pdf" %label)
    time.sleep(10)

if __name__ == '__main__':

  channels=["RS1WW","RS1ZZ","WZ","qW","qZ","BulkWW","BulkZZ"]
  channels=["ZprimeWW","BulkWW","BulkZZ"]
  channels=["WZ"]
  for chan in channels:
    print "chan =",chan
    masses =[m*100 for m in range(12,40+1)]
    masses =[1200,1400,1800,2000,2500,3000,3500,4000]
    
    HPplots=[]
    LPplots=[]
    WWHPplots=[]
    WZHPplots=[]
    ZZHPplots=[]
    WWLPplots=[]
    WZLPplots=[]
    ZZLPplots=[]
    combinedplots_old=[]
    combinedplots=[]
    for mass in masses: 
       print HPplots
       HPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_VVHPnew_asymptoticCLs_new.root"]
       LPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_VVLPnew_asymptoticCLs_new.root"]
       combinedplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_VVnew_asymptoticCLs_new.root"]
       combinedplots_old+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_VV_asymptoticCLs_new.root"]
       WWHPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_WWHP_asymptoticCLs_new.root"]
       WZHPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_WZHP_asymptoticCLs_new.root"]
       ZZHPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_ZZHP_asymptoticCLs_new.root"]
       WWLPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_WWLP_asymptoticCLs_new.root"]
       WZLPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_WZLP_asymptoticCLs_new.root"]
       ZZLPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_ZZLP_asymptoticCLs_new.root"]
       
    #
    Plot(WWHPplots,chan+"_WWHP", obs=True)
    Plot(WWLPplots,chan+"_WWLP", obs=True)
    # Plot(WZHPplots,chan+"_WZHP", obs=False)
    # Plot(WZLPplots,chan+"_WZLP", obs=False)
    Plot(ZZHPplots,chan+"_ZZHP", obs=True)
    Plot(ZZLPplots,chan+"_ZZLP", obs=True)
    # Plot(LPplots,chan+"_VVLP_new_combined_purity", obs=False)
    # Plot(HPplots,chan+"_VVHP_new_combined_purity", obs=False)
    # Plot(combinedplots,chan+"_new_combined", obs=False)
    # Plot(combinedplots_old,chan+"_old_combined", obs=True)