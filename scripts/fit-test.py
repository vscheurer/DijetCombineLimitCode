#!/usr/bin/python
from optparse import OptionParser
import os,commands, os.path
import sys
from ROOT import *
import math
import time
import copy
from array import *
import CMS_lumi, tdrstyle
from ROOT import TVirtualFitter

#Remember to have a version of CMS_lumi.C/h and tdrstyle.C/py in your folder!!

tdrstyle.setTDRStyle()
gStyle.SetOptFit(0) 
CMS_lumi.lumi_13TeV = "2.4 fb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Simulation"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod=4


sqrtS = 13000.
lumi = 2460.0

def doPull(hSB,hB,gSB,fNbins,xbins,fFitXmin,fFitXmax):
   pull = TH1F("pull","pull",fNbins,xbins)
   for b in xrange(1,fNbins):
     if( hSB.GetXaxis().GetBinLowEdge(b)>=fFitXmin and hSB.GetXaxis().GetBinUpEdge(b)<=fFitXmax ):
      nbkg = hB.GetBinContent(b)
      ndata = hSB.GetBinContent(b)
      if ndata != 0:
        err_data_low = gSB.GetErrorYlow(b-1) 
        err_data_high= gSB.GetErrorYhigh(b-1)
        if(nbkg > ndata):
          err = err_data_high
        else:
          err = err_data_low  
        # err = TMath.Sqrt( h.GetBinError(b)*hdata.GetBinError(b) + histsum.GetBinError(b)*histsum.GetBinError(b) )
        p = (ndata-nbkg)/err
        pull.SetBinContent(b,p)
        pull.SetBinError(b,1) 
	 #print "bin %i center %.2f pull %.2f" %(b,hdata.GetBinCenter(b),p)  

   pull.SetLineColor(kBlack)      
   pull.SetLineColor(kBlack)
   pull.SetMarkerColor(kBlack)
   pull.SetMarkerStyle(20)
   pull.SetMarkerSize(1.)
   pull.SetMinimum(-6.)
   pull.SetMaximum(6.)
   pull.GetYaxis().SetTitle("#frac{Data-MC}{#sigma}")
   pull.GetYaxis().SetNdivisions(4)
   pull.GetYaxis().SetLabelSize(0.15)
   pull.GetXaxis().SetLabelSize(0.15)
   pull.GetYaxis().SetTitleSize(0.2)
   pull.GetYaxis().SetTitleOffset(0.2)
   pull.GetYaxis().CenterTitle()

   return pull

def performFit(fInputFileB,fInputFileS, fPlot, fNbins, fBins,fFitXmin, fFitXmax,fLabel,fOutputFile,do2par,doSigmaBand,injectS,scaleS,leg,xSec,addBKG):
  
  if (fInputFileB.find("DATA")!=-1):
    CMS_lumi.extraText = "Preliminary"
    
  if (fInputFileB.find("QCD")!=-1):  
    CMS_lumi.extraText = "Simulation"
    
  
  fits = []
  residuals = []
  
  xbins = array('d',fBins)
  
# --------------------------------------- Rebinning-->hMass_rebinned ---------------------------------------
  file =TFile(fInputFileB,"READ")
  hMass = file.Get(fPlot)


  if fInputFileS.find("2000")!=-1:fileB =TFile("fit-test/injectedS/Wprime2TeV_WWHPsigscale0.0.root","READ")
  if fInputFileS.find("4000")!=-1:fileB =TFile("fit-test/injectedS/Wprime4TeV_WWHPsigscale0.0.root","READ")
  if fInputFileB.find("pseudodata")!=-1:fileB =TFile("fit-test/injectedS/realpseudodata_bins0_WWHPsigscale0.0.root","READ")
  bkgFIT = fileB.Get("2par")
  
  hMass_rebinned = hMass.Rebin(fNbins,"hMass_rebinned",xbins)
  hMassNEW = TH1F("hist_mass","",fNbins,xbins)
  
# --------------------------------------------- Inject signal ---------------------------------------------
  
  if(injectS):
        
    fileS =TFile(fInputFileS,"READ")
    hMassS = fileS.Get(fPlot)
    hMassS.Scale(xSec*lumi)
    hMassS.Scale(scaleS)
    hMass_rebinnedS = hMassS.Rebin(fNbins,"hMass_rebinnedS",xbins)
    
    hS = copy.copy(hMass_rebinnedS)
    hS.SetName("Signal")
    hB = copy.copy(hMass_rebinned)
    hB.SetName("Background")

    cSB = TCanvas("cSB","VV mass fit",600,700)
    cSB.GetWindowHeight()
    cSB.GetWindowWidth()
    cSB.SetLogy()
    cSB.Divide(1,2,0,0,0)

    cSB.cd(1)
    pad1 = cSB.GetPad(1)
    pad1.SetPad(0.01,0.26,0.99,0.98)
    pad1.SetLogy()
    pad1.SetRightMargin(0.05)
    pad1.SetTopMargin(0.05)
    pad1.SetFillColor(0)
    pad1.SetBorderMode(0)
    pad1.SetFrameFillStyle(0)
    pad1.SetFrameBorderMode(0)

    addInfo = TPaveText(0.2358691,0.04035043,0.5050171,0.1870085,"NDC")
    addInfo.AddText(fLabel)
    addInfo.SetFillColor(0)
    addInfo.SetLineColor(0)
    addInfo.SetFillStyle(0)
    addInfo.SetBorderSize(0)
    addInfo.SetTextFont(42)
    addInfo.SetTextSize(0.040)
    addInfo.SetTextAlign(12)

    frame1 = pad1.DrawFrame(fFitXmin,0.05,fFitXmax,hB.GetMaximum()*6.0)
    frame1.SetTitle("")
    frame1.SetXTitle("Dijet invariant mass [GeV]")
    frame1.SetYTitle("Entries")
    frame1.GetYaxis().SetTitleOffset(0.95)
    frame1.GetXaxis().SetTitleSize(0.06)
    frame1.GetXaxis().SetTitleOffset(0.95)
    frame1.GetXaxis().SetLabelSize(0.05)
    frame1.GetYaxis().SetTitleSize(0.06)
    frame1.GetYaxis().SetLabelSize(0.05)
    frame1.GetXaxis().SetNdivisions(405)

    hB.SetFillStyle(3002)
    hB.SetFillColor(kBlack)
    hB.SetLineColor(kBlack)
    hS.SetLineColor(kRed)
    hB.GetXaxis().SetRangeUser(fFitXmin,fFitXmax)
    hB.Draw("histSAME")
    hS.Draw("histSAME")
    hSB= copy.copy(hB)
    hSB.Add(hS)

    x=[]
    y=[]
    exl=[]
    exh=[]
    eyl=[]
    eyh=[]
    alpha = 1 - 0.6827
    for i in range(0,fNbins):
      n    = hSB.GetBinContent(i+1)
      dm   = hSB.GetBinWidth(i+1)
      mass = hSB.GetBinCenter(i+1)
      xl   = hSB.GetBinLowEdge(i+1)
      xh   = xl+dm
      x.append( (xl+xh)/2.)
      exl.append( dm/2.)
      exh.append( dm/2.)
      y.append( n)
      # l = 0.5*TMath.ChisquareQuantile(alpha/2,2*n) #as used in dijet analysis
      # h = 0.5*TMath.ChisquareQuantile(1-alpha/2,2*(n+1))
      if(n==0): l = 0
      else: l = Math.gamma_quantile(alpha/2,n,1.) #as recommended in https://twiki.cern.ch/twiki/bin/view/CMS/PoissonErrorBars
      h = Math.gamma_quantile_c(alpha/2,n+1,1)
      # eMC = hSB.GetBinError(i+1)
      eDATA_L = (n-l)
      eDATA_H = (h-n)
      # if (eMC < eDATA_L or eMC < eDATA_H):
      eyl.append( (n-l) )
      eyh.append( (h-n) )
      # else:
#         eyl.append(hSB.GetBinError(i+1))
#         eyh.append(hSB.GetBinError(i+1) )
    vx = array("f",x)
    vy = array("f",y)
    vexl = array("f",exl)
    vexh = array("f",exh)
    veyl = array("f",eyl)
    veyh = array("f",eyh)

    gSB = TGraphAsymmErrors(fNbins,vx,vy,vexl,vexh,veyl,veyh)
    gSB.SetName("gSB")
    gSB.SetMarkerSize(0.9)
    gSB.SetMarkerStyle(20)
    gSB.Draw("pe0 same")
    addInfo.Draw("same")

    legend = TLegend(0.6558006,0.7136243,0.80074784,0.9141038)
    legend.SetTextSize(0.038)
    legend.SetLineColor(0)
    legend.SetShadowColor(0)
    legend.SetLineStyle(1)
    legend.SetLineWidth(1)
    legend.SetFillColor(0)
    legend.SetFillStyle(0)
    legend.SetMargin(0.35)
    if fInputFileS.find("Wprime")!=-1:
      if fInputFileS.find("2000")!=-1: legend.AddEntry(hS, "W' (2 TeV) #times %s"%leg,"lpe")
      if fInputFileS.find("4000")!=-1: legend.AddEntry(hS, "W' (4 TeV) #times %s"%leg,"lpe")
    if fInputFileS.find("Bulk")!=-1:
      if fInputFileS.find("2000")!=-1: legend.AddEntry(hS, "G_{B} (2 TeV) #times %s"%leg,"lpe")
      if fInputFileS.find("4000")!=-1: legend.AddEntry(hS, "G_{B} (4 TeV) #times %s"%leg,"lpe")
    legend.AddEntry(hB, "Pseudodata (B)","f")
    legend.AddEntry(gSB, "Pseudodata (S+B)","p")
    legend.Draw("same")
    CMS_lumi.CMS_lumi(pad1, iPeriod, iPos)

    cSB.cd(2)
    pad2 = cSB.GetPad(2)
    pad2.SetPad(0.01,0.02,0.99,0.27)
    pad2.SetBottomMargin(0.35)
    pad2.SetRightMargin(0.05)
    pad2.SetGridx()
    pad2.SetGridy()
    frame2 = pad2.DrawFrame(pad1.GetUxmin(), -4.8, pad1.GetUxmax(), 4.8)
    # frame2 = pad2.DrawFrame(fFitXmin,-3.5,fFitXmax,3.5)
    frame2.SetTitle("")
    frame2.SetXTitle("Dijet invariant mass [GeV]")
    frame2.GetXaxis().SetTitleSize(0.06)
    frame2.SetYTitle("#frac{(S+B)-B}{#sigma}")
    frame2.GetYaxis().SetTitleSize(0.15)
    frame2.GetYaxis().SetTitleOffset(0.40)
    frame2.GetYaxis().SetLabelSize(0.09)
    frame2.GetXaxis().SetTitleSize(0.15)
    frame2.GetXaxis().SetTitleOffset(0.90)
    frame2.GetXaxis().SetLabelSize(0.12)
    frame2.GetXaxis().SetNdivisions(405)
    frame2.GetYaxis().SetNdivisions(405)
    h1 = doPull(hSB,hB,gSB,fNbins,xbins,fFitXmin,fFitXmax)
    h1.Draw("same")
    line = TLine(fFitXmin,0,fFitXmax,0)
    line.Draw("same")
    pad2.RedrawAxis()
    line2=TLine()
    line2.DrawLine(pad2.GetUxmin(), pad2.GetUymax(), pad2.GetUxmax(), pad2.GetUymax())
    line2.DrawLine(pad2.GetUxmax(), pad2.GetUymin(), pad2.GetUxmax(), pad2.GetUymax())
    cname = fOutputFile+"_Mjj_sigscale%0.1f.pdf"%scaleS
    cSB.SaveAs(cname)    
    cname = fOutputFile +"_Mjj.root"
    f = TFile(cname,'RECREATE')
    cSB.Write()
    f.Write()
    f.Close()
    # time.sleep(10)
    hMass_rebinned.Add(hMass_rebinnedS)

# --------------------------------------- Filling new h dividing by binwidth ---------------------------------------

  alpha = 1 - 0.6827
  for i in range (1,fNbins):
    bincontent = hMass_rebinned.GetBinContent(i)
    binwidth = hMass_rebinned.GetBinWidth(i)
    hMassNEW.SetBinContent(i,bincontent/(binwidth))
    # if(bincontent==0): l = 0
 #    else: l = Math.gamma_quantile(alpha/2,bincontent,1.) #as recommended in https://twiki.cern.ch/twiki/bin/view/CMS/PoissonErrorBars
 #    h = Math.gamma_quantile_c(alpha/2,bincontent+1,1)
 #    eMC = hMassNEW.GetBinError(i)/binwidth
 #    eDATA_L = (bincontent-l)/(binwidth)
 #    eDATA_H = (h-bincontent)/(binwidth)
 #    if (eMC < eDATA_L or eMC < eDATA_H):
 #      if(eDATA_L > eDATA_H):
 #        hMassNEW.SetBinError(i,eDATA_L)
 #      elif(eDATA_H > eDATA_L):
 #        hMassNEW.SetBinError(i,eDATA_H)
 #    else:
 #      hMassNEW.SetBinError(i,(hMass_rebinned.GetBinError(i)/(binwidth)))

  
  print "Last non-zero bin is at x=%f" %hMass_rebinned.GetBinCenter(hMass_rebinned.FindLastBinAbove(0.99999)) 
  
  
# ---------------------------- Create graph from data with Poisson error bars (for data) ----------------------------    
  x=[]
  y=[]
  exl=[]
  exh=[]
  eyl=[]
  eyh=[]
  alpha = 1 - 0.6827
  for i in range(0,fNbins):
    n    = hMass_rebinned.GetBinContent(i+1)
    dm   = hMass_rebinned.GetBinWidth(i+1)
    mass = hMass_rebinned.GetBinCenter(i+1)
    xl   = hMass_rebinned.GetBinLowEdge(i+1)
    xh   = xl+dm
    x.append( (xl+xh)/2.)
    exl.append( dm/2.)
    exh.append( dm/2.)
    y.append( n / (dm))
    # l = 0.5*TMath.ChisquareQuantile(alpha/2,2*n) #as used in dijet analysis
    # h = 0.5*TMath.ChisquareQuantile(1-alpha/2,2*(n+1))
    if(n==0): l = 0
    else: l = Math.gamma_quantile(alpha/2,n,1.) #as recommended in https://twiki.cern.ch/twiki/bin/view/CMS/PoissonErrorBars
    h = Math.gamma_quantile_c(alpha/2,n+1,1) 
    # eMC = hMass_rebinned.GetBinError(i+1)/dm
    # eDATA_L = (n-l)/(dm)
    # eDATA_H = (h-n)/(dm)
    # if (eMC < eDATA_L or eMC < eDATA_H):
    eyl.append( (n-l)/(dm) )
    eyh.append( (h-n)/(dm) )
    # else:
    # eyl.append(hMass_rebinned.GetBinError(i+1)/dm)
    # eyh.append(hMass_rebinned.GetBinError(i+1)/dm )
  
  vx = array("f",x)
  vy = array("f",y)
  vexl = array("f",exl)
  vexh = array("f",exh)
  veyl = array("f",eyl)
  veyh = array("f",eyh)
  
  #data in TGraph format
  g = TGraphAsymmErrors(fNbins,vx,vy,vexl,vexh,veyl,veyh)
  g.SetName("g_data")
 

  nBins_fit1 = hMassNEW.FindBin(fFitXmax)- hMassNEW.FindBin(fFitXmin)
  
  zeroBins = 0
  for i in range(0,nBins_fit1):
      if hMassNEW.GetBinContent(hMassNEW.FindBin(fFitXmin)+i)==0: 
        zeroBins +=1        
  nBins_fit = nBins_fit1 - zeroBins
  
  rss = []
  rssOver5 = []
  chi2 = []
  dof = []
  dofOver5 = []
  fisher = []
  ConfidenceLevel = []
  ConfidenceLevel2 = []
  fpdfs = []
  histoCI = []
  
  #For alternate F-test inspired by Yongjie (not skipping empty bins)
  Altfisher = []
  AltConfidenceLevel = []
  rssALL = []
  chi2ALL = []
  dofALL = []
  
# --------------------------------------- Do fits ---------------------------------------
  print "###################################################     START!!    ###################################################################"
  f = 0
  FunctionTypes = [-2,0,1,2]
  for FunctionType in FunctionTypes:
    print "########################################  Do fit for function type %i :###############################################################" %FunctionType
    
    fitresult = doFit(FunctionType,hMassNEW,hMass,g,fFitXmin,fFitXmax,fNbins,xbins)
    M1Bkg = fitresult[3]
    hist_fit_residual_vsMass = fitresult[4]
    nPar = nBins_fit - fitresult[1] - 1
    # if not do2par:
    #   if FunctionType != -2:fits.append(M1Bkg)
    #   if FunctionType != -2:residuals.append(hist_fit_residual_vsMass) #if skipping 2 paramter fit, dont plot!
    # else:
    fits.append(M1Bkg)
    residuals.append(hist_fit_residual_vsMass)
    rss.append(fitresult[0])
    chi2.append(fitresult[2])
    dof.append(fitresult[1])
    # rssALL.append(fitresult[7])
    # chi2ALL.append(fitresult[6])
    # dofALL.append(fitresult[5])
    histoCI.append(fitresult[8])
    # rssOver5.append(fitresult[10])
    # dofOver5.append(fitresult[9])
    print "Calculated Chi2 / dof for f%d = %f / %d" % (FunctionType,fitresult[2],fitresult[1])
    print "Residuals = %f" % (fitresult[0])
    if (f>0):
      result = FisherTest(rss[f-1],rss[f],dof[f-1],dof[f],nBins_fit)
      # result = FisherTest(rssOver5[f-1],rssOver5[f],dofOver5[f-1],dofOver5[f],nBins_fit)
      # if (f>3):
  #       result = FisherTest(rss[1],rss[f],dof[1],dof[f],nBins_fit) #now comparing alternate four paramter fit to 3 paramter fit
        # result = FisherTest(rssOver5[1],rssOver5[f],dofOver5[1],dofOver5[f],nBins_fit)
      F = result[0]
      CL = result[1]
      fisher.append(F)
      ConfidenceLevel.append(CL)
      CL2 = result[2]
      ConfidenceLevel2.append(CL2)
      fpdfs.append(result[3])
      # Altresult = AltFisherTest(rss[f-1],rss[f],dof[f-1],dof[f],nBins_fit1) #alternative Fisher test using TMath.FDistI
      # if (f>3):
#         Altresult = AltFisherTest(rssALL[1],rssALL[f],dofALL[1],dofALL[f],nBins_fit1)
#       AltF = Altresult[0]
#       AltCL = Altresult[1]
#       Altfisher.append(AltF)
#       AltConfidenceLevel.append(AltCL)
    f += 1
# --------------------------------------- Print some results ---------------------------------------  

  print ""
  print "FOR CATEGORY: %s" %fLabel
  print "#############################################                     DIJET METHOD                     #############################################"  
  print ""
  print "Summary:" 
  print "Residuals = "+str(rss)
  print "$\chi^2$ = "+str(chi2)
  print "D.O.F ="+str(dof)
  print "Fishers F = "+str(fisher)
  print "ConfidenceLevel = "+str(ConfidenceLevel)
  print "Alternate CL (using TMath.FDistI) = "+str(ConfidenceLevel2)
  print "Functions = "+str(FunctionTypes)
  print "Nr. bins in fit = "+str(nBins_fit)
  print "-----------------------------------------"
  print ""
  print "For Latex table:"
  print ""
  print ""
  print "Signal scaled by %.1f" %scaleS
  
  print"\\begin{table}[htb]"
  print"\centering"
  print"\\begin{tabular}{|l c c c |}"
  print"\hline"
  print "\multicolumn{4}{|c|}{%s}\\\\"%fLabel
  print "\hline"
  print "Function & Residuals & $\chi^2$ & ndof \\\\"
  print "\hline"
  print "2 par & %.3f & %.3f & %i \\\\"%(rss[0],chi2[0],dof[0])
  print "3 par & %.3f & %.3f & %i \\\\"%(rss[1],chi2[1],dof[1])
  print "4 par & %.3f & %.3f & %i \\\\"%(rss[2],chi2[2],dof[2])
  print "5 par & %.3f & %.3f & %i \\\\"%(rss[3],chi2[3],dof[3])
  # print "Alt. 4 par& %.3f & %.3f & %i \\"%(rss[4],chi2[4],dof[4])
  print "\hline"
  print "Fishers23 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\\\"%(fisher[0],ConfidenceLevel[0])
  print "Fishers34 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\\\"%(fisher[1],ConfidenceLevel[1])
  print "Fishers45 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\\\"%(fisher[2],ConfidenceLevel[2])
  # print "Fishers3Alt4 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(fisher[3],ConfidenceLevel[3])
  print "\hline"
  print "\end{tabular}"
  print "\caption{WWFP category. Injected W' Graviton signal scaled by %s. }"%leg
  print "\label{tab:WWHP_%s}"%leg
  print "\end{table}"
  print "##############################################################################################################"
  alpha = 0.1
  print  " F - test results using alternate method: " 
  print "-----------------------------------------"
  if (ConfidenceLevel[0]>alpha):
    print  "A two parameter fit is sufficient to describe these data" 
  elif (ConfidenceLevel[1]>alpha):
    print  " A three parameter fit is sufficient to describe these data." 
  elif (ConfidenceLevel[2]>alpha):
    print  " A four parameter fit appears adequate to describe these data." 
  # elif (ConfidenceLevel[3]>alpha):
    # print  " The alternate fit is better than the 3 parameter default fit"
  else:
    print  " A better fit is needed for these data. " 
    print  " You should check for higher-order polynomials. "
  print "-----------------------------------------"
  print ""
  print ""



# --------------------------------------- Make plots ---------------------------------------
    
  # DrawFit(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile) #when final fit is decided, plots only one fit
  FitComparisons(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile,chi2,dof,do2par,doSigmaBand,histoCI,injectS,scaleS,bkgFIT,addBKG) #draw all fit functions with residuals
  # doRooFit(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile,chi2,dof,do2par) #DO NOT USE, IN DEVELOPEMENT
  
  del file
  
  
  
  
# ------------------------------------------------------------------------------------------    
def doFit(FunctionType,hMassNEW,hMass,g,fFitXmin,fFitXmax,fNbins,xbins):
  
  if( FunctionType==-2 ):    
      nPar=2
      BKGfit = TF1("BKGfit%i"%FunctionType," [0] / ( TMath::Power(x/13000,[1]) )",fFitXmin,fFitXmax)
      # BKGfit.SetParameter(0,8.63367e-11)
   #    BKGfit.SetParameter(1,9.39072e+00)

  if( FunctionType==0 ):
    print "Fitting three parameter default function!"
    nPar=3
    BKGfit = TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)
    # SIGfit = TF1("SIGfit%i"%FunctionType,"(TMath::Gaus(x,[3]))",fFitXmin,fFitXmax)
    # BKGfit.SetParameter(0,8.63367e-11)
    # BKGfit.SetParameter(1,9.39072e+00)
    # BKGfit.SetParameter(0,1.03079e-08)
    # BKGfit.SetParameter(1,7.86584e+00)
    # BKGfit.SetParameter(2,1.42945e+01)



  elif( FunctionType==1 ):
    print "Fitting four parameter default function!"
    nPar=4
    BKGfit = TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)) )",fFitXmin,fFitXmax)
    BKGfit.SetParameter(0,8.63367e-11)
    BKGfit.SetParameter(1,9.39072e+00)
    BKGfit.SetParameter(0,4.07887e-18)
    BKGfit.SetParLimits(0,4.15001e-18-4.07887e-18,4.15001e-18+4.07887e-18)
    BKGfit.SetParameter(1,7.86584e+00)
    BKGfit.SetParameter(2,1.42945e+01)
    BKGfit.SetParameter(3,-5.1)
    
  elif( FunctionType==2 ):
    print "Fitting five parameter  function!"
    nPar=5
    # BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)+[4]*TMath::Power(log(x/13000),2)) )",fFitXmin,fFitXmax)
    BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1])*(1+[4]*x/13000) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)) )",fFitXmin,fFitXmax)
    BKGfit.SetParameter(0,4.07887e-18)
    BKGfit.SetParLimits(0,4.15001e-18-4.07887e-18,4.15001e-18+4.07887e-18)
    BKGfit.SetParameter(1,7.86584e+00)
    BKGfit.SetParameter(2,1.42945e+01)
    BKGfit.SetParameter(3,-5.1000699000)

  elif( FunctionType==3 ):
    print "Fitting four parameter alternate function!"
    nPar=4
    BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000 + [3]*TMath::Power(x/13000,2),[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)
    # BKGfit.SetParameter(0,3.61696e-06) #VVHP
    # BKGfit.SetParameter(1,-1.29729e+00 )
    # BKGfit.SetParameter(2,5.93603e+00 )
    # BKGfit.SetParameter(3,3.02948e+02 )

    
  
  
  stopProgram=1;
  for loop in range (0,10):
    r = hMassNEW.Fit("BKGfit%i"%FunctionType,"ISR","",fFitXmin,fFitXmax)          
    fitStatus = int(r)
    print "fit status : %d" % fitStatus
    if(fitStatus==0):
      stopProgram=0
      # r.Print("V")
      break
 
  if(stopProgram==1):
    print "######################" 
    print"######################" 
    print "ERROR : Fit %i failed!!!!" %FunctionType
    print "######################" 
    print "######################"
    sys.exit()
  
  NumberOfVarBins = 0
  NumberOfObservations_VarBin = 0
  chi2_VarBin = 0.
  chi2_VarBin_ALL = 0.
  chi2_VarBin_notNorm = 0.
  chi2_VarBin_notNorm_ALL = 0.
  
  chi2_VarBin_notNorm_over5 = 0.
  chi2_VarBin_over5 = 0.
  NumberOfObservations_VarBin_over5 = 0.
  
  #Create a histogram to hold the confidence intervals
  # histoCI=TH1D("histoCI","", fNbins,xbins)
  # # histoCI=TH1D()
  # histoCI.SetTitle("Fitted histogram with .95 conf. band")
  # (TVirtualFitter.GetFitter()).GetConfidenceIntervals(histoCI)
  # #Now the "hint" histogram has the fitted function values as the
  # #bin contents and the confidence intervals as bin errors
  # histoCI.SetStats(kFALSE)
  # histoCI.SetFillColor(kRed+2)
  # histoCI.SetLineColor(kRed+2)
  # histoCI.SetFillStyle(3354)


  histoCI = TGraphErrors(fNbins)
  histoCI.SetTitle("Fitted line with .95 conf. band")
  for i in range (0, fNbins):
    histoCI.SetPoint(i, g.GetX()[i], 0)
  #Compute the confidence intervals at the x points of the created graph
  (TVirtualFitter.GetFitter()).GetConfidenceIntervals(histoCI)
  # //Now the "grint" graph contains function values as its y-coordinates
 #  //and confidence intervals as the errors on these coordinates
 #  //Draw the graph, the function and the confidence intervals
  histoCI.SetLineColor(0)
  histoCI.SetLineWidth(-802)
  histoCI.SetFillStyle(3002)
  histoCI.SetFillColor(2)  
  
  
  
  
  
  hist_fit_residual_vsMass =  TH1D("hist_fit_residual_vsMass","hist_fit_residual_vsMass",fNbins,xbins)
  
  for bin in range (1,hMassNEW.GetNbinsX()):
    if( hMassNEW.GetXaxis().GetBinLowEdge(bin)>=fFitXmin and hMassNEW.GetXaxis().GetBinUpEdge(bin)<=fFitXmax ):
       NumberOfVarBins += 1
       data = hMassNEW.GetBinContent(bin)
       err_data_low = g.GetErrorYlow(bin-1) 
       err_data_high= g.GetErrorYhigh(bin-1)
       fit = BKGfit.Integral(hMassNEW.GetXaxis().GetBinLowEdge(bin) , hMassNEW.GetXaxis().GetBinUpEdge(bin) )
       fit = fit / ( hMassNEW.GetBinWidth(bin) )
       # fit = BKGfit.Eval(hMassNEW.GetBinCenter(bin)) #yields same results
       if(fit > data):
         err_tot = err_data_high
       else:
         err_tot = err_data_low
       # if(err_tot==0):continue
       fit_residual = (data - fit) / err_tot
       err_fit_residual = 1
       
       chi2_VarBin_notNorm_ALL += pow( (data - fit) , 2 ) #For alternate method from Yongjie, not skipping emty bins
       chi2_VarBin_ALL += pow( (data - fit) , 2 ) / pow( err_tot , 2 )
       
       if (hMassNEW.GetBinContent(bin)>0.0000000000001):
         NumberOfObservations_VarBin+=1
         chi2_VarBin += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 
         chi2_VarBin_notNorm += pow( (data - fit) , 2 ) 	 #For dijet method, skipping emty bins
         print "Mass = %f , Bin = %i ,  data = %f,  fit = %f,  rss = %f ,  +=rss= %f" %(hMassNEW.GetBinCenter(bin),bin,data,fit,(pow( (data - fit) , 2 )),chi2_VarBin_notNorm)
       if (hMassNEW.GetBinContent(bin)*hMassNEW.GetBinWidth(bin)>=5):
         NumberOfObservations_VarBin_over5+=1
         chi2_VarBin_over5 += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 
         chi2_VarBin_notNorm_over5+= pow( (data - fit) , 2 ) 	 #For F-test, skipping bins with less than 5 entries
         
       hist_fit_residual_vsMass.SetBinContent(bin,fit_residual)
       hist_fit_residual_vsMass.SetBinError(bin,err_fit_residual)
  
  ndf_VarBin = NumberOfObservations_VarBin - nPar -1 #ndof for >0 bins
  ndf_VarBin_over5 = NumberOfObservations_VarBin_over5 - nPar -1 #ndof for >5 bins
    
  return [chi2_VarBin_notNorm,ndf_VarBin,chi2_VarBin,BKGfit,hist_fit_residual_vsMass,nPar,chi2_VarBin_ALL,chi2_VarBin_notNorm_ALL,histoCI,ndf_VarBin_over5,chi2_VarBin_notNorm_over5]   

def FisherTest(RSS_1,RSS_2,dof_1,dof_2,N):
  RSS1 = RSS_1
  RSS2 = RSS_2
  n1 = N - dof_1 - 1
  n2 = N - dof_2 - 1
  # print "Residuals function 1 = %f" % RSS_1
  # print "Residuals function 2 = %f" % RSS_2
  # print "DOF function 1 = %i" % n1
  # print "DOF function 2 = %i" % n2
  # print "DOF TOT = %i" % N
  # print "n1 = %d    n2 = %d" % (n1,n2)
  # print "((RSS1-RSS2)/(n2-n1)) = %f" %((RSS1-RSS2)/(n2-n1))
  # print "(RSS2/(N-n2) = %f" %(RSS2/(N-n2))
  F = ((RSS1-RSS2)/(n2-n1)) / (RSS2/(N-n2))
  F_dist = TF1("F_distr","TMath::Sqrt( (TMath::Power([0]*x,[0]) * TMath::Power([1],[1])) / (TMath::Power([0]*x + [1],[0]+[1])) ) / (x*TMath::Beta([0]/2,[1]/2))",0,1000)
  F_dist.SetParameter(0, n2-n1)
  F_dist.SetParameter(1, N-n2)
  CL = 1 - F_distr.Integral(0.00000001,F)
  alternateCL =  1.-TMath.FDistI(F,n2-n1,N-n2)
  return [F,CL,alternateCL,F_dist]

def AltFisherTest(RSS_0,RSS_1,dof_0,dof_1,N):
  p1_10 = dof_1-dof_0;
  p2_10 = N-dof_1;
  rss0 = RSS_0
  rss1 = RSS_1
  Ftest_10 = (rss0-rss1)/p1_10 / (rss1/p2_10)
  good_CL10 =  1.-TMath.FDistI(Ftest_10,p1_10,p2_10)
  return [Ftest_10,good_CL10]
    
     
# ---------------------------------------------------------------------------------------------------------------------------
def DrawFit(hMassNEW,g,M1Bkg,hist_fit_residual_vsMass,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c1 = TCanvas("c1","VV mass fit",W,H)
  c1.GetWindowHeight()
  c1.GetWindowWidth()
  c1.SetLogy()
  c1.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c1.cd(1)
  p11_1 = c1.GetPad(1)
  p11_1.SetPad(0.01,0.26,0.99,0.98)
  p11_1.SetLogy()
  p11_1.SetRightMargin(0.05)
  p11_1.SetTopMargin(0.05)
  p11_1.SetFillColor(0)
  p11_1.SetBorderMode(0)
  p11_1.SetFrameFillStyle(0)
  p11_1.SetFrameBorderMode(0)

  #Pave text
  #addInfo = TPaveText(0.1558691,0.30735043,0.3750171,0.4070085,"NDC")
  addInfo = TPaveText(0.2358691,0.04035043,0.5050171,0.1870085,"NDC")
  
  addInfo.AddText(fLabel)
  # addInfo.AddText("65 GeV < M_{P} < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  # addInfo.AddText("Pruned mass sideband")
  # addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.000005,fFitXmax,hMassNEW.GetMaximum()*5.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("Dijet invariant mass [GeV]")
  vFrame.SetYTitle("#frac{dN}{dm_{jj}} [GeV^{-1}]")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)

  g.SetMarkerSize(0.9)
  g.SetMarkerStyle(20)
  g.GetXaxis().SetNdivisions(405)
  g.Draw("pe0 same")
  M1Bkg[0].SetLineWidth(2)
  M1Bkg[0].SetLineStyle(1)
  M1Bkg[0].SetLineColor(kRed)
  M1Bkg[0].Draw("same")

 
  legend = TLegend(0.56164991,0.62,0.8503575,0.80)
  legend.SetTextSize(0.038)
  legend.SetLineColor(0)
  legend.SetShadowColor(0)
  legend.SetLineStyle(1)
  legend.SetLineWidth(1)
  legend.SetFillColor(0)
  legend.SetFillStyle(0)
  legend.SetMargin(0.35)
  legend.AddEntry(g,"CMS data","lpe")
  legend.AddEntry(M1Bkg[0], "Fit","l")
  # legend.AddEntry(M1Bkg[3], "Fit","l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c1.cd(2)
  p11_2 = c1.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  # vFrame2 = p11_2.DrawFrame(fFitXmin,-3.5,fFitXmax,3.5)
  vFrame2.SetTitle("")
  vFrame2.SetXTitle("Dijet invariant masss [GeV]")
  vFrame2.GetXaxis().SetTitleSize(0.06)
  vFrame2.SetYTitle("#frac{Data-Fit}{#sigma}")
  vFrame2.GetYaxis().SetTitleSize(0.15)
  vFrame2.GetYaxis().SetTitleOffset(0.40)
  vFrame2.GetYaxis().SetLabelSize(0.09)
  vFrame2.GetXaxis().SetTitleSize(0.15)
  vFrame2.GetXaxis().SetTitleOffset(0.90)
  vFrame2.GetXaxis().SetLabelSize(0.12)

  hist_fit_residual_vsMass[0].GetXaxis().SetNdivisions(405)
  hist_fit_residual_vsMass[0].SetLineColor( kBlack)
  hist_fit_residual_vsMass[0].SetMarkerColor(kBlack)
  hist_fit_residual_vsMass[0].SetMarkerStyle(20)
  hist_fit_residual_vsMass[0].SetMarkerSize(1.)
  hist_fit_residual_vsMass[0].Draw("same")
  line = TLine(fFitXmin,0,fFitXmax,0)
  line.Draw("same")
  p11_2.RedrawAxis()
  line2=TLine()
  line2.DrawLine(p11_2.GetUxmin(), p11_2.GetUymax(), p11_2.GetUxmax(), p11_2.GetUymax())
  line2.DrawLine(p11_2.GetUxmax(), p11_2.GetUymin(), p11_2.GetUxmax(), p11_2.GetUymax())


  cname = fOutputFile+"_fit.pdf"
  c1.SaveAs(cname)
  time.sleep(100)
  # del c1
# ---------------------------------------------------------------------------------------------------------------------------    
def FitComparisons(hMassNEW,g,M1Bkg,hist_fit_residual_vsMass,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile,chi2,dof,do2par,doSigmaBand,histoCI,injectS,scaleS,bkgFIT,addBKG):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c2 = TCanvas("c2","VV mass fit",W,H)
  c2.GetWindowHeight()
  c2.GetWindowWidth()
  c2.SetLogy()
  c2.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c2.cd(1)
  p11_1 = c2.GetPad(1)
  p11_1.SetPad(0.01,0.26,0.99,0.98)
  p11_1.SetLogy()
  p11_1.SetRightMargin(0.05)
  p11_1.SetTopMargin(0.05)
  p11_1.SetFillColor(0)
  p11_1.SetBorderMode(0)
  p11_1.SetFrameFillStyle(0)
  p11_1.SetFrameBorderMode(0)

  #Pave text
  #addInfo = TPaveText(0.1558691,0.30735043,0.3750171,0.4070085,"NDC")
  addInfo = TPaveText(0.2358691,0.04035043,0.5050171,0.1870085,"NDC")
  
  # addInfo.AddText("Pruned mass sideband")
  addInfo.AddText(fLabel)
  # addInfo.AddText("65 GeV < M_{P} < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  # addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.00005,fFitXmax,hMassNEW.GetMaximum()*6.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("Dijet invariant mass [GeV]")
  vFrame.SetYTitle("#frac{dN}{dm_{jj}} [GeV^{-1}]")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)
  
  bkgFIT.SetLineWidth(4)
  bkgFIT.SetLineColor(kRed)
  bkgFIT.SetLineStyle(1)

  g.SetMarkerSize(0.9)
  g.SetMarkerStyle(20)
  g.GetXaxis().SetNdivisions(405)
  g.Draw("pe0 same")
  if (doSigmaBand): histoCI[1].Draw("same3")
  g.Draw("pe0 same")
  if addBKG: bkgFIT.Draw("same")
  
  if (do2par):
    colors = [kBlack,kGreen,kBlue,kMagenta,kBlack,kGreen]
    styles = [2,1,4,8,2,6,8,7]
  else:
    colors = [kRed,kBlue,kMagenta,kBlack,kGreen]
    styles = [1,4,8,2,6,8,7]
  i =0
  for f in M1Bkg:
    f.SetLineWidth(2)
    f.SetLineStyle(styles[i])
    f.SetLineColor(colors[i])
    f.Draw("same")
    i+=1

  legend = TLegend(0.500351,0.7177579,0.6390289,0.9182374)
  if (doSigmaBand): legend = TLegend(0.3692308,0.6115702,0.4871795,0.946281)
  legend.SetTextSize(0.038)
  legend.SetLineColor(0)
  legend.SetShadowColor(0)
  legend.SetLineStyle(1)
  legend.SetLineWidth(1)
  legend.SetFillColor(0)
  legend.SetFillStyle(0)
  legend.SetMargin(0.35)
  legend.AddEntry(g, "Pseudodata","lpe")
  if (doSigmaBand and do2par): 
    legend.AddEntry(M1Bkg[0], "Default fit, 2 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[0],dof[0]),"l")
    legend.AddEntry(M1Bkg[1], "Default fit, 3 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[1],dof[1]),"l")
    legend.AddEntry(histoCI[1], " #pm 1 #sigma (3 par. default fit)","f")
    legend.AddEntry(M1Bkg[2], "Default fit, 4 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[2],dof[2]),"l")
    legend.AddEntry(M1Bkg[3], "Default fit, 5 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[3],dof[3]),"l")
    # legend.AddEntry(M1Bkg[4], "Alternate fit, 4 par. (#chi^{2}/ndof = %.2f/%i)"%(chi2[4],dof[4]),"l")
  elif (do2par and not doSigmaBand): 
    legend.AddEntry(M1Bkg[0], "2 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[0],dof[0]),"l")
    legend.AddEntry(M1Bkg[1], "3 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[1],dof[1]),"l")
    legend.AddEntry(M1Bkg[2], "4 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[2],dof[2]),"l")
    legend.AddEntry(M1Bkg[3], "5 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[3],dof[3]),"l")
    if addBKG: legend.AddEntry(bkgFIT, "Background only fit","l")
    # legend.AddEntry(M1Bkg[4], "Alternate fit, 4 par. (#chi^{2}/ndof = %.2f/%i)"%(chi2[4],dof[4]),"l")
  else:
    legend.AddEntry(M1Bkg[0], "Default fit, 3 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[1],dof[1]),"l")
    legend.AddEntry(M1Bkg[1], "Default fit, 4 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[2],dof[2]),"l")
    legend.AddEntry(M1Bkg[2], "Default fit, 5 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[3],dof[3]),"l")
    legend.AddEntry(M1Bkg[3], "Alternate fit, 4 par. (#chi^{2}/ndof = %.2f/%i)"%(chi2[4],dof[4]),"l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c2.cd(2)
  p11_2 = c2.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  vFrame2.SetTitle("")
  vFrame2.SetXTitle("M_{jj} [GeV]")
  vFrame2.GetXaxis().SetTitleSize(0.06)
  vFrame2.SetYTitle("#frac{Data-Fit}{#sigma}")
  vFrame2.GetYaxis().SetTitleSize(0.15)
  vFrame2.GetYaxis().SetTitleOffset(0.40)
  vFrame2.GetYaxis().SetLabelSize(0.09)
  vFrame2.GetXaxis().SetTitleSize(0.15)
  vFrame2.GetXaxis().SetTitleOffset(0.90)
  vFrame2.GetXaxis().SetLabelSize(0.12)
  vFrame2.GetXaxis().SetNdivisions(405)
  
  styles = [20,22,32,25,32]
  i =0
  for h in hist_fit_residual_vsMass:   
    h.SetLineColor(colors[i])
    h.SetMarkerColor(colors[i])
    h.SetMarkerStyle(styles[i])
    h.SetMarkerSize(1.)
    h.Draw("same")
    i+=1
  line = TLine(fFitXmin,0,fFitXmax,0)
  line.Draw("same")
  p11_2.RedrawAxis()
  line2=TLine()
  line2.DrawLine(p11_2.GetUxmin(), p11_2.GetUymax(), p11_2.GetUxmax(), p11_2.GetUymax())
  line2.DrawLine(p11_2.GetUxmax(), p11_2.GetUymin(), p11_2.GetUxmax(), p11_2.GetUymax())
  
  cname = fOutputFile+"_sigscale%0.1f.pdf"%scaleS
  if (doSigmaBand): cname = fOutputFile+"_SigmaBand_sigscale%0.1f.pdf"%scaleS
  c2.SaveAs(cname)
  cname = fOutputFile +"sigscale%0.1f.root"%scaleS
  f = TFile(cname,'RECREATE')
  c2.Write()
  f.Write()
  M1Bkg[0].SetName("2par")
  M1Bkg[0].Write()
  M1Bkg[1].Write()
  M1Bkg[2].Write()
  M1Bkg[3].Write()
  f.Close()
  time.sleep(100)

# ---------------------------------------------------------------------------------------------------------------------------
if __name__ == '__main__':
  
  # orig_stdout = sys.stdout
  # f = file("Sideband-fits-Ftest2.txt", 'w')
  # sys.stdout = f
  
  
  


  massBins =[1, 3, 6, 10, 16, 23, 31, 40, 50, 61, 74, 88, 103, 119, 137, 156, 176, 197, 220, 244, 270, 296, 325, 354, 386, 419, 453, 489, 526, 565, 606, 649, 693, 740, 788, 838, 890, 944, 1000, 1058,
             1118, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687, 1770, 1856, 1945, 2037, 2132, 2231, 2332, 2438, 2546, 2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704, 3854, 4010, 4171, 4337, 
             4509, 4686, 4869, 5058, 5253, 5455, 5663, 5877, 6099, 6328, 6564, 6808]
             
  do2par = True #to add 2 paramter fit
  channels = ["VV","WW","WZ","ZZ"]
  channels = ["WZ"]
  for ch in channels:
    if ch.find("q") != -1: 
      fitmax = 5058
    else: 
      fitmax = 4509
      
    xsec_Wprime2TeV = 0.027864814398000003
    xsec_Wprime4TeV = 2.8830714364733906e-08
    xsec_Bulk2TeV = 0.000239518814875
    xsec_Bulk4TeV = 24.3824379692e-07
    excluded_Wprime2TeV = 0.015
    excluded_Wprime4TeV = 0.005
     

    performFit("input/PseudodataB.root","input/WprimeToWZ_13TeV_2000GeV.root",
      "DijetMassHighPuri%s"%ch, len(massBins)-1, massBins, 1000, 3147, "%s category, HP"%ch,
      "fit-test/injectedS/realpseudodata_2TeV_bins0_%sHP"%ch,do2par,doSigmaBand=False,injectS=True,scaleS=50,leg="50",xSec=xsec_Wprime2TeV,addBKG=False)
    # performFit("input/Pseudodata.root",
    #   "DijetMassLowPuri%s"%ch, len(massBins)-1, massBins, 1000, 3704, "%s category, LP"%ch,
    #   "fit-comparisons/Pseudodata_%sLP"%ch,do2par,doSigmaBand=True)
    # performFit("input/DATA_SB.root",
    #   "DijetMassNoPuri%s"%ch, len(massBins)-1, massBins, 1000, fitmax, "%s category, NP"%ch,
    #   "fit-comparisons/DATA2_SB_%sNP"%ch,do2par,doSigmaBand=True)
  # sys.stdout = orig_stdout

 
 