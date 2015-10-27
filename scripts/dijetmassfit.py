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
# from ROOT import Math/QuantFuncMathCore

tdrstyle.setTDRStyle()
gStyle.SetOptFit(0) 
CMS_lumi.lumi_13TeV = "578.3 pb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Preliminary"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod = 4

# ---------------------------------------------------------------------------------------------------------------------------
sqrtS = 13000.
lumi = 578.31
fits = []
residuals = []

def bkgFit3par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x,p[1])/pow(x,p[2])
  return bkgFit4par
  
def bkgFit4par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x,p[1])/pow(x,p[2]+p[3]*TMath.Log(x))
  return bkgFit4par

def bkgFit6par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x,p[1])/pow(x,p[2]+p[3]*TMath.Log(x)+p[4]*pow(TMath.Log(x),2)+p[5]*pow(TMath.Log(x),3))
  return bkgFit6par

def AltbkgFit4par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x +p[3]*x*x,p[1])/pow(x,p[2])
  return AltbkgFit4par
# ---------------------------------------------------------------------------------------------------------------------------  
def performFit(fInputFile, fPlot, fNbins, fBins,fFitXmin, fFitXmax,fLabel,  fOutputFile,  use6ParFit, fP0=1e-04, fP1=1e+01,fP2=4e+00,fP3=-0.1e-01, fP4=0, fP5=0):
  
  # TVirtualFitter.SetMaxIterations(30000)
  bins = sorted(fBins)
  xbins = array('d',bins)
  
# --------------------------------------- Rebinning-->hMass_rebinned ---------------------------------------
  file =TFile(fInputFile,"READ")
  hMass = file.Get(fPlot)
  # hMass.Scale(1/lumi)
  # hMass.SetBinErrorOption(TH1.kPoisson)
  
  hMass_rebinned = hMass.Rebin(fNbins,"hMass_rebinned",xbins)
  # hMass_rebinned = hMass.Rebin(2)
  # hMass_rebinned.GetXaxis().SetRangeUser(fFitXmin,fFitXmax)
  
  hMassNEW = TH1F("hist_mass","",fNbins,xbins)

# --------------------------------------- Filling new h with Poisson errors-->hMassNEW -------------------
  for i in range (1,fNbins):
    bincontent = hMass_rebinned.GetBinContent(i)
    binwidth = hMass_rebinned.GetBinWidth(i)
    hMassNEW.SetBinContent(i,bincontent/(binwidth))
    # hMassNEW.GetXaxis().SetRangeUser(fFitXmin,fFitXmax)
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
    # l = 0.5*TMath.ChisquareQuantile(alpha/2,2*n)
    # h = 0.5*TMath.ChisquareQuantile(1-alpha/2,2*(n+1))
    if(n==0): l = 0
    else: l = Math.gamma_quantile(alpha/2,n,1.)
    h = Math.gamma_quantile_c(alpha/2,n+1,1) 
    eyl.append( (n-l)/(dm) )
    eyh.append( (h-n)/(dm) )
    #print "%f   %f    %f    %f    %f     %f" % (x[i],y[i],exl[i],exh[i],eyl[i],eyh[i])
  
  vx = array("f",x)
  vy = array("f",y)
  vexl = array("f",exl)
  vexh = array("f",exh)
  veyl = array("f",eyl)
  veyh = array("f",eyh)
  
  #data in TGraph format
  g = TGraphAsymmErrors(fNbins,vx,vy,vexl,vexh,veyl,veyh)
  g.SetName("g_data")
  # g.GetXaxis().SetRangeUser(fFitXmin,fFitXmax)
  
  nBins_fit = hMassNEW.FindBin(fFitXmax)- hMassNEW.FindBin(fFitXmin)
  zeroBins = 0
  for i in range(0,nBins_fit):
      if hMassNEW.GetBinContent(hMassNEW.FindBin(fFitXmin)+i)==0: 
        zeroBins +=1        
  nBins_fit = nBins_fit - zeroBins
  rss = []
  chi2 = []
  dof = []
  fisher = []
  ConfidenceLevel = []
  fpdfs = []
  FunctionTypes = [0,1,2]
  for FunctionType in FunctionTypes:
    fitresult = doFit(FunctionType,hMassNEW,hMass,g,fFitXmin,fFitXmax,fNbins,xbins) 
    M1Bkg = fitresult[3]
    hist_fit_residual_vsMass = fitresult[4]
    nPar = nBins_fit - fitresult[1] - 1
    fits.append(M1Bkg)
    residuals.append(hist_fit_residual_vsMass)
    rss.append(fitresult[0])
    chi2.append(fitresult[2])
    dof.append(fitresult[1])
    print "chi2 / dof for f%d = %f / %d" % (FunctionType,fitresult[2],fitresult[1])
    print "##############################################################################################################" 
  
  print "Do F-test comparing function %i with %i" %(0,1)
  result = FisherTest(rss[0],rss[1],dof[0],dof[1],nBins_fit)
  F = result[0]
  CL = result[1]
  fisher.append(F)
  ConfidenceLevel.append(CL)
  fpdfs.append(result[2])
  print "##############################################################################################################" 
  print "Functions = "+str(FunctionTypes) 
  print "nBins_Fit = "+str(nBins_fit)
  print "rss = "+str(rss)
  print "chi2 = "+str(chi2)
  print "dof ="+str(dof)
  print "fisher = "+str(fisher)
  print "ConfidenceLevel = "+str(ConfidenceLevel)  
  print "##############################################################################################################" 
  DrawFit(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax)
  FitComparisons(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax)
  
  
  
  del file
# ---------------------------------------------------------------------------------------------------------------------------    
def doFit(FunctionType,hMassNEW,hMass,g,fFitXmin,fFitXmax,fNbins,xbins):
  print "##############################################################################################################" 
  print FunctionType
  print "##############################################################################################################" 
  
  if( FunctionType==0 ):    
    print "Fitting three parameter default function!"
    nPar=3
    BKGfit = TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)
    
  elif( FunctionType==1 ):
    print "Fitting four parameter default function!"
    nPar=4
    BKGfit = TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)) )",fFitXmin,fFitXmax)
    
    # BKGfit.SetParLimits(0,0.,10)
  elif( FunctionType==2 ):
    print "Fitting four parameter alternate function!"
    nPar=4
    BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000 + [3]*TMath::Power(x/13000,2),[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)
  
  # elif( FunctionType==3 ):
  #   print "Fitting three parameter alternate function!"
  #   nPar=3
  #   BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000 + TMath::Power(x/13000,2),[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)   
  
  stopProgram=1;
  for loop in range (0,10):
    r = hMassNEW.Fit("BKGfit%i"%FunctionType,"ELSR","",fFitXmin,fFitXmax)          
    fitStatus = int(r)
    print "fit status : %d" % fitStatus
    if(fitStatus==0):
      stopProgram=0
      r.Print("V")  
      break
 
  if(stopProgram==1):
    print "######################" 
    print"######################" 
    print "ERROR : Fit failed!!!!" 
    print "######################" 
    print "######################"
  
  
  NumberOfVarBins = 0
  NumberOfObservations_VarBin = 0
  NumberOfObservations_VarBin_5entries = 0
  chi2_VarBin = 0.
  chi2_VarBin_notNorm = 0.
  chi2_VarBin_5entries = 0.
  chi2_VarBin_zeroes = 0. 
  
  print hMassNEW.GetBinWidth(20)
  print hMassNEW.GetNbinsX()
  
  
  
  hist_fit_residual_vsMass =  TH1D("hist_fit_residual_vsMass","hist_fit_residual_vsMass",fNbins,xbins)
  
  for bin in range (1,hMassNEW.GetNbinsX()):
    if( hMassNEW.GetXaxis().GetBinLowEdge(bin)>=fFitXmin and hMassNEW.GetXaxis().GetBinUpEdge(bin)<=fFitXmax ):
       NumberOfVarBins += 1
       #print "bin content = " + str(hMassNEW.GetBinContent(bin)) + "   graph y = " + str(vy[bin-1]) + "  error y low = " + str(g.GetErrorYlow(bin-1))
       data = hMassNEW.GetBinContent(bin)
       err_data_low = g.GetErrorYlow(bin-1) 
       err_data_high= g.GetErrorYhigh(bin-1)
       fit = BKGfit.Integral(hMassNEW.GetXaxis().GetBinLowEdge(bin) , hMassNEW.GetXaxis().GetBinUpEdge(bin) )
       fit = fit / ( hMassNEW.GetBinWidth(bin) )
       if(fit > data):
         err_tot = err_data_high
       else:
         err_tot = err_data_low
       fit_residual = (data - fit) / err_tot
       err_fit_residual = 1
       ##skip bin with zero entries
       chi2_VarBin_zeroes += pow( (data - fit) , 2 ) / pow( err_tot , 2 )
       if (hMassNEW.GetBinContent(bin)>0):
         NumberOfObservations_VarBin+=1
         chi2_VarBin += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 
         chi2_VarBin_notNorm += pow( (data - fit) , 2 ) 	 

       ##skip bin with less than 5 entries
       if (hMassNEW.GetBinContent(bin)*lumi*hMassNEW.GetBinWidth(bin)>=5):
         NumberOfObservations_VarBin_5entries+=1
         chi2_VarBin_5entries += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 

       hist_fit_residual_vsMass.SetBinContent(bin,fit_residual)
       hist_fit_residual_vsMass.SetBinError(bin,err_fit_residual)
  
  ndf_VarBin_5entries = NumberOfObservations_VarBin_5entries - nPar -1
  ndf_VarBin = NumberOfObservations_VarBin - nPar -1
  ndf_VarBin_withzeroes = NumberOfVarBins - nPar - 1
  print "============================" 
  print "NumberOfObservations_VarBin: %d" %  NumberOfObservations_VarBin
  print "ndf_VarBin: %d" % ndf_VarBin 
  print "ndf_VarBin with zeroes: %d" % ndf_VarBin_withzeroes 
  print "ndf_VarBin with 5entries: %d" % ndf_VarBin_5entries
  print "chi2_VarBin with zeroes: %f" % chi2_VarBin_zeroes
  print "chi2_VarBin: %f" % chi2_VarBin
  print "chi2_VarBin_5entries: %f" % chi2_VarBin_5entries
  print "chi2_VarBin_notNorm: %f" % chi2_VarBin_notNorm
  print "============================"   

  
  return [chi2_VarBin_notNorm,ndf_VarBin,chi2_VarBin,BKGfit,hist_fit_residual_vsMass]   

def FisherTest(RSS_1,RSS_2,dof_1,dof_2,N):
  RSS1 = RSS_1
  RSS2 = RSS_2
  n1 = N - dof_1 - 1
  n2 = N - dof_2 - 1
  print "Residuals function 1 = %f" % RSS_1
  print "Residuals function 2 = %f" % RSS_2
  print "DOF function 1 = %i" % n1
  print "DOF function 2 = %i" % n2
  print "DOF TOT = %i" % N
  print "n1 = %d    n2 = %d" % (n1,n2)
  F = ((RSS1-RSS2)/(n2-n1)) / (RSS2/(N-n2))
  print "Fishers F = %f" %F
  #print "F = %f" % F
  F_dist = TF1("F_distr","TMath::Sqrt( (TMath::Power([0]*x,[0]) * TMath::Power([1],[1])) / (TMath::Power([0]*x + [1],[0]+[1])) ) / (x*TMath::Beta([0]/2,[1]/2))",0,20)
  print "d1 = %d    d2 = %d" %(n2-n1,N-n2)
  F_dist.SetParameter(0, n2-n1)
  F_dist.SetParameter(1, N-n2)
  CL = 1 - F_distr.Integral(0.00000001,F)
  print "CL == %f" %CL
  return [F,CL,F_dist]
  
     
# ---------------------------------------------------------------------------------------------------------------------------
def DrawFit(hMassNEW,g,M1Bkg,hist_fit_residual_vsMass,FunctionType,nPar,fFitXmin,fFitXmax):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c = TCanvas("c","VV mass fit",W,H)
  c.GetWindowHeight()
  c.GetWindowWidth()
  c.SetLogy()
  c.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c.cd(1)
  p11_1 = c.GetPad(1)
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
  
  # addInfo.AddText("AK8CHSPF jets")
  addInfo.AddText("65 GeV < M_{P} < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.000005,fFitXmax,hMassNEW.GetMaximum()*5.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("M_{jj} [GeV]")
  vFrame.SetYTitle("#frac{d#N}{dm_{jj}}")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)

  # hMassNEW.GetXaxis().SetNdivisions(405)
  # hMassNEW.SetMarkerSize(0.9)
  # hMassNEW.SetMarkerStyle(20)
  # g.GetXaxis().SetNdivisions(405)
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
  legend.AddEntry(g, "Data","lpe")
  legend.AddEntry(M1Bkg[0], "Fit","l")
  # legend.AddEntry(M1Bkg[3], "Fit","l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c.cd(2)
  p11_2 = c.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  # vFrame2 = p11_2.DrawFrame(fFitXmin,-3.5,fFitXmax,3.5)
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

  hist_fit_residual_vsMass[0].GetXaxis().SetNdivisions(405)
  # hist_fit_residual_vsMass[0].GetXaxis().SetRangeUser(fFitXmin,fFitXmax)
  # hist_fit_residual_vsMass[0].GetYaxis().SetRangeUser(-3.5,3.5)
  # hist_fit_residual_vsMass[0].SetLineWidth(0)
  # hist_fit_residual_vsMass[0].SetFillColor(2)
  # hist_fit_residual_vsMass[0].SetLineColor(1)
  # hist_fit_residual_vsMass[0].SetLineColor( kBlack)   
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

  # time.sleep(10)
  c.SaveAs("VVdijetfit.png")

  del c
# ---------------------------------------------------------------------------------------------------------------------------    
def FitComparisons(hMassNEW,g,M1Bkg,hist_fit_residual_vsMass,FunctionType,nPar,fFitXmin,fFitXmax):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c = TCanvas("c","VV mass fit",W,H)
  c.GetWindowHeight()
  c.GetWindowWidth()
  c.SetLogy()
  c.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c.cd(1)
  p11_1 = c.GetPad(1)
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
  
  # addInfo.AddText("AK8CHSPF jets")
  addInfo.AddText("65 GeV < M_{P} < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.0000005,fFitXmax,hMassNEW.GetMaximum()*5.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("M_{jj} [GeV]")
  vFrame.SetYTitle("#frac{dN}{dm_{jj}}")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)

  # hMassNEW.GetXaxis().SetNdivisions(405)
  # hMassNEW.SetMarkerSize(0.9)
  # hMassNEW.SetMarkerStyle(20)
  # g.GetXaxis().SetNdivisions(405)
  g.SetMarkerSize(0.9)
  g.SetMarkerStyle(20)
  g.GetXaxis().SetNdivisions(405)
  g.Draw("pe0 same")
  
  colors = [kRed,kBlue,kMagenta,kBlack]
  styles = [1,4,8,2]
  i =0
  for f in M1Bkg:
    f.SetLineWidth(2)
    f.SetLineStyle(styles[i])
    f.SetLineColor(colors[i])
    f.Draw("same")
    i+=1

 
  legend = TLegend(0.56164991,0.62,0.8503575,0.80)
  legend.SetTextSize(0.038)
  legend.SetLineColor(0)
  legend.SetShadowColor(0)
  legend.SetLineStyle(1)
  legend.SetLineWidth(1)
  legend.SetFillColor(0)
  legend.SetFillStyle(0)
  legend.SetMargin(0.35)
  legend.AddEntry(g, "Data","lpe")
  legend.AddEntry(M1Bkg[0], "Default fit, 3 param.","l")
  legend.AddEntry(M1Bkg[1], "Default fit, 4 param.","l")
  legend.AddEntry(M1Bkg[2], "Alternate fit, 4 param.","l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c.cd(2)
  p11_2 = c.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  # vFrame2 = p11_2.DrawFrame(fFitXmin,-3.5,fFitXmax,3.5)
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
  
  styles = [20,22,32,25]
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
  
  time.sleep(100)
  c.SaveAs("fit-comparisons.png")

  del c
  
# ---------------------------------------------------------------------------------------------------------------------------
if __name__ == '__main__':
  
  number_of_variableWidth_bins = 103

  massBins =[1, 3, 6, 10, 16, 23, 31, 40, 50, 61, 74, 88, 103, 119, 137, 156, 176, 197, 220, 244, 270, 296, 325, 354, 386, 419, 453, 489, 526, 
           565, 606, 649, 693, 740, 788, 838, 890, 944, 1000, 1058, 1118, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687, 1770, 1856, 1945, 2037, 
           2132, 2231, 2332, 2438, 2546, 2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704, 3854, 4010, 4171, 4337, 4509, 4686, 4869, 5058, 5253, 
           5455, 5663, 5877, 6099, 6328, 6564, 6808, 7060, 7320, 7589, 7866, 8152, 8447, 8752, 9067, 9391, 9726, 10072, 10430,10798, 11179, 11571, 11977, 12395, 12827, 13272, 13732, 14000]
           
  bins1 = range (900,1300,50)
  bins2 = range (1300,3000,100)
  bins3 = range (3000,4000,200)
  bins4 = range (4000,6000,300)
  bins = []
  bins = bins1
  bins += bins2
  bins += bins3
  bins += bins4
  # bins = range (900,6000,40)
  runArray = array('d',bins)
  
  performFit("DATA_noTau21.root",
             "DijetMassHighPuriVV", len(bins)-1, bins, 1000, 4500, "M_{jj}>1000 GeV",
             "VV-3ParFit_13TeV_225pb.pdf", 0, 8.62449e-04, 1.02685e+01, 5.00842e+00,  0.00000e+00)