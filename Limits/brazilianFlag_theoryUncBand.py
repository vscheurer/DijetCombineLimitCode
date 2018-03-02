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
import numpy as np

tdrstyle.setTDRStyle()
CMS_lumi.lumi_13TeV = "35.9 fb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Preliminary"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod=4

def getRoot(mass,gtheoryUP,y):
    #print " y "+str(y)+" up "+str(gtheoryUP.Eval(mass))
    UP =2*y
    DOWN = 2*y
    isu = mass+0.5*mass
    isd = mass-0.5*mass
    ism = mass
    NMAX = 100000;
    N =0
    resultUP = 0
    while ( N <= NMAX): # limit iterations to prevent infinite loop
        ism = (isu + isd)/2 # new midpoint
        fc = rt.TMath.Abs(gtheoryUP.Eval(ism)-y)
        #print fc
        #print ism
        #print 
        if fc < 0.00000001 or rt.TMath.Abs((isd - isu)/2) <= 0.00000001: # solution found
            resultUP = ism
            break    
        N = N + 1 # increment step counter
        fism = gtheoryUP.Eval(ism)-y
        fisu = gtheoryUP.Eval(isu)-y
        #print str(fism)+"    "+str(fisu)
        if np.sign(fism) == np.sign(fisu):
            isu = ism 
        else:
            isd = ism # new interval
    return resultUP

def getIntersectionOfObservedLimitTheoryLine(mass,g,f):
    isu = mass*(1.5)
    isd = mass*(0.5)
    ism = mass
    NMAX = 100000;
    N =0
    resultUP = 0
    while ( N <= NMAX): # limit iterations to prevent infinite loop
        ism = (isu + isd)/2 # new midpoint
        fc = rt.TMath.Abs(f.Eval(ism)-g.Eval(ism))
        #print fc
        #print ism
        #print 
        if fc < 0.00000001 or rt.TMath.Abs((isd - isu)/2) <= 0.00000001: # solution found
            resultUP = ism
            break    
        N = N + 1 # increment step counter
        fism = f.Eval(ism)-g.Eval(ism)
        fisu = f.Eval(isu)-g.Eval(isu)
        #print str(fism)+"    "+str(fisu)
        if np.sign(fism) == np.sign(fisu):
            isu = ism 
        else:
            isd = ism # new interval
    return resultUP


    


def printTheoryUncAtPoint(mass,gtheory,gtheoryUP,gtheoryDOWN):
    resultUP = getRoot(mass,gtheoryUP,gtheory.Eval(mass))
    resultDOWN = getRoot(mass,gtheoryDOWN,gtheory.Eval(mass))
    print "theory uncertainty at mass "+str(round(mass,2))
    print "up : "+str(round(resultUP,2))+" down : "+str(round(resultDOWN,2))
    print str(round(mass,2))+" + "+str(round(resultUP-mass,2))+" - "+ str(round(mass - resultDOWN,2))+" TeV"


def plotGraph(modelname,channel,radmasses,color,obs=False):
    efficiencies={}
    rad = []
    limits = []
    filenames =[]
    for m in radmasses:
        print "GRUMPFGURGL!!!!!!"
        filename = "newSF/CMS_jj_"+str(int(m*1000))+"_"+modelname+"_13TeV_CMS_jj_"+channel+"_asymptoticCLs_new.root"
        #filename = "withoutPDFandScale/CMS_jj_"+str(int(m*1000))+"_"+modelname+"_13TeV_CMS_jj_"+channel+"_asymptoticCLs_new.root"
        filenames.append(filename)
        if  modelname.find("WZ")!=-1:
            efficiencies[m]= 0.01/((0.6991*0.6760))
        if modelname.find("BulkZZ")!=-1:
            efficiencies[m]=0.01/((0.6991*0.6991))
        if modelname.find("BulkZZ")==-1 and modelname.find("WZ")==-1:
            efficiencies[m]=0.01
        else:
            efficiencies[m]=0.01

    for onefile in filenames:
        print "using file " + onefile
        file = rt.TFile(onefile)
        tree = file.Get("limit")
        limits = []
        for quantile in tree:
            limits.append(tree.limit)
            print ">>>   %.2f" % limits[-1]

        rad.append(limits[:6])
        file.Close()

    print limits
    print rad
    mg = rt.TMultiGraph()
    mg.SetTitle("X -> ZZ")
    x = []
    yobs = []
    y2up = []
    y1up = []
    y1down = []
    y2down = []
    ymean = []

    for i in range(0,len(efficiencies)):
        y2up.append(rad[i][0]*efficiencies[radmasses[i]])
        y1up.append(rad[i][1]*efficiencies[radmasses[i]])
        ymean.append(rad[i][2]*efficiencies[radmasses[i]])
        y1down.append(rad[i][3]*efficiencies[radmasses[i]])
        y2down.append(rad[i][4]*efficiencies[radmasses[i]])
        yobs.append(rad[i][5]*efficiencies[radmasses[i]])
     
    grobs = rt.TGraphErrors(1)
    grobs.SetMarkerStyle(8)
    grobs.SetMarkerSize(0.8)
    grobs.SetLineColor(kRed)
    grobs.SetMarkerColor(kRed)
    grobs.SetLineWidth(2)
    gr2up = rt.TGraphErrors(1)
    gr2up.SetLineColor(color+1)
    gr1up = rt.TGraphErrors(1)
    gr1up.SetLineColor(color+2)
    grmean = rt.TGraphErrors(1)
    grmean.SetLineColor(color+4)
    grmean.SetLineWidth(2)
    grmean.SetLineStyle(3)
    gr1down = rt.TGraphErrors(1)
    gr1down.SetLineColor(color+2)
    gr2down = rt.TGraphErrors(1)
    gr2down.SetLineColor(color+1)
    
    grmean.SetLineWidth(2)
    gr1down.SetLineWidth(2)
    gr1up.SetLineWidth(2)
    gr2down.SetLineWidth(2)
    gr2up.SetLineWidth(2)
    print len(rad)
    print len(radmasses)
    print len(y2up)
    for j in range(0,len(radmasses)):
        grobs.SetPoint(j, radmasses[j], yobs[j])
        gr2up.SetPoint(j, radmasses[j], y2up[j])
        gr1up.SetPoint(j, radmasses[j], y1up[j])
        grmean.SetPoint(j, radmasses[j], ymean[j])
        gr1down.SetPoint(j, radmasses[j], y1down[j])    
        gr2down.SetPoint(j, radmasses[j], y2down[j])
       
    
    #mg.Add(gr2up)#.Draw("same")
    #mg.Add(gr1up)#.Draw("same")
    #mg.Add(grmean,"L")#.Draw("same,AC*")
    #mg.Add(gr1down)#.Draw("same,AC*")
    #mg.Add(gr2down)#.Draw("same,AC*")
    #if obs: mg.Add(grobs,"L")#.Draw("AC*")
    if obs:
        return [grobs,grmean,gr1up,gr2up,gr1down,gr2down]
    else:
       return [grmean,gr1up,gr2up,gr1down,gr2down] 



def Plot(files, label, obs,CompareLimits=False,plotExpLimitRatio=False):
    
    radmasses = []
    for f in files:
      if not postfix:
        radmasses.append(float(f.replace("CMS_jj_","_").split("_")[0])/1000.)
      else:
        stmp = f.replace("higgsCombinenobtag.Asymptotic.mH","").split("/")
        print stmp
        stmp2 = stmp[1].split(".")
        print stmp2
        radmasses.append(float(stmp2[0])/1000.) 
    print radmasses

    efficiencies={}
    for mass in radmasses:
      efficiencies[mass]=0.01# assume 10/fb signal cross section #FOR Wprime=
      if "WZ" in label.split("_")[0]:
        print "Taking care of WZ hadronic branching fractions for exclusive samples: 0.6991*0.6760!"
        efficiencies[mass]=0.01/((0.6991*0.6760)) #assume 10/fb signal and get rid of hadronic branching fraction)
        
      elif "BulkWW" in label.split("_")[0] or "Zprime" in label.split("_")[0]:
        print "Taking care of WW hadronic branching fractions for inclusive samples: 1.!"
        efficiencies[mass]=0.01
        
      elif "BulkZZ" in label.split("_")[0]:
        print "Taking care of ZZ hadronic branching fractions for exclusive samples: 0.0.6991*0.0.6991!"
        efficiencies[mass]=0.01/((0.6991*0.6991))
        
      elif "qZ" in label.split("_")[0]:
        print "Taking care of qZ hadronic branching fractions for inclusive samples: 1.!"
        efficiencies[mass]=0.01
        
      elif "qW" in label.split("_")[0]:
        print "Taking care of qW hadronic branching fractions for inclusive samples: 1.!"
        efficiencies[mass]=0.01
         

    #fChain = []
    #for onefile in files:
        ## if onefile.find("2500")!=-1 or onefile.find("2500")!=-1: continue
        #fileIN = rt.TFile.Open(onefile)
        #fChain.append(fileIN.Get("limit;1")) 

        #rt.gROOT.ProcessLine("struct limit_t {Double_t limit;};")
        #from ROOT import limit_t
        #limit_branch = limit_t()

        #for j in range(0,len(fChain)):
            #chain = fChain[j]
            #chain.SetBranchAddress("limit", rt.AddressOf(limit_branch,'limit'))

    #rad = []
    #for j in range(0,len(fChain)):
        #chain = fChain[j]
        #thisrad = []
        #for  i in range(0,6):
          #chain.GetTree().GetEntry(i)
          #thisrad.append(limit_branch.limit)
        #rad.append(thisrad)
        
        
    rad = []
    for mass in radmasses:
        print "Richtige Stelle"
        print "using file " + "withoutPDFandScale/higgsCombinetest.Asymptotic.mH"+str(int(mass*1000))+".root"
        filey = rt.TFile("withoutPDFandScale/higgsCombinenobtag.Asymptotic.mH"+str(int(mass*1000))+".root")
        tree = filey.Get("limit")
        limits = []
        #tree.Print()
        for quantile in tree:
            limits.append(quantile.limit)
            print quantile.limit
            print ">>>   %.2f" % limits[-1]

        rad.append(limits[:6])
        filey.Close()
    print rad
    print efficiencies

    mg = rt.TMultiGraph()
    mg.SetTitle("X -> ZZ")
    x = []
    yobs = []
    y2up = []
    y1up = []
    y1down = []
    y2down = []
    ymean = []

    for i in range(0,len(efficiencies)):

        
        y2up.append(rad[i][0]*efficiencies[radmasses[i]])
        y1up.append(rad[i][1]*efficiencies[radmasses[i]])
        ymean.append(rad[i][2]*efficiencies[radmasses[i]])
        y1down.append(rad[i][3]*efficiencies[radmasses[i]])
        y2down.append(rad[i][4]*efficiencies[radmasses[i]])
        yobs.append(rad[i][5]*efficiencies[radmasses[i]])
     
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
    output = []
    output2 = []
    for j in range(0,len(radmasses)):
        grobs.SetPoint(j, radmasses[j], yobs[j])
        gr2up.SetPoint(j, radmasses[j], y2up[j])
        gr1up.SetPoint(j, radmasses[j], y1up[j])
        grmean.SetPoint(j, radmasses[j], ymean[j])
        gr1down.SetPoint(j, radmasses[j], y1down[j])    
        gr2down.SetPoint(j, radmasses[j], y2down[j])
        output.append( ymean[j])
        output2.append( yobs[j])
    np.savetxt(postfix+"meanvalues_new_Datawb.txt",output)
    np.savetxt(postfix+"obsvalues_new_Datawb.txt",output2)
    mg.Add(gr2up)#.Draw("same")
    mg.Add(gr1up)#.Draw("same")
    mg.Add(grmean,"L")#.Draw("same,AC*")
    mg.Add(gr1down)#.Draw("same,AC*")
    mg.Add(gr2down)#.Draw("same,AC*")
    if obs: mg.Add(grobs,"L")#.Draw("AC*")
    
    
    H_ref = 600; 
    W_ref = 800; 
    W = W_ref
    H  = H_ref
  
    T = 0.08*H_ref
    B = 0.12*H_ref 
    L = 0.12*W_ref
    R = 0.04*W_ref

    c1 = rt.TCanvas("c1","c1",50,50,W,H)
    c1.SetFillColor(0)
    c1.SetBorderMode(0)
    c1.SetFrameFillStyle(0)
    c1.SetFrameBorderMode(0)
    c1.SetLeftMargin( L/W )
    c1.SetRightMargin( R/W )
    c1.SetTopMargin( T/H )
    c1.SetBottomMargin( B/H )
    c1.SetTickx(0)
    c1.SetTicky(0)
    c1.GetWindowHeight()
    c1.GetWindowWidth()
    #c1.SetLogy()
    c1.SetGrid()
    #c1.SetLogy()
    c1.cd()
    
    frame = c1.DrawFrame(1.1,0.001, 4.2, 10)
    if "qZ" in label.split("_")[0] or label.find("qW")!=-1: frame = c1.DrawFrame(1.1,0.001, 6.2, 800.)
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
    if "WZ" in label.split("_")[0] and ( label.find("_WZ")!=-1 or label.find("_VV")!=-1):
      frame.SetMinimum(0.0001)
      frame.SetMaximum(9)
    if label.find("_new_combined")!=-1:
      frame.SetMinimum(0.0001)
      frame.SetMaximum(0.1)
    if (label.find("new")!=-1) and label.find("qW")!=-1 or label.find("qZ")!=-1:
      frame.SetMinimum(0.0002)
      frame.SetMaximum(300.)
    frame.GetXaxis().SetNdivisions(508)
    frame.GetYaxis().CenterTitle(True)
    
    
    if "qW" in label.split("_")[0] or "qZ" in label.split("_")[0]:
        resonance="q*"
        frame.GetXaxis().SetTitle("M_{q*} (TeV)")
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
    frame.GetYaxis().SetTitle("#sigma_{95%} #times BR("+resonance+" #rightarrow "+label.split("_")[0].replace("RS1","").replace("Bulk","").replace("Zprime","")+") (pb)")

    

    if(label.find("q")!=-1):
        mg.GetXaxis().SetLimits(1.2,6.2)
    elif "BulkZZ" in label.split("_")[0]:
        mg.GetXaxis().SetLimits(1.1,4.0)
    else:
        mg.GetXaxis().SetLimits(1.1,4.2)
        

    # histo to shade
    n=len(radmasses)

    grgreen = rt.TGraph(2*n)
    for i in range(0,n):
        grgreen.SetPoint(i,radmasses[i],y2up[i])
        grgreen.SetPoint(n+i,radmasses[n-i-1],y2down[n-i-1])

    grgreen.SetFillColor(rt.kOrange)
    grgreen.SetLineColor(rt.kOrange)
    grgreen.SetFillStyle(1001)
    grgreen.Draw("F") 


    gryellow = rt.TGraph(2*n)
    for i in range(0,n):
        gryellow.SetPoint(i,radmasses[i],y1up[i])
        gryellow.SetPoint(n+i,radmasses[n-i-1],y1down[n-i-1])

    gryellow.SetFillColor(rt.kGreen+1)
    gryellow.SetLineColor(rt.kGreen+1)
    gryellow.SetFillStyle(1001)
    gryellow.Draw("Fsame") 

    grmean.Draw("L")
    if obs: grobs.Draw("LPsame")
    

    gtheory = rt.TGraphErrors(1)
    gtheory.SetLineColor(rt.kRed)
    gtheory.SetLineWidth(3)
    gtheoryUP = rt.TGraphErrors(1)
    gtheoryUP.SetLineColor(rt.kRed-2)
    gtheoryUP.SetLineWidth(3)
    gtheoryDOWN = rt.TGraphErrors(1)
    gtheoryDOWN.SetLineColor(rt.kRed-2)
    gtheoryDOWN.SetLineWidth(3)
    gtheorySHADE = rt.TGraphErrors(1)
    gtheorySHADE.SetLineColor(rt.kRed-2)
    gtheorySHADE.SetLineWidth(3)
    
    
    filenameTH = "/usr/users/vscheurer/DijetCombineLimitCode/Limits/%s_xSecUnc.root"%label.split("_")[0]
    thFile       = rt.TFile.Open(filenameTH,'READ')   
    print "Opening file " ,thFile.GetName()
    gtheory      = thFile.Get("gtheory")
    gtheoryUP    = thFile.Get("gtheoryUP")
    gtheoryDOWN  = thFile.Get("gtheoryDOWN")
    gtheorySHADE = thFile.Get("grshade")
    gtheory     .SetName("%s_gtheory"    %label.split("_")[0] )
    gtheoryUP   .SetName("%s_gtheoryUP"  %label.split("_")[0] )
    gtheoryDOWN .SetName("%s_gtheoryDOWN"%label.split("_")[0] )
    gtheorySHADE.SetName("%s_grshade"    %label.split("_")[0] )
    gtheorySHADE.SetLineColor(0)
    gtheoryUP.SetLineColor(rt.kRed)
    gtheoryDOWN.SetLineColor(rt.kRed)
    gtheoryUP.SetLineWidth(1)
    gtheoryDOWN.SetLineWidth(1)
    
    print "max cross section (observed limit ) : " +str(round(rt.TMath.MaxElement(n,grobs.GetY()),5))+ " pb"
    print "min cross section (observed limit ) : " +str(round(rt.TMath.MinElement(n,grobs.GetY()),5))+ " pb"
    if label.find("Zprime")!=-1:
        root = getIntersectionOfObservedLimitTheoryLine(2.6,gtheory,grobs)
        printTheoryUncAtPoint(root,gtheory,gtheoryUP,gtheoryDOWN)
    if label.find("WZ")!=-1:
        root = getIntersectionOfObservedLimitTheoryLine(2.6,gtheory,grobs)
        printTheoryUncAtPoint(root,gtheory,gtheoryUP,gtheoryDOWN)
    if label.find("qZ")!=-1:
        root = getIntersectionOfObservedLimitTheoryLine(4.5,gtheory,grobs)
        printTheoryUncAtPoint(root,gtheory,gtheoryUP,gtheoryDOWN)
    if label.find("qW")!=-1:
        root = getIntersectionOfObservedLimitTheoryLine(4.6,gtheory,grobs)
        printTheoryUncAtPoint(root,gtheory,gtheoryUP,gtheoryDOWN)
    
    mg.Add(gtheory,"L")
    mg.Add(gtheoryUP,"L")
    mg.Add(gtheoryDOWN,"L")
    mg.Add(gtheorySHADE,"L")
    gtheory.Draw("L")
    # gtheoryUP.Draw("L")
#     gtheoryDOWN.Draw("L")
    gtheorySHADE.Draw("F")
    
    if CompareLimits:
        modelname = "altBulkWW"
        channel   = "VVnew"
        if combinedplots[0].find("WWHP")!=-1:
            channel = "WWHP"
        if combinedplots[0].find("WZHP")!=-1:
            channel = "WZHP"
        if combinedplots[0].find("ZZHP")!=-1:
            channel = "ZZHP"
        if combinedplots[0].find("WWLP")!=-1:
            channel = "WWLP"
        if combinedplots[0].find("WZLP")!=-1:
            channel = "WZLP"
        if combinedplots[0].find("ZZLP")!=-1:
                channel = "ZZLP"
        if combinedplots[0].find("qW")!=-1:
            modelname = "altqW"
            channel   = "qVnew"
            if combinedplots[0].find("qWHP")!=-1:
                channel   = "qWHP"
            if combinedplots[0].find("qWLP")!=-1:
                channel   = "qWLP"
            if combinedplots[0].find("qZHP")!=-1:
                channel   = "qZHP"
            if combinedplots[0].find("qZLP")!=-1:
                channel   = "qZLP"
            
       
        
        
       
        color = 5
        cgraphs = plotGraph(modelname,channel,radmasses,color,obs)
        for g in cgraphs:
            print g
            #g.SetLineColor(kRed)
            g.Draw("Lsame")
    
    if "qZ" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(q*#rightarrowqZ)"
    if "qW" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(q*#rightarrowqW)"  
    if "WZ" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(W'#rightarrowWZ) HVT_{B}"
    if "BulkWW" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(G_{Bulk}#rightarrowWW) #tilde{k}=0.5"
    if "BulkZZ" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(G_{Bulk}#rightarrowZZ) #tilde{k}=0.5"  
    if "ZprimeWW" in label.split("_")[0]:
      ltheory="#sigma_{TH}#timesBR(Z'#rightarrowWW) HVT_{B}"

    # if "WW" in label.split("_")[0] or "ZZ" in label.split("_")[0]:
    #    leg = rt.TLegend(0.43,0.65,0.95,0.89)
    #    leg2 = rt.TLegend(0.43,0.65,0.95,0.89)
    # else:
    leg = rt.TLegend(0.498995,0.6602591,0.9446734,0.9011917)
    leg2 = rt.TLegend(0.498995,0.6602591,0.9446734,0.9011917)
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

    if obs: leg.AddEntry(grobs, "Observed", "Lp")
    leg.AddEntry(gryellow, "Expected #pm 1 std. deviation", "f")
    leg.AddEntry(grgreen , "Expected #pm 2 std. deviation", "f")
    leg.AddEntry(gtheory, ltheory, "L")

    if obs: leg2.AddEntry(grobs, " ", "")
    leg2.AddEntry(grmean, " ", "L")
    leg2.AddEntry(grmean, " ", "L")
    leg2.AddEntry(gtheorySHADE, " ", "F")



    
    # addInfo = rt.TPaveText(0.548995,0.1830769,0.9346734,0.2897203,"NDC")
    addInfo = rt.TPaveText(0.6946309,0.5437063,0.795302,0.6363636,"NDC")
    addNarrow = rt.TPaveText(0.9,0.02,0.64,0.3,"NDC")
    if label.find("qW")!=-1 or label.find("qZ")!=-1 or label.find("Zprime")!=-1 or label.find("WZ_")!=-1:
        addNarrow = rt.TPaveText(0.15,0.02,0.64,0.3,"NDC")
    if (label.find("new")!=-1) and label.find("qW")!=-1 or label.find("qZ")!=-1:addInfo = rt.TPaveText(0.7846309,0.5437063,0.825302,0.6363636,"NDC")
    addInfo.SetFillColor(0)
    addInfo.SetLineColor(0)
    addInfo.SetFillStyle(0)
    addInfo.SetBorderSize(0)
    addInfo.SetTextFont(42)
    addInfo.SetTextSize(0.040)
    addInfo.SetTextAlign(12)
    
    addNarrow.SetFillColor(0)
    addNarrow.SetLineColor(0)
    #addNarrow.SetTextColor(kRed)
    addNarrow.SetFillStyle(0)
    addNarrow.SetBorderSize(0)
    addNarrow.SetTextFont(42)
    addNarrow.SetTextSize(0.035)
    addNarrow.SetTextAlign(12)
    addNarrow.AddText("narrow width approximation")
  
    # addInfo.AddText("Pruned mass sideband")
    if(label.find("HP")!=-1):
      if(label.find("_WW")!=-1):addInfo.AddText("WW enriched")
      elif(label.find("_WZ")!=-1):addInfo.AddText("WZ enriched")
      elif(label.find("_ZZ")!=-1):addInfo.AddText("ZZ enriched")
      elif(label.find("_VV_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VVHP_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VV_old")!=-1):addInfo.AddText("VV category")
      elif(label.find("_qW")!=-1):addInfo.AddText("qW enriched")
      elif(label.find("_qZ")!=-1):addInfo.AddText("qZ enriched")
      elif(label.find("_qV")!=-1):addInfo.AddText("qW+qZ")
      addInfo.AddText("High-purity")
    elif(label.find("LP")!=-1):
      if(label.find("_WW")!=-1):addInfo.AddText("WW enriched")
      elif(label.find("_WZ")!=-1):addInfo.AddText("WZ enriched")
      elif(label.find("_ZZ")!=-1):addInfo.AddText("ZZ enriched")
      elif(label.find("_VV_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VVLP_new")!=-1):addInfo.AddText("WW+WZ+ZZ")
      elif(label.find("_VV_old")!=-1):addInfo.AddText("VV category")
      
      elif(label.find("_qW")!=-1):addInfo.AddText("qW enriched")
      elif(label.find("_qZ")!=-1):addInfo.AddText("qZ enriched")
      elif(label.find("_qV")!=-1):addInfo.AddText("qW+qZ")
      addInfo.AddText("Low-purity")
    else:
      if label.find("old")!=-1:
        addInfo.AddText("VV category")
        addInfo.AddText("HP+LP")
      elif (label.find("new")!=-1) and label.find("qW")!=-1 or label.find("qZ")!=-1:
        addInfo.AddText("qW+qZ")
        addInfo.AddText("HP+LP")
        
      elif label.find("new_combined")!=-1:
        addInfo.AddText("WW+WZ+ZZ")
        addInfo.AddText("HP+LP")
      
    addInfo.Draw()
    addNarrow.Draw()
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
    
    fname = postfix+"brazilianFlag_%s_13TeV.pdf" %label
    c1.SaveAs(fname)
    c1.SaveAs(fname.replace(".pdf" ,".C"  ))
    
    if plotExpLimitRatio:     
        yexp1 = grmean.GetY()
        yexp2 = cgraphs[0].GetY()
        if obs:
            yexp2 = cgraphs[1].GetY()
        x = grmean.GetX()
        print x
        print yexp1
        print yexp2
        print yexp1[0]
        y = []
        n = grmean.GetN()
        gratio = TGraph()
        for i in range(0,n):
            y.append((yexp2[i]/yexp1[i]))
            gratio.SetPoint(i,x[i],y[i])
        print y
        gratio.SetMaximum(2)
        gratio.SetMinimum(0)
        cratio = TCanvas("cratio","cratio",400,400)
        gratio.Draw("AP")
        addInfo2 = addText(label)
        addInfo2.Draw()
        time.sleep(5)
        cratio.SaveAs("testratio_"+label+".pdf")
        
    
    # del c1
  #   del leg
  #   del leg2
  #   del addInfo
  #   del mg
  #   del gtheory
  #   del gtheoryUP
  #   del gtheoryDOWN
    del gtheorySHADE
    thFile.Close()

    
    
    time.sleep(5)
    
def addText(label):
    bla = rt.TPaveText(0.9,0.8,0.6,0.7,"NDC")
    bla.SetFillColor(0)
    bla.SetLineColor(0)
    bla.SetFillStyle(0)
    bla.SetBorderSize(0)
    bla.SetTextFont(42)
    bla.SetTextSize(0.040)
    bla.SetTextAlign(12)
    if label.find("BulkWW")!=-1:
        bla.AddText("Bulk Graviton to WW")
    if label.find("BulkZZ")!=-1:
        bla.AddText("Bulk Graviton to ZZ") 
    if label.find("WZ_")!=-1:
        bla.AddText("W' to WZ")
    if label.find("Zprime")!=-1:
        bla.AddText("Z' to WW")
    if label.find("qW")!=-1:
        bla.AddText("q* to qW")
    if label.find("qZ")!=-1:
        bla.AddText("q* to qZ") 
    if label.find("WWLP")!=-1:
        bla.AddText("WW LP")
    if label.find("ZZ LP")!=-1:
        bla.AddText("ZZ LP") 
    if label.find("WZLP")!=-1:
        bla.AddText("WZ LP")
    if label.find("WWHP")!=-1:
        bla.AddText("WW HP")
    if label.find("ZZ HP")!=-1:
        bla.AddText("ZZ HP") 
    if label.find("WZHP")!=-1:
        bla.AddText("WZ HP")
    if label.find("VVnew")!=-1:
        bla.AddText("WW + WZ + ZZ")
        bla.AddText("HP + LP")
    if label.find("qWLP")!=-1:
        bla.AddText("qW LP")
    if label.find("qZ LP")!=-1:
        bla.AddText("qZ LP") 
    if label.find("qWHP")!=-1:
        bla.AddText("qW HP")
    if label.find("qZ HP")!=-1:
        bla.AddText("qZ HP") 
    if label.find("qVnew")!=-1:
        bla.AddText("qW + qZ")
        bla.AddText("HP + LP")
    return bla
    
        
    
    
if __name__ == '__main__':
  postfix = "withoutPDFandScale/"

  channels=["WZ","BulkWW","BulkZZ"]#,"qW","qZ"]
  channels=["BulkZZ"]
  CompareLimits = False #True
  
  for chan in channels:
    masses =[m*100 for m in range(14,40+1)]
    
    #if chan.find("BulkZZ") != -1: masses =[m*100 for m in range(11,40+1)]
    if chan.find("q") != -1: masses =[m*100 for m in range(12,60+1)]
    # if chan.find("qZHPplots") != -1:
    # masses =[1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000,2100 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000, 3100, 3200, 3300, 3400, 3500, 3600, 3700, 3800, 3900, 4000, 4100, 4200, 4300, 4400, 4500, 4600, 4700, 4800, 4900, 5000, 5100, 5200, 5300, 5400, 5500, 5600, 5700, 5800, 5900, 6000, 6100, 6200]

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
    
    qVHPplots=[]
    qVLPplots=[]
    qWHPplots=[]
    qZHPplots=[]
    qWLPplots=[]
    qZLPplots=[]
    combinedplots_qV=[]
    
    for mass in masses:
       # HPplots+=[postfix+"CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_VVHPnew_asymptoticCLs.root"]
#        LPplots+=[postfix+"CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_VVLPnew_asymptoticCLs.root"]
       if chan.find("q")!=-1:
            combinedplots+=[postfix+"higgsCombinenobtag.Asymptotic.mH"+str(mass)+".root"]
            #combinedplots+=[postfix+"CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_qWLP_asymptoticCLs_new.root"]
       else:
           #combinedplots+=[postfix+"CMS_jj_"+str(mass)+"_"+chan+"_13TeV_CMS_jj_ZZHP_asymptoticCLs_new.root"]
           combinedplots+=[postfix+"higgsCombinenobtag.Asymptotic.mH"+str(mass)+".root"]
    print combinedplots
      
    # Plot(WWHPplots,chan+"_WWHP", obs=True)
#     Plot(WWLPplots,chan+"_WWLP", obs=True)
#     Plot(WZHPplots,chan+"_WZHP", obs=True)
#     Plot(WZLPplots,chan+"_WZLP", obs=True)
#     Plot(ZZHPplots,chan+"_ZZHP", obs=True)
#     Plot(ZZLPplots,chan+"_ZZLP", obs=True)
    # Plot(LPplots,chan+"_VVLP_new_combined_purity", obs=True)
    # Plot(HPplots,chan+"_VVHP_new_combined_purity", obs=True)
   
    #Plot(combinedplots,chan+"_WWHP_alt4par", obs=True,CompareLimits=True)  
    Plot(combinedplots,chan+"_VVnew_test_new_combined", obs=True,CompareLimits=False,plotExpLimitRatio=False)

    
    
    
    # Plot(qWHPplots,chan+"_qWHP", obs=True)
    # Plot(qWLPplots,chan+"_qWLP", obs=True)
    # Plot(qZHPplots,chan+"_qZHP", obs=True)
    # Plot(qZLPplots,chan+"_qZLP", obs=True)
    # # # Plot(qVLPplots,chan+"_qVLP_new_combined_purity", obs=True)
    # # Plot(qVHPplots,chan+"_qVHP_new_combined_purity", obs=True)
    # Plot(combinedplots_qV,chan+"_new_combined", obs=True)
    # # Plot(combinedplots_old,chan+"_old_combined", obs=False)
